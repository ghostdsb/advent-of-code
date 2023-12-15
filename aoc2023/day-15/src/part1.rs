pub fn process(input: &str) -> String {
    let ans = input.split(',').into_iter().fold(0, |mut acc_top, seq| {
        let x = get_hash(seq);
        acc_top += x;
        acc_top
    });
    ans.to_string()
}

fn get_hash(sequence: &str) -> u64 {
    sequence.chars().fold(0, |mut acc, c| {
        acc = ((acc + c as u64) * 17) % 256;
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "1320");
    }
}
