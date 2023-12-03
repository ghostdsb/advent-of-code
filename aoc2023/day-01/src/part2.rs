pub fn process(input: &str) -> String {
    let x: u32 = input
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .fold(0, |mut acc, line| {
            let line_value = parse_line(*line);
            acc += line_value;
            acc
        });
    x.to_string()
}

fn parse_line(line: &str) -> u32 {
    let nums = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let s = String::from(line);
    let mut digits: Vec<u8> = vec![];
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap().is_ascii_digit() {
            digits.push(s.chars().nth(i).unwrap().to_digit(10).unwrap() as u8);
        } else {
            for (digit, word) in nums.iter().enumerate() {
                if s[i..].starts_with(word) {
                    digits.push(digit as u8);
                }
            }
        }
        // dbg!(&digits);
    }

    digits[0] as u32 * 10 + digits[digits.len() - 1] as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "281");
    }

    #[test]
    fn parse_line_test() {
        let cases = [
            ("one", 11),
            ("two", 22),
            ("three", 33),
            ("four", 44),
            ("five", 55),
            ("six", 66),
            ("seven", 77),
            ("eight", 88),
            ("nine", 99),
            ("eighthree", 83),
            ("sevenine", 79),
            ("2sevenine", 29),
            ("twone", 21),
            ("eightwo", 82),
            ("sev8en", 88),
            ("sev8enine", 89),
            ("v43wonect", 41),
            ("nineightwoneighthreeight", 98),
        ];
        cases
            .iter()
            .for_each(|(word, num)| assert_eq!(parse_line(word), *num));
    }
}
