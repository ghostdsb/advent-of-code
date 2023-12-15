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

        let c = find_mirror(&part1) + 1;
        // dbg!(c);

        if c != 0 {
            ans += c
        } else {
            let part2 = part.lines().fold(vec![], |mut grid: Vec<String>, line| {
                grid.push(line.chars().collect());
                grid
            });
            let r = find_mirror(&part2) + 1;
            // dbg!(r);
            ans += 100 * r
        }
    });
    ans.to_string()
}

fn find_mirror(vec: &Vec<String>) -> i32 {
    let mut l: i32 = 0;
    let mut r: i32 = 1;
    for i in 0..vec.len() - 1 {
        if vec[i] == vec[i + 1] {
            l = i as i32;
            r = i as i32 + 1;
        }
        while l >= 0 && r as usize <= vec.len() - 1 && vec[l as usize] == vec[r as usize] {
            l -= 1;
            r += 1;
        }
        if l < 0 || r as usize >= vec.len() {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "405");
    }
}
