#[allow(unused)]
pub mod sol {
    #![feature(hash_set_entry)]
    use std::{fs, vec};
    use std::collections::HashSet;

    // #[derive(Debug, Eq, Hash, Clone)]
    // struct Directory{
    //     name: String,
    //     dirs: Vec<Directory>,
    //     files: Vec<String>
    // }

    // impl PartialEq for Directory{
    //     fn eq(&self, other: &Directory) -> bool{
    //         self.name == other.name
    //     }
    // }

    // #[derive(Debug)]
    // struct File{
    //     name: String,
    //     size: u64
    // }

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
        ans.to_string()
    }

    fn part2(content: String) -> String {
        unimplemented!()
    }
}
