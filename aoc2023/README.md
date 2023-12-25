# AOC 2023

Solutions for Advent of Code 2023.

Trying out 
- cargo generate: creates new day project from  a template.
- benchmarks: divan
- http request: fetches input from advent of code
- file system: writes/reads inputs locally

For fetching input, this first checks if its locally available.
## Prerequisites
  - `.env` file with; 
        
        SESSION=session_string_from_advent_of_code
        YEAR=YYYY
  - > cargo install cargo-generate
## Commands


- For starting project for a new day
  > make solution day=N, where N ∈ [01, 25] 

- For benchmarking
  > make bench day=N, where N ∈ [01, 25]
