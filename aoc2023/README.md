# AOC 2023

Solutions for Advent of Code 2023.

Trying out 
- cargo generate: creates new day project from  a template.
- benchmarks: divan
- http request: fetches input from advent of code
- file system: writes/reads inputs locally

For fetching input, this first checks if its locally available.

## Commands

- Prerequisites    
  > cargo install cargo-generate

- For starting project for a new day
  > make solution day=N, where N ∈ [01, 25] 

- FOr benchmarking
  > make bench day=N, where N ∈ [01, 25]
