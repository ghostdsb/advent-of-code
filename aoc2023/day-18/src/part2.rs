pub fn process(input: &str) -> String {
    let mut curr_coord: (i64, i64) = (0,0); 
    let mut vertices: Vec<(i64, i64)> = vec![(0,0)];
    let mut perimeter = 0;
    input.lines().for_each(|line|{
        let line: Vec<&str> = line.split(&[' ', '(', ')', '#'][..]).collect();
        let color = line[4];
        let dir = color.chars().last().unwrap();
        let distance = distance_from_hex(&color[0..color.len()-1]);
        perimeter += distance;
        match dir{
            '2' => {
                curr_coord = (curr_coord.0 - distance as i64, curr_coord.1);
                vertices.push(curr_coord);
            },
            '0' => {
                curr_coord = (curr_coord.0 + distance as i64, curr_coord.1);
                vertices.push(curr_coord);
                
            },
            '3' => {
                curr_coord = (curr_coord.0, curr_coord.1 - distance as i64);
                vertices.push(curr_coord);
                
            },
            '1' => {
                curr_coord = (curr_coord.0, curr_coord.1 + distance as i64);
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

fn determinant((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    x1 as i64 * y2 as i64 - x2 as i64 * y1 as i64
}

fn distance_from_hex(hex: &str) -> u64{
    u64::from_str_radix(hex, 16).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "952408144115");
    }
}
