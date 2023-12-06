use std::collections::HashMap;

pub fn process(input: &str) -> String {
    let mut mappings: HashMap<String, HashMap<(u64, u64), u64>> = HashMap::new();
    let mut current_map = "";
    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let _ = input.lines().filter(|l| !l.is_empty()).for_each(|l| {
        if l.contains(":") {
            let (command, options) = l.split_at(l.find(':').unwrap());
            if command == "seeds" {
                options
                    .split_whitespace()
                    .filter(|c| *c != ":")
                    .map(|c| {
                        dbg!(c);
                        c.parse::<u64>().unwrap()
                    })
                    .collect::<Vec<u64>>()
                    .chunks(2)
                    .for_each(|seed_info|{
                        seeds.push((seed_info[0], seed_info[0] +seed_info[1] - 1));
                    });
            } else {
                let (map_type, _) = command.split_at(command.find(" ").unwrap());
                current_map = map_type;
            }
            // dbg!(command, options, &seed, current_map);
        } else {
            let data: Vec<u64> = l
                .split_whitespace()
                .map(|c| c.parse::<u64>().unwrap())
                .collect();
            mappings
                .entry(current_map.to_string())
                .and_modify(|hm| {
                    hm.insert((data[0], data[1]), data[2]);
                })
                .or_insert(HashMap::from([((data[0], data[1]), data[2])]));
        }
    });
    // let maps = [
    //     "seed-to-soil",
    //     "soil-to-fertilizer",
    //     "fertilizer-to-water",
    //     "water-to-light",
    //     "light-to-temperature",
    //     "temperature-to-humidity",
    //     "humidity-to-location",
    //     ];

    // dbg!(mappings);

    // let mut min_location = u64::MAX;

    // for seed in seeds.iter() {
    //     let soil = a_to_b(&mappings.get("seed-to-soil").unwrap(), *seed);
    //     let fertilizer = a_to_b(&mappings.get("soil-to-fertilizer").unwrap(), soil);
    //     let water = a_to_b(&mappings.get("fertilizer-to-water").unwrap(), fertilizer);
    //     let light = a_to_b(&&mappings.get("water-to-light").unwrap(), water);
    //     let temperature = a_to_b(&&mappings.get("light-to-temperature").unwrap(), light);
    //     let humidity = a_to_b(
    //         &&mappings.get("temperature-to-humidity").unwrap(),
    //         temperature,
    //     );
    //     let location = a_to_b(&&mappings.get("humidity-to-location").unwrap(), humidity);
    //     min_location = min_location.min(location);
    // }
    dbg!(seeds);
    "1".to_string()
}

fn a_to_b(map: &HashMap<(u64, u64), u64>, seed: u64) -> u64 {
    let mut val: Option<u64> = None;
    map.into_iter().for_each(|((b_start, a_start), jump)| {
        if seed >= *a_start && seed < a_start + jump {
            val = Some(seed - a_start + b_start);
        }
    });
    match val {
        Some(v) => v,
        None => seed,
    }
}

fn a_to_c(map: &HashMap<(u64, u64), u64>, seed_range: Vec<(u64, u64)>) -> u64 {
    // let mut val: Option<u64> = None;
    let mut v: Vec<(u64, u64)> = seed_range;
    map.into_iter().for_each(|((c_start, a_start), jump)| {
        let map_range = (*a_start, *a_start + jump - 1);
        let delta: i64 = *c_start as i64 - *a_start as i64;
        // v = ranger(map_range, v, delta);
        let mut x: Vec<(u64, u64)> = vec![];
        for (seed_l, seed_r) in v.iter(){
            x = ranger(map_range, (*seed_l, *seed_r), delta);
        }
        // x.iter().for_each(|m| )

    });
    0
    // match val {
    //     Some(v) => v,
    //     None => seed,
    // }
}

fn ranger(map_range: (u64, u64), seed_range: (u64, u64), delta: i64) -> Vec<(u64, u64)>{
    if seed_range.1 < map_range.0 {
        vec![seed_range]
    }else if seed_range.0 < map_range.0 && seed_range.1 >= map_range.0 && seed_range.1 <= map_range.1 {
        vec![(seed_range.0, map_range.0-1), ((map_range.0 as i64 + delta) as u64, (seed_range.1 as i64 + delta) as u64)]
    } else if seed_range.0 >= map_range.0 && seed_range.1 <= map_range.1{
        vec![((seed_range.0 as i64 + delta) as u64, (seed_range.1 as i64 + delta) as u64)]
    } else if seed_range.0 >= map_range.0 && seed_range.0 <= map_range.1 && seed_range.1 > map_range.1 {
        vec![((seed_range.0 as i64 + delta) as u64, (map_range.1 as i64 + delta) as u64), (map_range.1 + 1, seed_range.1)]
    } else if seed_range.0 > map_range.1{
        vec![seed_range]
    } else{
        vec![(seed_range.0, map_range.0-1), ((map_range.0 as i64 + delta) as u64, (map_range.1 as i64 + delta) as u64), (map_range.1+1, seed_range.1)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "46");
    }

    #[test]
    fn convertions() {
        let mut hm: HashMap<(u64, u64), u64> = HashMap::new();
        hm.insert((50, 98), 2);
        hm.insert((52, 50), 48);
        assert_eq!(a_to_b(&hm, 98), 50);
        assert_eq!(a_to_b(&hm, 50), 52);
        assert_eq!(a_to_b(&hm, 79), 81);
    }
}
