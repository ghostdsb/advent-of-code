use std::collections::VecDeque;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Copy)]
struct Ranger {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl Ranger {
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn count(&self) -> u64 {
        let count_x = self.x.1 - self.x.0 + 1;
        let count_m = self.m.1 - self.m.0 + 1;
        let count_a = self.a.1 - self.a.0 + 1;
        let count_s = self.s.1 - self.s.0 + 1;

        count_x * count_m * count_a * count_s
    }

    fn limit(&self, operation: &Operation) -> (Self, Self) {
        let (included_range, excluded_range) = match (operation.category, operation.comparison) {
            ('x', Ordering::Less) => ((self.x.0, operation.value - 1), (operation.value, self.x.1)),
            ('x', Ordering::Greater) => {
                ((operation.value + 1, self.x.1), (self.x.0, operation.value))
            }
            ('m', Ordering::Less) => ((self.m.0, operation.value - 1), (operation.value, self.m.1)),
            ('m', Ordering::Greater) => {
                ((operation.value + 1, self.m.1), (self.m.0, operation.value))
            }
            ('a', Ordering::Less) => ((self.a.0, operation.value - 1), (operation.value, self.a.1)),
            ('a', Ordering::Greater) => {
                ((operation.value + 1, self.a.1), (self.a.0, operation.value))
            }
            ('s', Ordering::Less) => ((self.s.0, operation.value - 1), (operation.value, self.s.1)),
            ('s', Ordering::Greater) => {
                ((operation.value + 1, self.s.1), (self.s.0, operation.value))
            }
            _ => panic!(),
        };
        let mut included = self.clone();
        let mut excluded = self.clone();
        included.set(included_range, operation.category);
        excluded.set(excluded_range, operation.category);
        (included, excluded)
    }

    fn set(&mut self, range: (u64, u64), category: char) {
        match category {
            'x' => self.x = range,
            'm' => self.m = range,
            'a' => self.a = range,
            's' => self.s = range,
            _ => (),
        };
    }
    fn set_range(&mut self, ranger: Ranger) {
        self.x = ranger.x;
        self.m = ranger.m;
        self.a = ranger.a;
        self.s = ranger.s;
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    operations: Vec<OperationType<'a>>,
}

impl<'a> Workflow<'a> {
    fn new(operations: Vec<OperationType<'a>>) -> Self {
        Self {
            operations,
        }
    }
}

#[derive(Debug, Clone)]
enum OperationType<'a> {
    Transit(Operation<'a>),
    Terminal(&'a str),
}

impl<'a> OperationType<'a> {
    fn apply(&self, ranger: Ranger) -> (Ranger, Ranger) {
        let x = match &self {
            OperationType::Transit(operation) => {
                let r = ranger.clone();
                r.limit(operation)
            }
            OperationType::Terminal(_) => {
                let mut r = ranger.clone();
                r.set_range(ranger);
                (r, r)
            }
        };
        x
    }

    fn name(&self) -> &str {
        match &self {
            OperationType::Transit(operation) => operation.destination,
            OperationType::Terminal(destination) => &destination,
        }
    }
}

#[derive(Debug, Clone)]
struct Operation<'a> {
    category: char,
    comparison: Ordering,
    value: u64,
    destination: &'a str,
}

impl<'a> Operation<'a> {
    fn new(category: char, comparision: char, value: u64, destination: &'a str) -> Self {
        let comp = match comparision {
            '>' => Ordering::Greater,
            '<' => Ordering::Less,
            _ => Ordering::Equal,
        };
        Self {
            category,
            comparison: comp,
            value,
            destination,
        }
    }
}

pub fn process(input: &str) -> String {
    let mut workflow_map: HashMap<&str, Workflow> = HashMap::new();
    let workflow_str: &str = input.split("\n\n").collect::<Vec<&str>>()[0];
    workflow_str.lines().for_each(|l| {
        let line: Vec<&str> = l.split(&['{', '}']).filter(|&y| y != "").collect();
        let workflow_name = line[0];

        let x = line[1].split(',').collect::<Vec<&str>>();
        let workflow_destination = x.last().unwrap();

        let mut workflow_operations: Vec<OperationType> = x[0..x.len() - 1]
            .iter()
            .map(|a| a.to_owned())
            .collect::<Vec<&str>>()
            .iter()
            .fold(Vec::new(), |mut wfo, s| {
                let line: Vec<&str> = s.split(&[':']).collect();
                let unit_destination = line.last().unwrap();
                let unit_category = s.chars().nth(0).unwrap();
                let unit_comparision = s.chars().nth(1).unwrap();
                let unit_value = s
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<Vec<char>>()
                    .iter()
                    .fold(0, |d, &c| d * 10 + (c as u8 - b'0') as u64);
                let wfu = Operation::new(
                    unit_category,
                    unit_comparision,
                    unit_value,
                    unit_destination,
                );
                wfo.push(OperationType::Transit(wfu));
                wfo
            });
        workflow_operations.push(OperationType::Terminal(&workflow_destination));
        workflow_map.insert(workflow_name, Workflow::new(workflow_operations));
    });

    let mut stack: VecDeque<(Ranger, &str)> = VecDeque::new();

    stack.push_back((Ranger::new(), "in"));
    let mut accepted: Vec<Ranger> = vec![];
    while !stack.is_empty() {
        let (mut ranger, current_workflow_name) = stack.pop_front().unwrap();
        if let Some(workflow) = &workflow_map.get(current_workflow_name) {
            for oper in workflow.operations.iter() {
                let x = oper.apply(ranger);
                let name = oper.name();
                stack.push_back((x.0, name));
                ranger = x.1;
                if name == "A" {
                    accepted.push(x.0);
                }
            }
        }
    }
    let ans = accepted.iter().fold(0, |acc, range| acc + range.count());

    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "167409079868000");
    }
}
