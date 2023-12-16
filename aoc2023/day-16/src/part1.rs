use std::collections::HashSet;

pub fn process(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().fold(vec![], |mut lines, line| {
        lines.push(line.chars().collect::<Vec<char>>());
        lines
    });
    let mut path: Vec<((i32, i32), (i8, i8))> = vec![((0, 0), (1, 0))];
    let m = grid.len();
    let n = grid[0].len();
    // let mut set: HashSet<(i32, i32)> = HashSet::new();
    let mut set: HashSet<((i32, i32), (i8, i8))> = HashSet::new();
    while path.len() != 0 {
        let ((curr_y, curr_x), (dir_x, dir_y)) = path.pop().unwrap();
        set.insert(((curr_y, curr_x), (dir_x, dir_y)));
        let direction_vecs = get_next_dir(grid[curr_y as usize][curr_x as usize], (dir_x, dir_y));
        direction_vecs
            .iter()
            .filter(|&&dir_vec| dir_ok(curr_y + dir_vec.1, curr_x + dir_vec.0, m, n))
            .map(|dir_vec| {
                (
                    (curr_y + dir_vec.1, curr_x + dir_vec.0),
                    (dir_vec.0 as i8, dir_vec.1 as i8),
                )
            })
            .collect::<Vec<((i32, i32), (i8, i8))>>()
            .iter()
            .for_each(|&item| {
                if !set.contains(&item) {
                    path.push(item)
                }
            });
    }
    set.iter()
        .fold(
            HashSet::new(),
            |mut hs: HashSet<(i32, i32)>, ((y, x), _)| {
                hs.insert((*y, *x));
                hs
            },
        )
        .len()
        .to_string()
}

fn get_next_dir(block: char, (dir_x, dir_y): (i8, i8)) -> Vec<(i32, i32)> {
    let next_dirs = match (block, (dir_x, dir_y)) {
        ('.', _) => vec![(dir_x as i32, dir_y as i32)],
        ('|', (0, _)) => vec![(dir_x as i32, dir_y as i32)],
        ('|', (_x, _)) => vec![(0, -1), (0, 1)],
        ('-', (_, 0)) => vec![(dir_x as i32, dir_y as i32)],
        ('-', (_, _y)) => vec![(-1, 0), (1, 0)],
        ('\\', (1, 0)) => vec![(0, 1)],
        ('\\', (-1, 0)) => vec![(0, -1)],
        ('\\', (0, 1)) => vec![(1, 0)],
        ('\\', (0, -1)) => vec![(-1, 0)],
        ('/', (1, 0)) => vec![(0, -1)],
        ('/', (-1, 0)) => vec![(0, 1)],
        ('/', (0, 1)) => vec![(-1, 0)],
        ('/', (0, -1)) => vec![(1, 0)],
        _ => !unreachable!(),
    };
    next_dirs
}

fn dir_ok(y: i32, x: i32, m: usize, n: usize) -> bool {
    y >= 0 && x >= 0 && y < m as i32 && x < n as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "46");
    }
}
