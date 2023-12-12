pub fn process(input: &str) -> String {
    input
        .lines()
        .fold(0, |mut acc, line| {
            let vals: Vec<i64> = line
                .split_whitespace()
                .rev()
                .map(|v| v.parse::<i64>().unwrap())
                .collect();
            let last_val = create_subs(vals)
                .iter()
                .enumerate()
                .fold(0, |mut acc, (index, val)| {
                    if index % 2 == 0 {
                        acc += val;
                    } else {
                        acc += -val;
                    }
                    acc
                });
            acc += last_val;
            acc
        })
        .to_string()
}

fn create_subs(mut vals: Vec<i64>) -> Vec<i64> {
    let mut firsts: Vec<i64> = vec![*vals.last().unwrap()];
    let mut all_zero = false;
    while !all_zero {
        vals = vals.windows(2).fold(vec![], |mut acc, window| {
            acc.push(window[0] - window[1]);
            acc
        });
        let final_num = *vals.last().unwrap();
        firsts.push(final_num);
        all_zero = vals.iter().all(|&v| v == 0);
    }
    firsts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "2");
    }
}
