use std::collections::HashMap;

pub fn process(input: &str) -> String {
    let mut mappings: HashMap<String, HashMap<(u64, u64), u64>> = HashMap::new();
    let mut current_map = "";
    let mut seeds: Vec<u64> = Vec::new();
    let _ = input.lines().filter(|l| !l.is_empty()).for_each(|l| {
        if l.contains(":") {
            let (command, options) = l.split_at(l.find(':').unwrap());
            if command == "seeds" {
                options
                    .split_whitespace()
                    .for_each(|dig| match dig.parse::<u64>() {
                        Ok(num) => seeds.push(num),
                        _ => {}
                    });
            } else {
                let (map_type, _) = command.split_at(command.find(" ").unwrap());
                current_map = map_type;
            }
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

    let mut min_location = u64::MAX;

    for &seed in seeds.iter() {
        let soil = a_to_b(&mappings.get("seed-to-soil").unwrap(), seed);
        let fertilizer = a_to_b(&mappings.get("soil-to-fertilizer").unwrap(), soil);
        let water = a_to_b(&mappings.get("fertilizer-to-water").unwrap(), fertilizer);
        let light = a_to_b(&&mappings.get("water-to-light").unwrap(), water);
        let temperature = a_to_b(&&mappings.get("light-to-temperature").unwrap(), light);
        let humidity = a_to_b(
            &&mappings.get("temperature-to-humidity").unwrap(),
            temperature,
        );
        let location = a_to_b(&&mappings.get("humidity-to-location").unwrap(), humidity);
        min_location = min_location.min(location);
        dbg!((
            seed,
            soil,
            fertilizer,
            water,
            light,
            temperature,
            humidity,
            location
        ));
    }
    min_location.to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "35");
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
