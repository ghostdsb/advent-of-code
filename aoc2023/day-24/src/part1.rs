#[derive(Debug)]
struct Hailstone {
    px: f32,
    py: f32,
    pz: f32,
    vx: f32,
    vy: f32,
    vz: f32,
}

impl Hailstone {
    fn new(data: &Vec<f32>) -> Self {
        Self {
            px: data[0],
            py: data[1],
            pz: data[2],
            vx: data[3],
            vy: data[4],
            vz: data[5],
        }
    }

    fn intersection(&self, other: &Self) -> Option<(f32, f32)> {
        let nm = other.vy * self.py * self.vx
            - other.vx * other.py * self.vy
            - self.px * self.vy * other.vy
            + other.px * other.vy * self.vy;
        let dm = self.vx * other.vy - other.vx * self.vy;
        if dm.eq(&0.) {
            return None;
        }
        let y = nm / dm;
        let x = (y - self.py) * (self.vx / self.vy) + self.px;
        Some((x, y))
    }

    fn same_direction(&self, point: (f32, f32)) -> bool {
        let slx = point.0 - self.px;
        let sly = point.1 - self.py;

        if self.vx.gt(&0.) && self.vy.gt(&0.) {
            slx.gt(&0.) && sly.gt(&0.)
        } else if self.vx.gt(&0.) && self.vy.lt(&0.) {
            slx.gt(&0.) && sly.lt(&0.)
        } else if self.vx.lt(&0.) && self.vy.gt(&0.) {
            slx.lt(&0.) && sly.gt(&0.)
        } else if self.vx.lt(&0.) && self.vy.lt(&0.) {
            slx.lt(&0.) && sly.lt(&0.)
        } else if self.vx.eq(&0.) && self.vy.lt(&0.) {
            slx.eq(&0.) && sly.lt(&0.)
        } else if self.vx.eq(&0.) && self.vy.gt(&0.) {
            slx.eq(&0.) && sly.gt(&0.)
        } else if self.vx.gt(&0.) && self.vy.eq(&0.) {
            slx.gt(&0.) && sly.eq(&0.)
        } else if self.vx.lt(&0.) && self.vy.eq(&0.) {
            slx.lt(&0.) && sly.eq(&0.)
        } else {
            false
        }
    }
}

pub fn process(input: &str) -> String {
    let hailstones = input.lines().fold(vec![], |mut acc, line| {
        let x = line
            .split(&[' ', ',', '@'])
            .filter(|c| c != &"")
            .map(|c| c.parse::<f32>().unwrap())
            .collect::<Vec<f32>>();
        let hailstone = Hailstone::new(&x);
        acc.push(hailstone);
        acc
    });
    let mut ans = 0;
    let l = 200000000000000.;
    let r = 400000000000000.;
    for i in 0..hailstones.len() - 1 {
        for j in i..hailstones.len() {
            if let Some((x, y)) = hailstones[i].intersection(&hailstones[j]) {
                if x.gt(&l)
                    && x.lt(&r)
                    && y.gt(&l)
                    && y.lt(&r)
                    && hailstones[i].same_direction((x, y))
                    && hailstones[j].same_direction((x, y))
                {
                    ans += 1;
                }
            };
        }
    }
    dbg!(ans);
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "2");
    }
}
