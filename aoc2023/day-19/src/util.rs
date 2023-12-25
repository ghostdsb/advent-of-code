use std::error::Error;
extern crate dotenv;

pub fn get_input(part: u8) -> String {
    let day = env!("CARGO_PKG_NAME")
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |mut acc, digit| {
            let digit = digit.to_digit(10).unwrap() as u8;
            acc = acc * 10 + digit;
            acc
        });
    match parse_local_cache(part) {
        Ok(data) => data,
        Err(_) => match parse_remote(day, part) {
            Ok(data) => data,
            Err(err) => {
                dbg!(err);
                "".to_string()
            }
        },
    }
}

fn parse_local_cache(part: u8) -> Result<String, Box<dyn Error>> {
    let data = std::fs::read_to_string(format!("input/final/input-{part}.txt"))?;
    if data.is_empty() {
        return Err("Empty input file".into());
    }
    Ok(data)
}

fn parse_remote(day: u8, part: u8) -> Result<String, Box<dyn Error>> {
    let year = dotenv::var("YEAR")?;
    let session = dotenv::var("SESSION")?;
    let session_cookie = format!("session={session}");

    let client = reqwest::blocking::Client::builder().build()?;
    let resp = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header("cookie", session_cookie)
        .send()?
        .text()?;
    std::fs::write(format!("input/final/input-{part}.txt"), &resp)?;
    Ok(resp)
}
