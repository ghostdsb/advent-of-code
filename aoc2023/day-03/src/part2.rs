#[derive(Debug, Copy, Clone)]
struct Run {
    is_valid: bool,
    number: u32,
    gear_position: Option<(usize, usize)>,
}

impl Run {
    fn new() -> Self {
        Run {
            is_valid: false,
            number: 0,
            gear_position: None,
        }
    }

    fn move_forward(&mut self, digit: char) {
        self.number = self.number * 10 + digit.to_digit(10).unwrap();
    }

    fn make_valid(&mut self) {
        self.is_valid = true;
    }

    fn set_gear_position(&mut self, y: usize, x: usize) {
        self.gear_position = Some((y, x));
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
                // dbg!(c);
                run.move_forward(*c);
                // dbg!(run);
                directions.iter().for_each(|(dy, dx)| {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;
                    if is_valid_neighbour_coordinate(&grid, y, x) {
                        let x = x as usize;
                        let y = y as usize;
                        if grid[y][x] != '.' && !grid[y][x].is_ascii_digit() {
                            run.make_valid();
                            if grid[y][x] == '*' {
                                run.set_gear_position(y, x);
                            }
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

    use std::collections::HashMap;

    let mut hm: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    runs.iter().for_each(|r| {
        if r.number != 0 && r.gear_position.is_some() {
            hm.entry(r.gear_position.unwrap())
                .and_modify(|nums| nums.push(r.number))
                .or_insert(vec![r.number]);
        }
    });
    let x: u32 = hm
        .values()
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum();

    // dbg!(x);

    x.to_string()
}

fn is_valid_neighbour_coordinate(grid: &Vec<Vec<char>>, y: i32, x: i32) -> bool {
    !(x < 0 || y < 0 || x >= grid.iter().nth(0).unwrap().len() as i32 || y >= grid.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "467835");
    }
}
