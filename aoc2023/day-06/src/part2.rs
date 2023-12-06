pub fn process(input: &str) -> String {
    // let mut hm: HashMap<u32, u32> = HashMap::new();
    let mut time: u64 = 0;
    let mut record: u64 = 0;
    input
    .lines()
    .enumerate()
    .for_each(|(index, line)| {
        if index == 0{
            time = 
            line.chars().into_iter().fold(0, |mut acc, c|{
                if c.is_ascii_digit(){
                    acc = acc * 10 + c.to_digit(10).unwrap() as u64;
                }
                acc
            });
            // line.split_whitespace()
            // .skip(1)
            // .for_each(|time|{
            //     times.push(time.parse::<u32>().unwrap());
            // });
        }else{
            record = 
            line.chars().into_iter().fold(0, |mut acc, c|{
                if c.is_ascii_digit(){
                    acc = acc * 10 + c.to_digit(10).unwrap() as u64;
                }
                acc
            });

        }
    });
    let lower = (time as f64 - square_root((time*time) as f64 - 4_f64 * record as f64) ) / 2_f64;
    let higher = (time as f64 + square_root((time*time) as f64 - 4_f64 * record as f64)) / 2_f64;
    
    let lower = if lower == (lower as u32) as f64{ lower + 1.0} else {lower.ceil()};
    let higher = if higher == (higher as u32) as f64{ higher - 1.0} else {higher.floor()};
    let x = higher - lower + 1.;
        // dbg!(lower, higher, x);
    // dbg!(p);
    x.to_string()
}

fn square_root(number: f64) -> f64{
    f64::sqrt(number)
} 

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "71503");
    }
}
