#[allow(unused)]
pub mod sol {

    use std::collections::{HashMap, HashSet};
    use std::hash::Hash;
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
        let rucksack = content
            .split(|c| c == '\n')
            .map(|d| d.parse::<String>().unwrap())
            .collect::<Vec<String>>();

        for item in rucksack {
            let len = item.len();
            let item1 = &item[0..len / 2];
            let item2 = &item[len / 2..len];
            ans += get_common(item1, item2);
        }
        ans.to_string()
    }

    fn get_common(str1: &str, str2: &str) -> u16 {
        let mut hm = HashMap::new();
        for c in str1.chars() {
            let counter = hm.entry(c).or_insert(1);
            *counter += 1;
        }
        let mut ans = 0;
        for c in str2.chars() {
            match hm.get(&c) {
                Some(_) => {
                    if c.is_lowercase() {
                        ans = c as u16 - 'a' as u16 + 1;
                    } else {
                        ans = c as u16 - 'A' as u16 + 1 + 26;
                    }
                    break;
                }
                None => (),
            };
        }
        ans
    }

    fn part2(content: String) -> String {
        let mut ans = 0;
        let mut cur_ans = 0;
        let rucksack = content
            .split(|c| c == '\n')
            .map(|d| d.parse::<String>().unwrap())
            .collect::<Vec<String>>();

        for chunk in rucksack.chunks(3).into_iter() {
            let common = get_common_in_vec(chunk.to_vec().iter().map(|c| c.as_str()).collect());
            ans += common;
        }
        ans.to_string()
    }

    fn get_common_in_vec(strs: Vec<&str>) -> u16 {
        let mut ans = 0;
        let mut map = HashMap::new();
        for string in strs {
            let mut set = HashSet::new();
            for c in string.chars() {
                set.insert(c);
            }
            for val in set {
                let counter = map.entry(val).or_insert(0);
                *counter += 1;
            }
        }
        for (c, v) in map {
            if v == 3 {
                ans += if c.is_lowercase() {
                    c as u16 - 'a' as u16 + 1
                } else {
                    c as u16 - 'A' as u16 + 1 + 26
                }
            }
        }
        ans
    }
}
