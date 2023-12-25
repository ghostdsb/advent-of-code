use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Block{
    pub end1: (u32, u32, u32),
    pub end2: (u32, u32, u32),
    index: usize,
}

impl Block{
    fn new(index: usize, end1: (u32, u32, u32), end2: (u32, u32, u32)) -> Self{
        Self{
            index, end1, end2
        }
    }

    fn xy_overlap(&self, other: &Block) -> bool{
        self.end1.0.max(other.end1.0) <= self.end2.0.min(other.end2.0) &&   
        self.end1.1.max(other.end1.1) <= self.end2.1.min(other.end2.1)   
        // true
    }
}

pub fn create_tower(input: &str) -> Vec<Block>{
    let zenga: Vec<Block> = input.lines().enumerate().fold(vec![], |mut acc, (i, line)|{
        let x = line.split('~').collect::<Vec<&str>>();
        let end1 = x[0].split(',').map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let end2 = x[1].split(',').map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let block = Block::new(i, (end1[0], end1[1], end1[2]), (end2[0], end2[1], end2[2]));
        acc.push(block);
        acc
    });
    zenga
}

pub fn process(input: &str) -> String {
    let mut zenga: Vec<Block> = create_tower(input);
    zenga.sort_by(|b1, b2| (b1.end1.2).cmp(&b2.end1.2));
    for i in 1..zenga.len(){
        let mut max_z = 1;
        for j in 0..i{
            if zenga[i].xy_overlap(&zenga[j]){
                println!("{}-{}",i,j);
                max_z = max_z.max(zenga[j].end2.2 + 1);
            }
        }
        zenga[i].end2.2 -= zenga[i].end1.2 - max_z;
        zenga[i].end1.2 = max_z
    }
    zenga.sort_by(|b1, b2| (b1.end1.2).cmp(&b2.end1.2));
    let mut support: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut supportz: HashMap<usize, HashSet<usize>> = HashMap::new();
    for i in 0..zenga.len()-1{
        for j in i+1..zenga.len(){
            if zenga[i].xy_overlap(&zenga[j]) && zenga[i].end2.2 == zenga[j].end1.2 -1 {
                support
                .entry(zenga[i].index)
                .and_modify(|s| {s.insert(zenga[j].index);})
                .or_insert({
                    let mut s = HashSet::new();
                    s.insert(zenga[j].index);
                    s
                });
                
                supportz
                .entry(zenga[j].index)
                .and_modify(|s| {s.insert(zenga[i].index);})
                .or_insert({
                    let mut s = HashSet::new();
                    s.insert(zenga[i].index);
                    s
                });
            }
        }
    }
    let mut ans = 0;
    for i in 0..zenga.len(){
        if let Some(x) = support.get(&i){
            let y = x.iter().map(|a|{
                if let Some(z) = supportz.get(a){
                    z.len() as u32
                }else{
                    0
                }
            }).collect::<Vec<u32>>().iter().all(|c| c >= &2);
            if y{
                ans += 1;
            }
            dbg!(x,y);
        }else{
            ans += 1;
        };
    }
    // dbg!(support);
    // dbg!(supportz);
    // dbg!(ans);
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "5");
    }
}
