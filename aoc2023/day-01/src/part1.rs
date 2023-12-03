pub fn process(input: &str) -> String {
    let x: u32 = input
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .fold(0, |mut acc, line| {
            let y: Vec<u32> = line
                .chars()
                .filter(|x| x.is_ascii_digit())
                .map(|x| x.to_digit(10).unwrap())
                .collect();
            acc += y[0] * 10 + y[y.len() - 1];
            acc
        });
    // dbg!(x);
    x.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "142");
    }
}
