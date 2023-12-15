extern crate dotenv;

use day_14::part1::process;
use day_14::util::get_input;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let input = get_input(1);
    let output = process(&input);
    dbg!(output);
}
