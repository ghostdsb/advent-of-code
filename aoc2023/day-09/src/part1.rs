pub fn process(input: &str) -> String {
    input
        .lines()
        .fold(0, |mut acc, line| {
            let vals: Vec<i64> = line
                .split_whitespace()
                .rev()
                .map(|v| v.parse::<i64>().unwrap())
                .collect();
            let lasts = create_subs(vals);
            acc += lasts.iter().sum::<i64>();
            acc
        })
        .to_string()
}

fn create_subs(mut vals: Vec<i64>) -> Vec<i64> {
    let mut firsts: Vec<i64> = vec![*vals.first().unwrap()];
    let mut final_num = i64::MAX;
    while final_num != 0 {
        vals = vals.windows(2).fold(vec![], |mut acc, window| {
            acc.push(window[0] - window[1]);
            acc
        });
        final_num = *vals.first().unwrap();
        firsts.push(final_num);
    }
    firsts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "114");
    }
}
