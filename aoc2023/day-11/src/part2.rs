pub fn process(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let distance_factor = 1000000;
    input.lines().enumerate().for_each(|(row, line)|{
        grid.push(line.chars().collect());
        if line.chars().all(|c| c == '.'){
            empty_rows.push(row);
        }
    });
    for i in 0..grid[0].len(){
        let mut empty = true;
        for j in 0..grid.len(){
            if grid[j][i] != '.'{
                empty = false;
                break;
            }
        }
        if empty{
            empty_cols.push(i);
        }
    }
    for i in 0..grid[0].len(){
        for j in 0..grid.len(){
            if grid[j][i] == '#'{
                galaxies.push((j,i));
            }  
        }
    }
    let mut total_min_distance: u64 = 0;
    for i in 0..galaxies.len()-1{
        for j in i+1..galaxies.len(){
            let er = empty_rows.iter().filter(|&&r| {
                if galaxies[i].0 > galaxies[j].0{
                    r > galaxies[j].0 && r < galaxies[i].0
                }else{
                    r < galaxies[j].0 && r > galaxies[i].0
                }
            }).count() * (distance_factor-1);
            let ec = empty_cols.iter().filter(|&&r| {
                if galaxies[i].1 > galaxies[j].1{
                    r > galaxies[j].1 && r < galaxies[i].1
                }else{
                    r < galaxies[j].1 && r > galaxies[i].1
                }
            }).count() * (distance_factor-1);
            let horizontal_distance = (galaxies[i].0 as i32 - galaxies[j].0 as i32).abs() + er as i32;
            let vertical_distance = (galaxies[i].1 as i32 - galaxies[j].1 as i32).abs() + ec as i32;
            total_min_distance += horizontal_distance as u64 + vertical_distance as u64;
        }
    }
    dbg!(empty_cols, empty_rows, galaxies);
    total_min_distance.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "1030");
    }
}
