use std::collections::BTreeSet;
pub fn process(input: &str) -> String {
    let mut ans = 0;
    input.lines().for_each(|line| {
        let info = line.split(&[':', '|'][..]).collect::<Vec<&str>>();
        let mut set: BTreeSet<u32> = BTreeSet::new();
        let _setter = info[2].trim().split_whitespace().for_each(|num| {
            let n = num.parse::<u32>().unwrap();
            set.insert(n);
        });
        let mut count = 0;
        let mut found = false;
        let _match_counter = info[1].trim().split_whitespace().for_each(|num| {
            let n = num.parse::<u32>().unwrap();
            if set.contains(&n) {
                count += 1;
                found = true
            }
        });
        if found {
            ans += 2u32.pow(count - 1);
        }
    });
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "13");
    }
}
