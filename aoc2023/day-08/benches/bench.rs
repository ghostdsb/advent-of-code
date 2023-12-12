use day_08::part1;
use day_08::part2;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Define a `fibonacci` function and register it for benchmarking.
#[divan::bench]
fn part1() -> String {
    part1::process(divan::black_box(include_str!("../input/final/input-1.txt")))
    // compute(divan::black_box(10))
}

#[divan::bench]
fn part2() -> String {
    part2::process(divan::black_box(include_str!("../input/final/input-2.txt")))
    // compute(divan::black_box(10))
}
