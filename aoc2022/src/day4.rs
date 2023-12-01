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
        let sections = content
            .split(|c| c == '\n')
            .map(|d| {
                let a = d.split(',').collect::<Vec<&str>>();
                let b = a[0]
                    .split('-')
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                let c = a[1]
                    .split('-')
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                (b, c)
            })
            .collect::<Vec<(Vec<i64>, Vec<i64>)>>();
        for (sec1, sec2) in sections.iter() {
            if sec1[0] >= sec2[0] && sec1[1] <= sec2[1] || sec2[0] >= sec1[0] && sec2[1] <= sec1[1]
            {
                ans += 1;
            }
        }
        // println!("{:?}", sections);
        ans.to_string()
    }

    fn part2(content: String) -> String {
        let mut ans = 0;
        let mut cur_ans = 0;
        let sections = content
            .split(|c| c == '\n')
            .map(|d| {
                let a = d.split(',').collect::<Vec<&str>>();
                let b = a[0]
                    .split('-')
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                let c = a[1]
                    .split('-')
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                if c[0] > b[0] {
                    (b, c)
                } else {
                    (c, b)
                }
            })
            .collect::<Vec<(Vec<i64>, Vec<i64>)>>();
        for (i, (sec1, sec2)) in sections.iter().enumerate() {
            if (sec1[1] >= sec2[0]) {
                ans += 1;
            }
        }
        // println!("{:?}", sections);
        ans.to_string()
    }
}
