pub fn process(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().fold(vec!(), |mut grid:Vec<Vec<char>>, line|{
        grid.push(line.chars().collect());
        grid
    });
    let ans = calculate_north_wall_load(grid);
    ans.to_string()
}

fn calculate_north_wall_load(grid: Vec<Vec<char>>) -> usize{
    let total_row = grid.len();
    // dbg!(&grid);
    let mut cols:Vec<Vec<usize>> = vec![];
    for j in 0..grid[0].len(){
        let mut row_count = 0;
        let mut rock_row:Vec<usize> = vec![];
        for i in 0..grid.len(){
            if grid[i][j] == '#'{
                row_count = i+1;
            }
            if grid[i][j] == 'O'{
                rock_row.push(row_count);
                row_count += 1;
            }
        }
        cols.push(rock_row);
    }
    cols.iter().map(|col|{
        col.iter().fold(0, |mut acc, row_index|{
            acc += total_row-row_index;
            acc
        })
    }).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "136");
    }
}
