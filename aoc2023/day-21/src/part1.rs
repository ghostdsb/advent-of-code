use std::collections::{HashSet, VecDeque};

pub fn process(input: &str) -> String {
    let grid = input.lines().fold(vec![], |mut acc, line| {
        acc.push(line.chars().collect::<Vec<char>>());
        acc
    });
    let mut starting_position: (usize, usize) = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                starting_position = (i, j);
                break;
            }
        }
    }
    let last_positions = traverse(&grid, starting_position, 64);
    last_positions.to_string()
}

fn traverse(grid: &Vec<Vec<char>>, starting_position: (usize, usize), steps: usize) -> usize{
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut q: VecDeque<(usize, (usize, usize))> = VecDeque::new();
    q.push_back((0, starting_position));
    let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    // let mut last_points: Vec<(usize, usize)> = vec![];
    let mut ans = 0;
    let mut level:isize = -1;
    let last_level = steps;
    while !q.is_empty() {
        let (depth, (top_y, top_x)) = q.pop_front().unwrap();
        visited.insert((top_y, top_x));
        if level < depth as isize{
            level = depth as isize;
        }
        dirs.iter().for_each(|(dy, dx)| {
            let ny = top_y as i32 + dy;
            let nx = top_x as i32 + dx;
            if is_valid_neighbour_coordinate(&grid, ny as i32, nx as i32)
                && !visited.contains(&(ny as usize, nx as usize))
            {
                visited.insert((ny as usize, nx as usize));
                q.push_back((depth + 1, (ny as usize, nx as usize)));
            }
        });
        if depth % 2 == 0 {
            ans += 1;
        }
        if depth == last_level + 1{
            break;
        }
    }
    ans
}

fn is_valid_neighbour_coordinate(grid: &Vec<Vec<char>>, y: i32, x: i32) -> bool {
    !(x < 0
        || y < 0
        || x >= grid[0].len() as i32
        || y >= grid.len() as i32
        || grid[y as usize][x as usize] == '#')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        // let output = process(input);
        let grid = input.lines().fold(vec![], |mut acc, line| {
            acc.push(line.chars().collect::<Vec<char>>());
            acc
        });
        let mut starting_position: (usize, usize) = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 'S' {
                    starting_position = (i, j);
                    break;
                }
            }
        }
        let output = traverse(&grid, starting_position, 6).to_string();
        assert_eq!(output, "16");
    }
}
