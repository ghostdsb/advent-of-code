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
        let calories = content
            .split(|c| c == '\n')
            .map(|d| d.parse::<i64>().unwrap_or(-1))
            .collect::<Vec<i64>>();

        for i in 1..calories.len() {
            if calories[i] != -1 {
                cur_ans += calories[i];
            } else {
                if cur_ans > ans {
                    ans = cur_ans;
                }
                cur_ans = 0;
            }
        }
        ans.to_string()
    }

    fn part2(content: String) -> String {
        let mut ans = vec![];
        let mut cur_ans = 0;
        let calories = content
            .split(|c| c == '\n')
            .map(|d| d.parse::<i64>().unwrap_or(-1))
            .collect::<Vec<i64>>();

        for i in 1..calories.len() {
            if calories[i] != -1 {
                cur_ans += calories[i];
            } else {
                ans.push(cur_ans);
                cur_ans = 0;
            }
        }
        ans.sort();
        ans.reverse();

        let ans: i64 = ans[0..3].into_iter().sum();

        ans.to_string()
    }
}
