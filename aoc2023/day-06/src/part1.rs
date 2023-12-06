pub fn process(input: &str) -> String {
    // let mut hm: HashMap<u32, u32> = HashMap::new();
    let mut times: Vec<u32> = vec!();
    let mut records: Vec<u32> = vec!();
    input
    .lines()
    .enumerate()
    .for_each(|(index, line)| {
        if index == 0{
            line.split_whitespace()
            .skip(1)
            .for_each(|time|{
                times.push(time.parse::<u32>().unwrap());
            });
        }else{
            line.split_whitespace()
            .skip(1)
            .for_each(|distance|{
                records.push(distance.parse::<u32>().unwrap());
            });

        }
    });
    let p = times.iter().enumerate().map(|(index, time)|{
        let record = records.get(index).unwrap();
        let lower = (*time as f32 - f32::sqrt((time*time) as f32 - 4_f32 * *record as f32) as f32) / 2_f32;
        let higher = (*time as f32 + f32::sqrt((time*time) as f32 - 4_f32 * *record as f32) as f32) / 2_f32;
        
        let lower = if lower == (lower as u32) as f32{ lower + 1.0} else {lower.ceil()};
        let higher = if higher == (higher as u32) as f32{ higher - 1.0} else {higher.floor()};
        let x = higher - lower + 1.;
        // dbg!(lower, higher, x);
        x
    })
    .product::<f32>() as u32;
    // dbg!(p);
    p.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "288");
    }
}
