use std::{collections::VecDeque, ops::Range};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<(u64, u64)>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn get_mapped_seeds(&self, seed_ranges: Vec<Range<u64>>) -> u64 {
        let mapped = self.maps.iter().fold(seed_ranges, |acc, m| m.process(acc));
        mapped.iter().map(|x| x.start).min().unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
struct AlmanacRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl AlmanacRange {
    fn new(destination_start: u64, source_start: u64, length: u64) -> Self {
        Self {
            destination_start,
            source_start,
            length,
        }
    }

    fn get_overlap(&self, r: &Range<u64>) -> Option<Range<u64>> {
        let overlap_start = r.start.max(self.source_start);
        let overlap_end = r.end.min(self.source_start + self.length);

        let overlap = overlap_start..overlap_end;
        (!overlap.is_empty()).then_some(overlap)
    }

    fn create_mapped_range(&self, overlap: Range<u64>) -> Range<u64> {
        let start_offset = overlap.start - self.source_start;
        let end_offset = overlap.end - self.source_start;

        let new_start = self.destination_start + start_offset;
        let new_end = self.destination_start + end_offset;

        new_start..new_end
    }
}

#[derive(Clone, Debug)]
struct AlmanacMap {
    ranges: Vec<AlmanacRange>,
}

impl AlmanacMap {
    fn new(ranges: &Vec<AlmanacRange>) -> Self {
        Self {
            ranges: ranges.to_vec(),
        }
    }

    fn process(&self, seeds: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut new_seeds = vec![];
        let mut seed_queue = VecDeque::from(seeds);
        while let Some(range) = seed_queue.pop_front() {
            let mapped: Vec<Range<u64>> = self
                .ranges
                .iter()
                .filter_map(|almanac_range| {
                    almanac_range.get_overlap(&range).map(|overlap| {
                        if overlap.start > range.start {
                            seed_queue.push_back(range.start..overlap.start)
                        }
                        if overlap.end < range.end {
                            seed_queue.push_back(overlap.end..range.end)
                        }
                        almanac_range.create_mapped_range(overlap)
                    })
                })
                .collect();

            if mapped.is_empty() {
                new_seeds.push(range);
            } else {
                new_seeds.extend(mapped);
            }
        }
        new_seeds
    }
}

pub fn process(input: &str) -> String {
    let mut range_vec: Vec<AlmanacRange> = vec![];
    let mut range_mapping_vec: Vec<AlmanacMap> = vec![];
    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let _ = input.lines().filter(|l| !l.is_empty()).for_each(|l| {
        if l.contains(":") {
            let (command, options) = l.split_at(l.find(':').unwrap());
            if command == "seeds" {
                options
                    .split_whitespace()
                    .filter(|c| *c != ":")
                    .map(|c| {
                        c.parse::<u64>().unwrap()
                    })
                    .collect::<Vec<u64>>()
                    .chunks(2)
                    .for_each(|seed_info| {
                        seeds.push((seed_info[0], seed_info[0] + seed_info[1]));
                    });
            } else {
                if range_vec.len() > 0 {
                    let a: AlmanacMap = AlmanacMap::new(&range_vec);
                    range_mapping_vec.push(a);
                    range_vec = vec![];
                }
            }
        } else {
            let data: Vec<u64> = l
                .split_whitespace()
                .map(|c| c.parse::<u64>().unwrap())
                .collect();
            let almanac_range = AlmanacRange::new(data[0], data[1], data[2]);
            range_vec.push(almanac_range);
        }
    });
    let data = Almanac {
        maps: range_mapping_vec,
        seeds,
    };

    let ranges = data
        .seeds
        .iter()
        .map(|(start, end)| *start..*end)
        .collect::<Vec<Range<u64>>>();

    let ans = data.get_mapped_seeds(ranges);
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "46");
    }
}
