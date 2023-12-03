extern crate dotenv;

use {{crate_name}}::part2::process;
use {{crate_name}}::util::get_input;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let input = get_input(2);
    let output = process(&input);
    dbg!(output);
}
