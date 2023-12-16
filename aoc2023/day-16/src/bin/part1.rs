extern crate dotenv;

use day_16::part1::process;
use day_16::util::get_input;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let input = get_input(1);
    let output = process(&input);
    dbg!(output);
}
