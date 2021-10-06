mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

macro_rules! task {
    ($func_name:expr) => {
        {
            let (res, time) = $func_name;
            println!("{}: {} - {}us", stringify!($func_name), res, time);
        }
    };
}
fn main() {
    task!(day1::part1());
    task!(day1::part2());
    task!(day2::part1());
    task!(day2::part2());
    task!(day3::part1());
    task!(day3::part2());
    task!(day4::part1());
    task!(day4::part2());
    println!("Day 5 left out...");
    //task!(day5::part1()); // ~4s
    //task!(day5::part2()); // ~14s, consider removing this day
    task!(day6::part1());
    task!(day6::part2());
    task!(day7::part1());
    task!(day7::part2());
}
