use std::collections::{HashMap, HashSet, VecDeque};

pub fn process(input: &str) -> String {
    let mut hm : HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut edges: HashSet<(&str, &str)> = HashSet::new();
    let mut ans = 0;
    input.lines().for_each(|line|{
        let x = line.split(&[':', ' ']).filter(|c| c!= &"").collect::<Vec<&str>>();
        for i in 1..x.len(){
            hm.entry(x[0]).and_modify(|set| {set.insert(x[i]);}).or_insert({
                let mut y = HashSet::new(); 
                y.insert(x[i]);
                y
            });
            hm.entry(x[i]).and_modify(|set| {set.insert(x[0]);}).or_insert({
                let mut y = HashSet::new(); 
                y.insert(x[0]);
                y
            });
            let edge1 = (x[0], x[i]); 
            let edge2 = (x[i], x[0]); 
            if !edges.contains(&edge1) && !edges.contains(&edge2){
                edges.insert(edge1);
            }
        }
    });

    let mut found = false;

    for k in hm.keys(){
        let x:Vec<(u64, (&str, &str))> = bfs(&hm, k);
        let y = group(&x);
        let y = y.iter().filter(|node_group|{
            node_group.len() == 3
        }).map(|node_group| {
            node_group.iter().fold(vec![], |mut acc, node|{
                acc.push(node.1);
                acc
            })
        }).collect::<Vec<_>>();

        for node in y.iter(){
            let mut updated_hm = hm.clone();
            let a = node[0].0;
            let b = node[0].1;
            let c = node[1].0;
            let d = node[1].1;
            let e = node[2].0;
            let f = node[2].1;
            // dbg!(a,b,c,d,e,f);
            updated_hm.entry(a).and_modify(|s| {s.remove(b);});
            updated_hm.entry(b).and_modify(|s| {s.remove(a);});
            updated_hm.entry(c).and_modify(|s| {s.remove(d);});
            updated_hm.entry(d).and_modify(|s| {s.remove(c);});
            updated_hm.entry(e).and_modify(|s| {s.remove(f);});
            updated_hm.entry(f).and_modify(|s| {s.remove(e);});
    
            if good_cut(&updated_hm, a, b) && good_cut(&updated_hm, c, d) && good_cut(&updated_hm, e, f){
                // dbg!(a,b,c,d,e,f);
                // visualize(&updated_hm);
                let l1 = graph_length(&updated_hm, a); 
                let l2 = graph_length(&updated_hm, b); 
                dbg!(l1*l2);
                ans = l1 * l2;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    ans.to_string()
}

// 5_339_372_100

fn group<'a>(input: &Vec<(u64, (&'a str, &'a str))>) ->  Vec<Vec<(u64, (&'a str, &'a str))>> {

    let mut result: Vec<Vec<(u64, (&str, &str))>> = Vec::new();
    let mut current_chunk: Vec<(u64, (&str, &str))> = Vec::new();

    for (depth, node) in input {
        if current_chunk.is_empty() || current_chunk[0].0 == *depth {
            current_chunk.push((*depth, *node));
        } else {
            result.push(current_chunk.clone());
            current_chunk = vec![(*depth, *node)];
        }
    }
    if !current_chunk.is_empty() {
        result.push(current_chunk);
    }

    result
}

fn good_cut(hm : &HashMap<&str, HashSet<&str>>, start: &str, end: &str) -> bool{
    let mut stack: Vec<&str> = vec![start];
    let mut visited: HashSet<&str> = HashSet::new();
    while !stack.is_empty(){
        let top = stack.pop().unwrap();
        if top == end{
            return false;
        }
        visited.insert(top);
        for nbr in hm.get(top).unwrap().iter().collect::<Vec<&&str>>().iter(){
            if **nbr == end{
                return false;
            }
            if !visited.contains(*nbr){
                visited.insert(nbr);
                stack.push(nbr);
            }
        }
    }
    true
}

fn bfs<'a>(hm : &HashMap<&str, HashSet<&'a str>>, start: &'a str) -> Vec<(u64, (&'a str, &'a str))>{
    let mut queue: VecDeque<(u64, &str)> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();
    queue.push_back((0,start));
    let mut path:Vec<(u64, (&str, &str))> = Vec::new();
    path.push((0,(start, start)));
    while !queue.is_empty(){
        let (d, top) = queue.pop_front().unwrap();
        visited.insert(top);
        for nbr in hm.get(top).unwrap().iter().collect::<Vec<&&str>>().iter(){
            if !visited.contains(*nbr){
                visited.insert(nbr);
                queue.push_back((d+1, nbr));
                path.push((d+1, (top, nbr)));
            }
        }
    }
    path
}

fn graph_length(hm : &HashMap<&str, HashSet<&str>>, start: &str) -> u64{
    let mut stack: Vec<&str> = vec![start];
    let mut visited: HashSet<&str> = HashSet::new();
    let mut count = 0;
    while !stack.is_empty(){
        let top = stack.pop().unwrap();
        count += 1;
        visited.insert(top);
        for nbr in hm.get(top).unwrap().iter().collect::<Vec<&&str>>().iter(){
            if !visited.contains(*nbr){
                visited.insert(nbr);
                stack.push(nbr);
            }
        }
    }
    count
}

use std::io::Write;
use std::fs::OpenOptions;
fn visualize(hm : &HashMap<&str, HashSet<&str>>) {
    let mut s: HashSet<(&str, &str)> = HashSet::new();
    let mut d = String::new();
    for (n, set) in hm.iter(){
        for c in set.iter(){
            if !s.contains(&(n,c)) && !s.contains(&(c,n)){
                s.insert((n,c));
                s.insert((c,n));
                let l1 = format!("{}->{}[arrowhead=none]", n, c);
                // let l2 = format!("{}->{}[arrowhead=none]", c, n);
                let data = format!("{}\n", l1);
                d.push_str(&data);
            }
        }
    }

    let graph = format!("digraph G{{ \n{} \n}}", d);

    let mut f = OpenOptions::new()
        .append(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open("./out.dot")
        .expect("Unable to open file");
    f.write_all(graph.as_bytes()).expect("Unable to write data");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "54");
    }
}

