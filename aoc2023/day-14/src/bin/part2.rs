extern crate dotenv;

use day_14::part2::process;
use day_14::util::get_input;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let input = get_input(2);
    let output = process(&input);
    dbg!(output);
}
