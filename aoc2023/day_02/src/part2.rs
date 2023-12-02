pub fn process(input: &str) -> String {
    let ans = input.lines().fold(0, |mut acc, line| {
        let game: Vec<&str> = line.split(':').collect();
        let handfuls: Vec<&str> = game
            .get(1)
            .unwrap()
            .split(';')
            .into_iter()
            .map(|c| c.trim())
            .collect();
        let mut game_power = [0, 0, 0];
        let _ = handfuls.iter().for_each(|handful| {
            handful.split(", ").into_iter().for_each(|set| {
                let color_frequency: Vec<&str> = set.split(" ").collect();
                let ball_count = color_frequency.get(0).unwrap().parse::<u32>().unwrap();
                let &color = color_frequency.get(1).unwrap();
                match color {
                    "red" => {
                        if ball_count >= game_power[0] {
                            game_power[0] = ball_count;
                        }
                    }
                    "green" => {
                        if ball_count >= game_power[1] {
                            game_power[1] = ball_count;
                        }
                    }
                    "blue" => {
                        if ball_count >= game_power[2] {
                            game_power[2] = ball_count;
                        }
                    }
                    _ => (),
                }
            });
        });
        acc += game_power.iter().filter(|&&x| x != 0).product::<u32>();
        acc
    });
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "2286");
    }
}
