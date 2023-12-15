struct Lens {
    name: String,
    focal_length: u8,
}

struct LensBox {
    id: usize,
    lens: Vec<Lens>,
}

impl Lens {
    fn new(name: String, focal_length: u8) -> Self {
        Self { name, focal_length }
    }
}

impl LensBox {
    fn new(id: usize) -> Self {
        Self { id, lens: vec![] }
    }

    fn remove(&mut self, lens_name: String) {
        if let Some(index) = self.lens.iter().position(|l| l.name == lens_name) {
            self.lens.remove(index);
        };
    }

    fn insert(&mut self, lens: Lens) {
        if let Some(index) = self.lens.iter().position(|l| l.name == lens.name) {
            self.lens[index].focal_length = lens.focal_length;
        } else {
            self.lens.push(lens);
        };
    }

    fn score(&self) -> u64 {
        self.lens
            .iter()
            .enumerate()
            .fold(0, |mut acc, (index, lens)| {
                acc += (self.id as u64 + 1) * (index as u64 + 1) * lens.focal_length as u64;
                acc
            })
    }
}

pub fn process(input: &str) -> String {
    let mut boxes: Vec<LensBox> = vec![];
    for i in 0..256 {
        boxes.push(LensBox::new(i));
    }
    input.split(',').into_iter().for_each(|seq| {
        if seq.contains('=') {
            let lens_name_raw = seq.split('=').into_iter().nth(0).unwrap();
            let focal_length = seq
                .split('=')
                .into_iter()
                .nth(1)
                .unwrap()
                .parse::<u8>()
                .unwrap();
            let lens_name = get_hash(lens_name_raw);
            boxes[lens_name as usize].insert(Lens::new(lens_name_raw.to_string(), focal_length));
        } else {
            let lens_name_raw = seq.split('-').into_iter().nth(0).unwrap();
            let lens_name = get_hash(lens_name_raw);
            boxes[lens_name as usize].remove(lens_name_raw.to_string());
        }
    });
    let ans = boxes.iter().fold(0, |mut acc, b| {
        acc += b.score();
        acc
    });
    ans.to_string()
}

fn get_hash(sequence: &str) -> u64 {
    sequence.chars().fold(0, |mut acc, c| {
        acc = ((acc + c as u64) * 17) % 256;
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "145");
    }
}
