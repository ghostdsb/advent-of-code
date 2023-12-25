use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn new(x: u64, m: u64, a: u64, s: u64) -> Self {
        Self { x, m, a, s }
    }

    fn apply(&mut self, apply_str: &str) {
        let statement = apply_str.split('=').collect::<Vec<&str>>();
        match statement[0] {
            "x" => self.x = statement[1].parse::<u64>().unwrap(),
            "m" => self.m = statement[1].parse::<u64>().unwrap(),
            "a" => self.a = statement[1].parse::<u64>().unwrap(),
            "s" => self.s = statement[1].parse::<u64>().unwrap(),
            _ => (),
        }
    }

    fn rating_sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    operations: Vec<Operation<'a>>,
    destination: &'a str,
}

impl<'a> Workflow<'a> {
    fn new(operations: Vec<Operation<'a>>, destination: &'a str) -> Self {
        Self {
            operations,
            destination,
        }
    }

    fn apply(&self, part: Part) -> &str {
        for operation in self.operations.iter() {
            let category_value = match operation.category {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => 0,
            };
            if category_value.cmp(&operation.value) == operation.comparison {
                return operation.destination;
            } else {
                continue;
            }
        }
        self.destination
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

        let workflow_operations = x[0..x.len() - 1]
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
                wfo.push(wfu);
                wfo
            });
        workflow_map.insert(
            workflow_name,
            Workflow::new(workflow_operations, workflow_destination),
        );
    });

    // dbg!(workflow_map);

    let parts_str: &str = input.split("\n\n").collect::<Vec<&str>>()[1];
    let parts_vec: Vec<Part> = parts_str.lines().fold(Vec::new(), |mut parts, line| {
        let mut part = Part::new(0, 0, 0, 0);
        let x: Vec<&str> = line.split(&['{', '}'][..]).filter(|&x| x != "").collect();
        let x: Vec<&str> = x[0].split(&[','][..]).collect();
        x.iter().for_each(|s| part.apply(s));
        parts.push(part);
        parts
    });
    // dbg!(&parts_vec);

    // let mut accepted = 0;
    let x = parts_vec.iter().fold(0, |mut acc, part| {
        let mut d = "in";
        while d != "A" && d != "R" {
            d = workflow_map.get(&d).unwrap().apply(*part);
        }
        if d == "A" {
            acc += part.rating_sum();
        }
        acc
    });

    x.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "19114");
    }
}
