extern crate dotenv;

use day_07::part2::process;
use day_07::util::get_input;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let input = get_input(2);
    let output = process(&input);
    dbg!(output);
}