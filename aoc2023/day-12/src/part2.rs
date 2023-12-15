use std::collections::HashMap;

fn solve(
    run: &String,
    ans: &Vec<usize>,
    run_index: usize,
    ans_index: usize,
    cur_index: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(&result) = cache.get(&(run_index, ans_index, cur_index)) {
        return result;
    }
    if run_index == run.len() {
        if ans_index != ans.len() {
            return 0;
        }
        return 1;
    }
    let c = run.chars().nth(run_index).unwrap();

    if ans_index == ans.len() {
        if c == '#' {
            return 0;
        }
        return solve(run, ans, run_index + 1, ans_index, 0, cache);
    }

    let mut ret = 0;

    if c == '.' || c == '?' {
        if cur_index == ans[ans_index] {
            ret += solve(&run, &ans, run_index + 1, ans_index + 1, 0, cache);
        }
        if cur_index == 0 {
            ret += solve(&run, &ans, run_index + 1, ans_index, 0, cache);
        }
    }

    if c == '#' || c == '?' {
        ret += solve(run, ans, run_index + 1, ans_index, cur_index + 1, cache)
    }

    cache.insert((run_index, ans_index, cur_index), ret);

    return ret;
}

pub fn process(input: &str) -> String {
    let mut ans = 0;
    let mut count = 0;
    input.lines().for_each(|l| {
        let x = l.split(" ").nth(0).unwrap();
        let y = l
            .split(" ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut a: String = x.to_string().clone();
        let mut b = y.clone();
        for _ in 0..4 {
            a = format!("{}?{}", a, x);
            b.append(&mut y.clone());
        }
        count = count + 1;
        let comb = solve(&(a + "."), &b, 0, 0, 0, &mut HashMap::new());
        ans += comb;
    });
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "525152");
    }
}
