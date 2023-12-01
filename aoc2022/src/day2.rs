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
        let mut ans = 0;
        let mut cur_ans = 0;
        let round = content
            .split(|c| c == '\n')
            .map(|d| d.parse::<String>().unwrap())
            .collect::<Vec<String>>();

        // println!("{:?}", round);
        for m in round {
            match m[..].split(" ").into_iter().collect::<Vec<&str>>()[..] {
                ["A", "Y"] => ans += 2 + 6,
                ["B", "Z"] => ans += 3 + 6,
                ["C", "X"] => ans += 1 + 6,
                ["A", "X"] => ans += 1 + 3,
                ["B", "Y"] => ans += 2 + 3,
                ["C", "Z"] => ans += 3 + 3,
                ["A", "Z"] => ans += 3 + 0,
                ["B", "X"] => ans += 1 + 0,
                ["C", "Y"] => ans += 2 + 0,
                _ => {
                    ans;
                }
            };
        }
        ans.to_string()
    }

    fn part2(content: String) -> String {
        let mut ans = 0;
        let mut cur_ans = 0;
        let round = content
            .split(|c| c == '\n')
            .map(|d| d.parse::<String>().unwrap())
            .collect::<Vec<String>>();

        // println!("{:?}", round);
        for m in round {
            match m[..].split(" ").into_iter().collect::<Vec<&str>>()[..] {
                ["A", "X"] => ans += 3 + 0,
                ["B", "X"] => ans += 1 + 0,
                ["C", "X"] => ans += 2 + 0,
                ["A", "Y"] => ans += 1 + 3,
                ["B", "Y"] => ans += 2 + 3,
                ["C", "Y"] => ans += 3 + 3,
                ["A", "Z"] => ans += 2 + 6,
                ["B", "Z"] => ans += 3 + 6,
                ["C", "Z"] => ans += 1 + 6,
                _ => {
                    ans;
                }
            };
        }
        ans.to_string()
    }
}
