pub fn process(input: &str) -> String {
    let x: u32 = input
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .fold(0, |mut acc, line| {
            let line_value = parse_line(line);
            acc += line_value;
            acc
        });
    x.to_string()
}

fn parse_line(line: &&str) -> u32 {
    let nums = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];
    let mut min: usize = usize::MAX;
    let mut max = usize::MIN;

    let mut min_value = u8::MAX;
    let mut max_value = u8::MIN;
    nums.iter().for_each(|num| {
        if let Some(index) = line.find(*num) {
            if index <= min {
                min = index;
                min_value = get_number(&&num);
            }
        };
        ();
        if let Some(index) = line.rfind(*num) {
            if index >= max {
                max = index;
                max_value = get_number(&&num);
            }
        };
        ()
    });
    let _ = line
        .chars()
        .enumerate()
        .filter(|(_i, x)| x.is_ascii_digit())
        .for_each(|(i, x)| {
            if i <= min {
                min = i;
                min_value = x.to_digit(10).unwrap() as u8;
            }
            if i >= max {
                max = i;
                max_value = x.to_digit(10).unwrap() as u8;
            }
        });
    (min_value * 10) as u32 + max_value as u32
}

fn get_number(word: &str) -> u8{
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
    _ => 0
    }
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
            ("eighthree", 83),
            ("sevenine", 79),
            ("2sevenine", 29),
            ("twone", 21),
            ("eightwo", 82),
            ("one", 11),
            ("two", 22),
            ("three", 33),
            ("four", 44),
            ("five", 55),
            ("six", 66),
            ("seven", 77),
            ("eight", 88),
            ("nine", 99),
            ("sev8en", 88),
            ("sev8enine", 89),
            ("trknlxnv43zxlrqjtwonect", 41),
            ("nineightwoneighthreeight", 98),
        ];
        cases
            .iter()
            .for_each(|(word, num)| assert_eq!(parse_line(word), *num));
    }
}
