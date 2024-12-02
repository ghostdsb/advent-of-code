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
        let mut lookup = [-1; 26];
        let mut start = 0;
        let mut length = 0;

        for (i, c) in content.chars().enumerate() {
            if lookup[char_index(c)] == -1 || lookup[char_index(c)] < start as i32 {
                lookup[char_index(c)] = i as i32;
                length += 1;
            } else {
                start = lookup[char_index(c)] as usize + 1;
                lookup[char_index(c)] = i as i32;
                length = i - start;
            }
            if length == 4 {
                break;
            }
        }
        ans = start + 4;
        ans.to_string()
    }

    fn char_index(c: char) -> usize {
        (c as u8 - 'a' as u8) as usize
    }

    fn part2(content: String) -> String {
        let mut ans = 0;
        let mut lookup = [-1; 26];
        let mut start = 0;
        let mut length = 0;

        for (i, c) in content.chars().enumerate() {
            if lookup[char_index(c)] == -1 || lookup[char_index(c)] < start as i32 {
                lookup[char_index(c)] = i as i32;
                length += 1;
            } else {
                start = lookup[char_index(c)] as usize + 1;
                lookup[char_index(c)] = i as i32;
                length = i - start;
            }
            if length == 14 {
                break;
            }
        }
        ans = start + 14;
        ans.to_string()
    }
}
