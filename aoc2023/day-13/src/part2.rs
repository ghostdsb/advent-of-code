pub fn process(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut ans = 0;
    parts.iter().for_each(|part| {
        let grid1 = part.lines().fold(vec![], |mut grid: Vec<Vec<char>>, line| {
            grid.push(line.chars().collect());
            grid
        });

        let mut part1: Vec<String> = vec![];

        for j in 0..grid1[0].len() {
            let mut col = String::new();
            for i in 0..grid1.len() {
                col.push(grid1[i][j]);
            }
            part1.push(col);
        }

        let c = find_mirror(&part1);
        dbg!(c);

        if c != 0 {
            ans += c
        } else {
            let part2 = part.lines().fold(vec![], |mut grid: Vec<String>, line| {
                grid.push(line.chars().collect());
                grid
            });
            let r = find_mirror(&part2);
            dbg!(r);
            ans += 100 * r
        }
    });
    ans.to_string()
}

fn find_mirror(vec: &Vec<String>) -> usize {
    for r in 1..vec.len() {
        let above: Vec<String> = vec[..r].iter().rev().cloned().collect();
        let below: Vec<String> = vec[r..].to_vec();

        let diff_count = above
            .iter()
            .zip(below.iter())
            .map(|(a, b)| {
                a.chars()
                    .zip(b.chars())
                    .filter(|&(ca, cb)| ca != cb)
                    .count()
            })
            .sum::<usize>();

        if diff_count == 1 {
            return r;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "400");
    }
}
