#[macro_use]
pub mod runner{
    macro_rules! aoc {
        ($p:path; $i:ident) => {
            {
                use $p as srcpath;
                srcpath::$i::day1();
            } 
        }
        // pub(crate) use aoc;
        // (day: u8, part: u8, module: mod) => {
        //     let day1_1 = day1::sol::aoc(1, 1);
        //     let day1_2 = day1::sol::aoc(1, 2);
        //     println!("day1_1: {}", day1_1);
        //     println!("day1_2: {}", day1_2);
        // };
    }
}