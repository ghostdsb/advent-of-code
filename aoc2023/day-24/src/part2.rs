use z3::ast::Ast;
#[derive(Debug)]
struct Hailstone{
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64
}

impl Hailstone{
    fn new(data: &Vec<f64>)-> Self{
        Self { px: data[0], py: data[1], pz: data[2], vx: data[3], vy: data[4], vz: data[5] }
    }
}

pub fn process(input: &str) -> String {
    let hailstones = input.lines().fold(vec![], |mut acc, line| {
        let x = line
            .split(&[' ', ',', '@'])
            .filter(|c| c != &"")
            .map(|c| c.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        let hailstone = Hailstone::new(&x);
        acc.push(hailstone);
        acc
    });

    // for (i, hailstone) in hailstones.iter().take(3).enumerate(){
    //     println!("vx * t{2} + px = {1} * t{2} + {0}", hailstone.px, hailstone.vx, i);
    //     println!("vy * t{2} + py = {1} * t{2} + {0}", hailstone.py, hailstone.vy, i);
    //     println!("vz * t{2} + px = {1} * t{2} + {0}", hailstone.pz, hailstone.vz, i);
    //     // dbg!(hailstone);
    // }

    // vx*t0 + px = h[0].vx * t0 + h[0].px
    // vy*t0 + py = h[0].vy * t0 + h[0].py
    // vz*t0 + pz = h[0].vz * t0 + h[0].pz

    // vx*t1 + px = h[1].vx * t1 + h[1].px
    // vy*t1 + py = h[1].vy * t1 + h[1].py
    // vz*t1 + pz = h[1].vz * t1 + h[1].pz

    // vx*t2 + px = h[2].vx * t2 + h[2].px
    // vy*t2 + py = h[2].vy * t2 + h[2].py
    // vz*t2 + pz = h[2].vz * t2 + h[2].pz
    
    // t0, t1 ,t2, vx, vy, vz, px, py, pz 
    
    let cfg = z3::Config::new();
    let context = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&context);
    let l = z3::ast::Real::new_const(&context, "px");
    let x = z3::ast::Real::new_const(&context, "px");
    let y = z3::ast::Real::new_const(&context, "py");
    let z = z3::ast::Real::new_const(&context, "pz");
    let vx = z3::ast::Real::new_const(&context, "vx");
    let vy = z3::ast::Real::new_const(&context, "vy");
    let vz = z3::ast::Real::new_const(&context, "vz");

    for (i, hailstone) in hailstones.iter().take(3).enumerate() {
        let a = z3::ast::Real::from_int(&z3::ast::Int::from_i64(&context, hailstone.px as i64));
        let b = z3::ast::Real::from_int(&z3::ast::Int::from_i64(&context, hailstone.py as i64));
        let c = z3::ast::Real::from_int(&z3::ast::Int::from_i64(&context, hailstone.pz as i64));
        let va = z3::ast::Real::from_int(&z3::ast::Int::from_i64(&context, hailstone.vx as i64));
        let vb = z3::ast::Real::from_int(&z3::ast::Int::from_i64(&context, hailstone.vy as i64));
        let vc = z3::ast::Real::from_int(&z3::ast::Int::from_i64(&context, hailstone.vz as i64));

        let t = z3::ast::Real::new_const(&context, format!("t{i}"));
        solver.assert(&t.gt(&z3::ast::Real::from_int(&z3::ast::Int::from_i64(&context, 0_i64))));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }
    let mut ans: String = String::new(); 
    if solver.check() == z3::SatResult::Sat {
        let Some(m) = solver.get_model() else {
            println!("Failed to solve!");
            return "XX".to_string();
        };
        ans = format!("{}", m.eval(&(x + y + z), true).unwrap());
    }

    ans
}
// 668042796359267
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "47");
    }
}