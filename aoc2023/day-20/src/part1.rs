use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Broadcaster<'a>{
    output: u8,
    targets: Vec<&'a str>,
}

impl<'a> Broadcaster<'a>{
    fn new(name: &'a str, target_raw: &'a str) -> Self{
        Self { 
            output: 0,
            targets: target_raw.split(", ").collect::<Vec<&str>>(),
        }
    }

    fn send(&self, hm: &mut HashMap<&str, Module> ){
        self.targets.iter().for_each(|m|{
            let ref mut module = hm.get(m).unwrap();
            match module{
                Module::FlipFlop(f) => {
                    f.clone().set_input(self.output)
                },
                Module::Conjunction(c) => {
                    c.clone().set_input(self.output)
                },
                Module::Broadcaster(_) => todo!(),
            }
        });
    }

    fn set_input(&mut self, input: u8){
        // self.input = input
    }
}

#[derive(Debug ,Clone)]
struct  FlipFlop<'a>{
    name: &'a str,
    status: bool,
    input: u8,
    targets: Vec<&'a str>,
}

impl<'a> FlipFlop<'a>{
    fn new(name: &'a str, target_raw: &'a str) -> Self{
        Self { 
            name: name, 
            status: false, 
            input: 0, 
            targets: target_raw.split(", ").collect::<Vec<&str>>(),
        }
    }

    fn set_input(&mut self, input: u8){
        self.input = input
    }
}

#[derive(Debug, Clone)]
struct Conjunction<'a>{
    name: &'a str,
    inputs: Vec<Module<'a>>,
    targets: Vec<&'a str>,
}

impl<'a> Conjunction<'a>{
    fn new(name: &'a str, target_raw: &'a str) -> Self{
        Self { 
            name: name, 
            inputs: vec![],
            targets: target_raw.split(", ").collect::<Vec<&str>>(),
        }
    }

    fn set_input(&mut self, input: u8){
        
    }
}

#[derive(Debug, Clone)]
enum Module<'a>{
    Broadcaster(Broadcaster<'a>),
    FlipFlop(FlipFlop<'a>),
    Conjunction(Conjunction<'a>)
}

impl<'a> Module<'a>{
    fn name(&self) -> &'a str{
        match self{
            Module::Broadcaster(_b) => "broadcaster",
            Module::FlipFlop(f) => f.name,
            Module::Conjunction(c) => c.name,
        }
    }

    fn send(&self,hm: &mut HashMap<&str, Module> ){
        match self{
            Module::Broadcaster(b) => b.send(hm),
            _ => (),
            // Module::FlipFlop(f) => f.name,
            // Module::Conjunction(c) => c.name,
        }
    }
}

pub fn process(input: &str) -> String {
    let mut hm: HashMap<&str, Module> = HashMap::new();
    input.lines().into_iter().for_each(|l|{
        let source = l.split(" -> ").collect::<Vec<&str>>()[0];
        let targets_raw = l.split(" -> ").collect::<Vec<&str>>()[1];
        let x: Module = match source.chars().nth(0).unwrap(){
            'b' => Module::Broadcaster(Broadcaster::new("broadcaster",targets_raw)),
            '%' => {
                let name = &source[1..][..];
                Module::FlipFlop(FlipFlop::new(name, targets_raw))
            },
            '&' => {
                let name = &source[1..][..];
                Module::Conjunction(Conjunction::new(name, targets_raw))
            },
            _ => !unreachable!(),
        };
        hm.insert(x.name(), x);
    });
    dbg!(&hm);
    // let b = hm.get_mut("broadcaster").unwrap().send(&mut hm);
    // b.send(&mut hm);
    // dbg!(&hm);
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "");
    }
}
