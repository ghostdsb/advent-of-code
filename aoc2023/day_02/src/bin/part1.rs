use std::error::Error;
use day_02::part1::process;

fn main() {
    // let input = include_str!("../../input/final/input-1.txt");
    let input = get_input();
    let output = process(&input);
    println!("{}", output);
    dbg!(output);
}

fn get_input() -> String {
    let day = env!("CARGO_PKG_NAME")
    .chars()
    .filter(|c| c.is_ascii_digit())
    .rev()
    .enumerate()
    .fold(0, |mut acc, (place, digit)| {
        let digit = digit.to_digit(10).unwrap();
        acc += u8::pow(10, place as u32)*digit as u8;
        acc
    });
    match content_present(){
        Ok(data) => data,
        Err(_) => {
            match download_input(day){
                Ok(data) => data,
                Err(_) => "".to_string(),
            }
        }
    }
}

fn download_input(day: u8) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder()
        .build()?;
    let resp = client
        .get(format!("https://adventofcode.com/2023/day/{day}/input"))
        .header("cookie", "session=53616c7465645f5fce3d64f6cd728ad6cd1412a9126e57b3f65c935ce2e4e531130033135310d4986c9f1ce598b3e7b7d324519e3121742417811cf2954fd20b")
        .send()?
        .text()?;
    std::fs::write("input/final/input-1.txt", &resp)?;
    Ok(resp)
}

fn content_present() -> Result<String, Box<dyn Error>> {
    let data = std::fs::read_to_string(format!("input/final/input-1.txt"))?;
    Ok(data)
}