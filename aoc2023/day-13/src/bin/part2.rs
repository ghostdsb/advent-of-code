extern crate dotenv;

use day_13::part2::process;
use day_13::util::get_input;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let input = get_input(2);
    let output = process(&input);
    dbg!(output);
}
