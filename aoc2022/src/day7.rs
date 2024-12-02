#[allow(unused)]
pub mod sol {
    #![feature(hash_set_entry)]
    use std::{fs, collections::BTreeMap};
    
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

    fn part1(input: String) -> String {
        let mut ans = 0;
        let mut stack: Vec<&str> = vec![];
        let mut files: Vec<String> = vec![];
        let mut hm: BTreeMap<String, u64> = BTreeMap::new();
        let mut tree: BTreeMap<String, Vec<String>> = BTreeMap::new();
        input.lines().for_each(|line| {
            if line.contains("$ cd") {
                let x = line.split_whitespace().collect::<Vec<&str>>();
                let dir = x.last().unwrap();
                if dir == &".." {
                    stack.pop();
                } else if dir != &"/" {
                    stack.push(dir);
                }
            } else if !line.contains("$ ls") {
                let mut location = stack.iter().fold(String::new(), |acc, l| acc + "/" + l);
                let x = line.split_whitespace().collect::<Vec<&str>>();
                let file = "/".to_owned() + x[1];
                let name = location.clone() + &file;
                if x[0] != "dir"{
                    hm.insert(name.clone(), x[0].parse::<u64>().unwrap());
                }
                tree.entry(location.to_owned()).and_modify(|c| {
                    c.push(name.clone())
                }).or_insert(vec![name]);
            }
        });
        tree.keys().into_iter().for_each(|d|{
            let size = get_size(&tree,&hm, d.to_string());
            if size <= 100000{
                ans += size;
            }
        } );
        ans.to_string()
    }

    fn get_size(tree: &BTreeMap<String, Vec<String>>, file_size: &BTreeMap<String, u64>, file: String) -> u64{
        let mut ans = 0;
        let mut stack: Vec<String> = vec![];
        stack.push(file);
        while !stack.is_empty(){
            // println!("{}",stack.len());
            let top = stack.pop().unwrap();
            if file_size.contains_key(&top){
                ans += file_size.get(&top).unwrap();
            }else if tree.contains_key(&top){
                tree.get(&top).unwrap().iter().for_each(|c| stack.push(c.to_string()));
            }
        }
        ans
    }

    fn part2(input: String) -> String {
        let mut stack: Vec<&str> = vec![];
        let mut files: Vec<String> = vec![];
        let mut hm: BTreeMap<String, u64> = BTreeMap::new();
        let mut tree: BTreeMap<String, Vec<String>> = BTreeMap::new();
        input.lines().for_each(|line| {
            if line.contains("$ cd") {
                let x = line.split_whitespace().collect::<Vec<&str>>();
                let dir = x.last().unwrap();
                if dir == &".." {
                    stack.pop();
                } else if dir != &"/" {
                    stack.push(dir);
                }
            } else if !line.contains("$ ls") {
                let mut location = stack.iter().fold(String::new(), |acc, l| acc + "/" + l);
                let x = line.split_whitespace().collect::<Vec<&str>>();
                let file = "/".to_owned() + x[1];
                let name = location.clone() + &file;
                if x[0] != "dir"{
                    hm.insert(name.clone(), x[0].parse::<u64>().unwrap());
                }
                tree.entry(location.to_owned()).and_modify(|c| {
                    c.push(name.clone())
                }).or_insert(vec![name]);
            }
        });
        let total_occupied = get_size(&tree, &hm, "".to_string());
        dbg!(total_occupied);
        let required = 30000000 - (70000000 - total_occupied);
        let ans = tree.keys().into_iter().fold(u64::MAX, |ans,  d|{
            let s = get_size(&tree,&hm, d.to_string());
            if s < required{
                ans
            }else{
                ans.min(s)
            }
        });
        ans.to_string()
    }
}
