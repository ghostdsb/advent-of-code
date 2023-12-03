pub fn process(input: &str) -> String {
    let x: u32 = input.lines().enumerate().fold(0, |mut ans, (i, line)| {
        let game: Vec<&str> = line.split(':').collect();
        let handfuls: Vec<&str> = game
            .get(1)
            .unwrap()
            .trim()
            .split(';')
            .into_iter()
            .map(|c| c.trim())
            .collect();
        let possible = handfuls.iter().all(|handful| {
            handful.split(", ").into_iter().all(|set| {
                let color_frequency: Vec<&str> = set.split(" ").collect();
                let ball_count = color_frequency.get(0).unwrap().parse::<u32>().unwrap();
                let &color = color_frequency.get(1).unwrap();
                match color {
                    "red" => ball_count <= 12,
                    "green" => ball_count <= 13,
                    "blue" => ball_count <= 14,
                    _ => false,
                }
            })
        });
        // dbg!(handfuls);
        if possible {
            ans += i as u32 + 1;
        }
        ans
    });
    // .collect();
    dbg!(x);
    x.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "8");
    }
}
