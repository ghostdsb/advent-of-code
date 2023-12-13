use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::vec;

pub fn process(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut starting_index = (0, 0);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let hm: HashMap<char, Vec<char>> = HashMap::from([
        ('S', vec!['E', 'W', 'N', 'S']),
        ('|', vec!['N', 'S']),
        ('-', vec!['E', 'W']),
        ('L', vec!['E', 'N']),
        ('J', vec!['W', 'N']),
        ('F', vec!['E', 'S']),
        ('7', vec!['W', 'S']),
    ]);
    input.lines().enumerate().for_each(|(i, line)| {
        grid.push(vec![]);
        line.chars().enumerate().for_each(|(j, c)| {
            grid[i].push(c);
            if c == 'S' {
                starting_index = (i, j);
            }
        });
    });
    let mut stack: VecDeque<(usize, (usize, usize))> = VecDeque::new();
    stack.push_back((0, starting_index));
    visited.insert(starting_index);
    let directions: Vec<(i32, i32)> = vec![
        (1, 0),  // south
        (-1, 0), // north
        (0, -1), // west
        (0, 1),  // east
    ];
    let mut max_distance: u64 = u64::MIN;
    while !stack.is_empty() {
        let (depth, top) = stack.pop_front().unwrap();
        max_distance = if depth as u64 > max_distance {
            depth as u64
        } else {
            max_distance
        };
        directions.iter().for_each(|(dy, dx)| {
            let x = top.1 as i32 + dx;
            let y = top.0 as i32 + dy;
            if is_valid_neighbour_coordinate(&grid, y, x) {
                let x = x as usize;
                let y = y as usize;
                let direction = match (dy, dx) {
                    (1, 0) => 'S',
                    (-1, 0) => 'N',
                    (0, -1) => 'W',
                    (0, 1) => 'E',
                    _ => 'E',
                };
                if is_valid_connector(grid[top.0][top.1], grid[y][x], direction, &hm)
                    && !visited.contains(&(y, x))
                {
                    stack.push_back((depth + 1, (y, x)));
                    visited.insert((y, x));
                }
            }
        });
    }
    max_distance.to_string()
}

fn is_valid_neighbour_coordinate(grid: &Vec<Vec<char>>, y: i32, x: i32) -> bool {
    !(x < 0 || y < 0 || x >= grid.iter().nth(0).unwrap().len() as i32 || y >= grid.len() as i32)
}

fn is_valid_connector(
    me: char,
    other: char,
    direction: char,
    hm: &HashMap<char, Vec<char>>,
) -> bool {
    // dbg!(me, other, direction);
    if other == '.' {
        return false;
    }

    let my_connectors: HashSet<&char> = hm.get(&me).unwrap().into_iter().collect();
    let other_connectors: HashSet<&char> = hm
        .get(&other)
        .unwrap()
        .into_iter()
        .map(|d: &char| match d {
            'S' => &'N',
            'N' => &'S',
            'E' => &'W',
            'W' => &'E',
            _ => &'E',
        })
        .collect();

    my_connectors
        .intersection(&other_connectors)
        .collect::<Vec<&&char>>()
        .contains(&&&direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "8");
    }
}
