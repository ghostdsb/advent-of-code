pub fn process(input: &str) -> String {
    let mut curr_coord: (i32, i32) = (0,0); 
    let mut vertices: Vec<(i32, i32)> = vec![(0,0)];
    let mut perimeter = 0;
    input.lines().for_each(|line|{
        let line: Vec<&str> = line.split(&[' ', '(', ')'][..]).collect();
        let dir = line[0];
        let distance = line[1].parse::<u64>().unwrap();
        let _color = line[2];
        perimeter += distance;
        match dir{
            "L" => {
                curr_coord = (curr_coord.0 - distance as i32, curr_coord.1);
                vertices.push(curr_coord);
            },
            "R" => {
                curr_coord = (curr_coord.0 + distance as i32, curr_coord.1);
                vertices.push(curr_coord);
                
            },
            "U" => {
                curr_coord = (curr_coord.0, curr_coord.1 - distance as i32);
                vertices.push(curr_coord);
                
            },
            "D" => {
                curr_coord = (curr_coord.0, curr_coord.1 + distance as i32);
                vertices.push(curr_coord);
            },
            _ => {},
        }

    });
    vertices.push((0,0));
    let area: i64 = vertices.windows(2).fold(0, |mut acc, window| {
        let a = window[0];
        let b = window[1];
        let d = determinant(a, b);
        acc += d;
        acc
    }) / 2;

    
    let dots = (area.abs() + 1 ) - perimeter as i64 / 2;
    
    let ans = dots + perimeter as i64;

    ans.to_string()
}

fn determinant((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i64 {
    x1 as i64 * y2 as i64 - x2 as i64 * y1 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "62");
    }
}
