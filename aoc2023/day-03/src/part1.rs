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
    let directions = [
        (-1, 0),
        (0, -1),
        (1, 0),
        (0, 1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    for i in 0..height {
        let line: Vec<char> = input.lines().nth(i).unwrap().chars().collect();
        grid.push(line);
    }
    grid.iter().enumerate().for_each(|(j, line)| {
        let mut run = Run::new();
        line.iter().enumerate().for_each(|(i, c)| {
            if c.is_ascii_digit() {
                run.move_forward(*c);
                directions.iter().for_each(|(dy, dx)| {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;
                    if is_valid_neighbour_coordinate(&grid, y, x) {
                        let x = x as usize;
                        let y = y as usize;
                        if grid[y][x] != '.' && !grid[y][x].is_ascii_digit() {
                            run.make_valid();
                        }
                    }
                });
                if i == grid.iter().nth(0).unwrap().len() - 1 {
                    runs.push(run);
                }
            } else {
                runs.push(run);
                run = Run::new();
            }
        })
    });

    runs.iter()
        .filter(|r| r.is_valid)
        .map(|run| run.number)
        .sum::<u32>()
        .to_string()
}

fn is_valid_neighbour_coordinate(grid: &Vec<Vec<char>>, y: i32, x: i32) -> bool {
    !(x < 0 || y < 0 || x >= grid.iter().nth(0).unwrap().len() as i32 || y >= grid.len() as i32)
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
