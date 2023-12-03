use day_01::part1;
use day_01::part2;
use day_01::util::get_input;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Define a `fibonacci` function and register it for benchmarking.
#[divan::bench]
fn part1() -> String {
    part1::process(divan::black_box(&get_input(1)))
    // compute(divan::black_box(10))
}

#[divan::bench]
fn part2() -> String {
    part2::process(divan::black_box(&get_input(2)))
    // compute(divan::black_box(10))
}
