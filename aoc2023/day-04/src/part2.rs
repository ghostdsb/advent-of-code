use std::collections::BTreeSet;
pub fn process(input: &str) -> String {
    let mut matches = vec![0; input.lines().count() + 1];
    input.lines().enumerate().for_each(|(index, line)| {
        matches[index + 1] += 1;
        let info = line.split(&[':', '|'][..]).collect::<Vec<&str>>();
        let mut set: BTreeSet<u32> = BTreeSet::new();
        let _setter = info[2].trim().split_whitespace().for_each(|num| {
            let n = num.parse::<u32>().unwrap();
            set.insert(n);
        });
        let mut count = 0;
        let _match_counter = info[1].trim().split_whitespace().for_each(|num| {
            let n = num.parse::<u32>().unwrap();
            if set.contains(&n) {
                count += 1;
            }
        });
        (1..=count).for_each(|c| {
            matches[index + 1 + c as usize] += matches[index + 1];
        });
    });
    // dbg!(&m);
    matches.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "30");
    }
}
