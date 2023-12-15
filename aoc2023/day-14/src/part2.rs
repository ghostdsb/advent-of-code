use std::collections::BTreeMap;

pub fn process(input: &str) -> String {
    let mut grid = input.lines().fold(vec!(), |mut grid:Vec<Vec<char>>, line|{
        grid.push(line.chars().collect());
        grid
    });

    let mut hm: BTreeMap<String, (usize, u64)> = BTreeMap::new();
    let cycles = 1000000000;
    let loop_factor: u64 ;
    let mut total_load: u64;
    let mut i =0;
    let mut load_loop: Vec<(usize, u64)> = Vec::new();
    loop {
        roll_north(&mut grid);
        roll_west(&mut grid);
        roll_south(&mut grid);
        roll_east(&mut grid);
        let a = calculate_north_wall_load(&grid);
        // println!("{}->{}", i, a);
        if !hm.contains_key(&encode(&grid)){
            hm.insert(encode(&grid), (i ,a as u64));
            load_loop.push((i,a as u64));
        }else{
            let (index, score) = hm.get(&encode(&grid)).unwrap();
            loop_factor = i as u64 - *index as u64;
            total_load = *score;
            break;
        }
        i += 1;

    }

    for (a, load) in load_loop.iter(){
        // println!("{} {} {}", a, load, (cycles - a - 1 ) % loop_factor as usize);
        if (cycles - a - 1) % loop_factor as usize == 0 {
            total_load = *load;
        }
    }

    let ans = total_load;

    ans.to_string()
}

fn encode(grid: &Vec<Vec<char>>) -> String{
    grid.iter().fold(String::new(), |mut acc, line|{
        acc += &line.iter().collect::<String>();
        acc.push('\n');
        acc
    })
}

fn roll_north(grid: &mut Vec<Vec<char>>) {
    let mut cols:Vec<Vec<usize>> =vec![];
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
    for i in 0..grid[0].len(){
        for j in 0..grid.len(){
            if grid[i][j] == '#'{
                continue;
            }
            if cols[j].contains(&i){
                grid[i][j] = 'O';
            }else{
                grid[i][j] = '.';
            }
        }
    }
    // grid
}

fn roll_south(grid: &mut Vec<Vec<char>>) {
    let mut cols:Vec<Vec<usize>> =vec![];
    for j in 0..grid[0].len(){
        let mut row_count:i32 = grid.len() as i32 - 1;
        let mut rock_row:Vec<usize> = vec![];
        for i in (0..grid.len()).rev(){
            if grid[i][j] == '#'{
                row_count = i as i32 -1;
            }
            if grid[i][j] == 'O'{
                rock_row.push(row_count as usize);
                row_count -= 1;
            }
        }
        cols.push(rock_row);
    }
    for i in 0..grid[0].len(){
        for j in 0..grid.len(){
            if grid[i][j] == '#'{
                continue;
            }
            if cols[j].contains(&i){
                grid[i][j] = 'O';
            }else{
                grid[i][j] = '.';
            }
        }
    }
    // grid
}

fn roll_west(grid: &mut Vec<Vec<char>>) {
    
    let mut cols:Vec<Vec<usize>> =vec![];

    for i in 0..grid.len(){
        let mut col_count = 0;
        let mut rock_col:Vec<usize> = vec![];
        for j in 0..grid[0].len(){
            if grid[i][j] == '#'{
                col_count = j+1;
            }
            if grid[i][j] == 'O'{
                rock_col.push(col_count);
                col_count += 1;
            }
        }
        cols.push(rock_col);
    }
    for i in 0..grid.len(){
        for j in 0..grid[0].len(){
            if grid[i][j] == '#'{
                continue;
            }
            if cols[i].contains(&j){
                grid[i][j] = 'O';
            }else{
                grid[i][j] = '.';
            }
        }
    }
    // grid
}

fn roll_east(grid: &mut Vec<Vec<char>>) {
    let mut cols:Vec<Vec<usize>> =vec![];

    for i in 0..grid.len(){
        let mut col_count = grid[0].len() as i32 -1 ;
        let mut rock_col:Vec<usize> = vec![];
        for j in (0..grid[0].len()).rev(){
            if grid[i][j] == '#'{
                col_count = j as i32 -1;
            }
            if grid[i][j] == 'O'{
                rock_col.push(col_count as usize);
                col_count -= 1;
            }
        }
        cols.push(rock_col);
    }
    for i in 0..grid.len(){
        for j in 0..grid[0].len(){
            if grid[i][j] == '#'{
                continue;
            }
            if cols[i].contains(&j){
                grid[i][j] = 'O';
            }else{
                grid[i][j] = '.';
            }
        }
    }
    // grid
}

fn calculate_north_wall_load(grid: &Vec<Vec<char>>) -> usize{
    let total_row = grid.len();
    let mut cols:Vec<Vec<usize>> = vec![];
    for j in 0..grid[0].len(){
        let mut rock_row:Vec<usize> = vec![];
        for i in 0..grid.len(){
            if grid[i][j] == 'O'{
                rock_row.push(i);
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
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "64");
    }
}
