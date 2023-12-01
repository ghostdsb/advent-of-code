#[allow(unused)]
pub mod sol {

    use std::{fs, vec};

    pub fn aoc(day: u8, part: u8) -> String {
        let input_path = format!("./input/day{}.txt", day);
        match (fs::read_to_string(&input_path), part) {
            (Ok(content), 1) => part1(content),
            (Ok(content), 2) => part2(content),
            (_, _) => {
                println!("something wrong");
                String::new()
            }
        }
    }

    fn part1(content: String) -> String {
        let mut cur_ans = 0;
        let steps = content
            .split(|c| c == '\n')
            .map(|d| {
                let line: Vec<&str> = d.split(' ').collect();
                (
                    line[1].parse::<i64>().unwrap(),
                    line[3].parse::<i64>().unwrap(),
                    line[5].parse::<i64>().unwrap(),
                )
            })
            .collect::<Vec<(i64, i64, i64)>>();
        let mut stack = initial_position();
        for (count, from, to) in steps {
            for i in 0..count as usize {
                let x = stack[from as usize - 1].pop().unwrap();
                stack[to as usize - 1].push(x);
            }
        }
        let ans = stack.iter().fold(String::new(), |mut l, c| {
            l.push(c[c.len() - 1]);
            l
        });
        ans.to_string()
    }

    fn part2(content: String) -> String {
        let mut cur_ans = 0;
        let steps = content
            .split(|c| c == '\n')
            .map(|d| {
                let line: Vec<&str> = d.split(' ').collect();
                (
                    line[1].parse::<i64>().unwrap(),
                    line[3].parse::<i64>().unwrap(),
                    line[5].parse::<i64>().unwrap(),
                )
            })
            .collect::<Vec<(i64, i64, i64)>>();
        let mut stack = initial_position();
        for (count, from, to) in steps {
            let mut y = vec![];
            for i in 0..count as usize {
                let x = stack[from as usize - 1].pop().unwrap();
                y.push(x);
            }
            y.reverse();
            for c in y {
                stack[to as usize - 1].push(c);
            }
        }
        let ans = stack.iter().fold(String::new(), |mut l, c| {
            l.push(c[c.len() - 1]);
            l
        });
        ans.to_string()
    }

    fn initial_position() -> Vec<Vec<char>> {
        let stack = vec![
            "DHNQTWVB", "DWB", "TSQWJC", "FJRNZTP", "GPVJMST", "BWFTN", "BLDQFHVN", "HPFR",
            "ZSMBLNPH",
        ]
        .iter()
        .fold(vec![], |mut p, s| {
            p.push(s.chars().fold(vec![], |mut acc, x| {
                acc.push(x);
                acc
            }));
            p
        });
        stack
    }
}
