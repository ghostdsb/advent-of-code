/*
nums_vec = []

in current run
is_valid: false,
char -> check neighbours;
    if symbol -> is_valid: true
    if next symbol None | dot -> end run

on run end:
    if is_valid : push to nums_vec
    start next run
*/

#[derive(Debug, Copy, Clone)]
struct Run {
    is_valid: bool,
    number: u32,
}

impl Run {
    fn new() -> Self {
        Run {
            is_valid: false,
            number: 0,
        }
    }

    fn move_forward(&mut self, digit: char) {
        self.number = self.number * 10 + digit.to_digit(10).unwrap();
    }

    fn make_valid(&mut self) {
        self.is_valid = true;
    }
}

pub fn process(input: &str) -> String {
    let mut runs: Vec<Run> = vec![];
    let mut grid: Vec<Vec<char>> = vec![];
    let height = input.lines().count();

    for i in 0..height {
        let line: Vec<char> = input.lines().nth(i).unwrap().chars().collect();
        grid.push(line);
    }
    grid.iter().enumerate().for_each(|(j, line)| {
        let mut run = Run::new();
        line.iter().enumerate().for_each(|(i, c)| {
            if c.is_ascii_digit() {
                // dbg!(c);
                run.move_forward(*c);
                // dbg!(run);
                //west
                if check_stuff(&grid, j as i32, i as i32 - 1) {
                    if grid[j][i - 1] != '.' && !grid[j][i - 1].is_ascii_digit() {
                        run.make_valid();
                    }
                }
                //east
                if check_stuff(&grid, j as i32, i as i32 + 1) {
                    if grid[j][i + 1] != '.' && !grid[j][i + 1].is_ascii_digit() {
                        run.make_valid();
                    }
                }
                //north
                if check_stuff(&grid, j as i32 - 1, i as i32) {
                    // dbg!(grid[j-1][i]);
                    if grid[j - 1][i] != '.' && !grid[j - 1][i].is_ascii_digit() {
                        run.make_valid();
                    }
                }
                //south
                if check_stuff(&grid, j as i32 + 1, i as i32) {
                    // dbg!(grid[j+1][i]);
                    if grid[j + 1][i] != '.' && !grid[j + 1][i].is_ascii_digit() {
                        run.make_valid();
                    }
                }
                // south-west
                if check_stuff(&grid, j as i32 + 1, i as i32 + 1) {
                    if grid[j + 1][i + 1] != '.' && !grid[j + 1][i + 1].is_ascii_digit() {
                        run.make_valid();
                    }
                }
                // north-east
                if check_stuff(&grid, j as i32 - 1, i as i32 - 1) {
                    if grid[j - 1][i - 1] != '.' && !grid[j - 1][i - 1].is_ascii_digit() {
                        run.make_valid();
                    }
                }

                // north-west
                if check_stuff(&grid, j as i32 - 1, i as i32 + 1) {
                    if grid[j - 1][i + 1] != '.' && !grid[j - 1][i + 1].is_ascii_digit() {
                        run.make_valid();
                    }
                }
                // south-east
                if check_stuff(&grid, j as i32 + 1, i as i32 - 1) {
                    if grid[j + 1][i - 1] != '.' && !grid[j + 1][i - 1].is_ascii_digit() {
                        run.make_valid();
                    }
                }
                if i == grid.iter().nth(0).unwrap().len() - 1 {
                    runs.push(run);
                }
            } else {
                runs.push(run);
                run = Run::new();
            }
        })
    });

    let ans = runs.iter().fold(0, |mut acc, run| {
        if run.is_valid {
            acc += run.number;
        } else {
            acc += 0;
        }
        acc
    });

    ans.to_string()
}

fn check_stuff(grid: &Vec<Vec<char>>, y: i32, x: i32) -> bool {
    if x < 0 || y < 0 || x >= grid.iter().nth(0).unwrap().len() as i32 || y >= grid.len() as i32 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "4361");
    }
}
