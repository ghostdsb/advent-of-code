use num::Integer;
use std::collections::HashMap;

pub fn process(input: &str) -> String {
    let mut hm: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut path = "";
    input
        .lines()
        .filter(|&l| l != "")
        .enumerate()
        .for_each(|(index, l)| {
            if index == 0 {
                path = l;
            } else {
                let details = l
                    .split(&['=', ' ', '(', ',', ')'][..])
                    .filter(|&c| c != "")
                    .collect::<Vec<&str>>();
                hm.insert(details[0], (details[1], details[2]));
            }
        });
    let distance = hm
        .keys()
        .filter(|&&k| k.ends_with('A'))
        .map(|k| distance_to_end(&hm, k, path))
        .collect::<Vec<u64>>()
        .iter()
        .fold(1u64, |p, a| p.lcm(&a));

    distance.to_string()
}

fn distance_to_end(hm: &HashMap<&str, (&str, &str)>, current: &str, path: &str) -> u64 {
    let mut distance = 0;
    let mut path = path.chars().cycle();
    let mut current = current;
    while !current.ends_with('Z') {
        distance += 1;
        let (left, right) = hm.get(current).unwrap();
        let dir = path.next().unwrap();
        if dir == 'R' {
            current = right;
        } else {
            current = left;
        }
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "6");
    }
}
