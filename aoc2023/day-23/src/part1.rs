use std::collections::BTreeSet;

pub fn process(input: &str) -> String {
    let grid = input.lines().fold(vec![], |mut acc, line|{
        acc.push(line.chars().collect::<Vec<char>>());
        acc
    });
    let starting_position = grid[0].iter().position(|&c| c == '.').take().map(|x|(0_usize,x)).unwrap();
    let ending_position = grid[grid.len()-1].iter().position(|&c| c == '.').take().map(|x|(grid.len()-1,x)).unwrap();
    let dirs = vec![(0,-1), (0,1), (-1,0), (1, 0)];
    dbg!(starting_position, ending_position);
    dfs(&grid, starting_position, ending_position, dirs);
    "input".to_string()
}

fn dfs(grid: &Vec<Vec<char>>, starting_position: (usize, usize), ending_position: (usize, usize), dirs: Vec<(i32, i32)>) {
    let mut stack: Vec<(usize, (usize, usize))> = vec![(0, starting_position)];
    let mut visited: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut path: Vec<(usize, usize)> = vec![];
    let mut pathx: Vec<(usize, (usize, usize))> = vec![];
    let mut path_count = 100000; 
    let mut max_paths = usize::MIN;
    while !stack.is_empty() && path_count > 0{
        let  (d, (top_y, top_x)) = stack.pop().unwrap();
        visited.insert((top_y as usize, top_x as usize));
        pathx.push((d, (top_y as usize, top_x as usize)));
        // dbg!(top_y, top_x);
        if (top_y, top_x) == ending_position{
            // print_grid(grid, &pathx);
            max_paths = max_paths.max(pathx.last().unwrap().0);
            // println!("{}", &pathx.last().unwrap().0);
            pathx = vec![];
            visited.clear();
            // dbg!(&visited);
            // dbg!(&stack);
            path_count -= 1;
            continue;
        }

        for (dy, dx) in dirs.iter(){

            match (grid[top_y][top_x], (dy, dx)){
                ('>', (0, 1)) => {},
                ('>', _) => continue,
                ('<', (0, -1)) => {},
                ('<', _) => continue,
                ('^', (-1, 0)) => {},
                ('^', _) => continue,
                ('v', (1, 0)) => {},
                ('v', _) => continue,
                _ => {}
            };

            let ny = top_y as i32 + dy;
            let nx = top_x as i32+ dx;
            let can_step = if is_valid_neighbour_coordinate(&grid, ny, nx){
                match ((dy, dx), grid[ny as usize][nx as usize]){
                    ((0, 1), next) if next != '<' => true,
                    ((0, -1), next) if next != '>' => true,
                    ((-1, 0), next) if next != 'v' => true,
                    ((1, 0), next) if next != '^' => true,
                    _ => false,
                }
            }
            else{
                false
            };
            if can_step && !visited.contains(&(ny as usize, nx as usize)){
                stack.push((d + 1, (ny as usize, nx as usize)));
                path.push((ny as usize, nx as usize));
                // x += 1;
                // pathx.push((d + 1, (ny as usize, nx as usize)));
            }
            
        }
    }
    println!("max: {}",max_paths);
    // print_grid(grid, pathx);
}

fn print_grid(grid: &Vec<Vec<char>>, path: &Vec<(usize, (usize, usize))>){
    let mut g: Vec<Vec<String>> = vec![vec![format!("{:04}", "."); grid[0].len()]; grid.len()];
    for (d, (y,x)) in path.iter(){
        g[*y][*x] = (d+1).to_string();
    }
    for i in 0..g.len(){
        for j in 0..g[0].len(){
            print!("{}", format!("{:04}", g[i][j]));
        }
        println!();
    }
}

fn is_valid_neighbour_coordinate(grid: &Vec<Vec<char>>, y: i32, x: i32) -> bool {
    !(x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 || grid[y as usize][x as usize]  == '#')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "");
    }
}
