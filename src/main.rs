mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

enum TaskRunType {
    RunAllow,
    RunRestrict(&'static str, u128)
}

macro_rules! task {
    ($func_name:expr) => {
        let (res, time) = $func_name;
        println!("{:<14} {:<20} {:>10}us", stringify!($func_name), res, time);
    };
}

macro_rules! run_task {
    ($fun_name: path, $filename: expr, $run_type: expr) => {
        match $run_type {
            RunAllow => match std::fs::read_to_string($filename) {
                Ok(source) => {
                    let now = std::time::Instant::now();
                    let res = $fun_name(source);
                    let time = now.elapsed().as_micros();
                    println!("{:<14} {:<20} {:>10}us", stringify!($fun_name), res, time);
                },
                _ => println!("{}: File not found!", stringify!($fun_name))
            },
            RunRestrict(default_answer, default_time) => println!("{:<14} {:<20} {:>10}us (precomputed)", stringify!($fun_name), default_answer, default_time)
        };
    };
}

fn main() {
    use TaskRunType::*;
    println!("\n{:^48}\n", "### Advent Of Code 2016 ###");
    println!("{:<14} {:<20} {:>12}", "Task", "Answer", "Time in us");
    task!(day1::part1());
    task!(day1::part2());
    task!(day2::part1());
    task!(day2::part2());
    task!(day3::part1());
    task!(day3::part2());
    task!(day4::part1());
    task!(day4::part2());
    run_task!(day5::part1, "data/day5.txt", RunRestrict("f97c354d", 4000000));
    run_task!(day5::part2, "data/day5.txt", RunRestrict("863dde27", 14000000));
    task!(day6::part1());
    task!(day6::part2());
    task!(day7::part1());
    task!(day7::part2());
    run_task!(day8::part1, "data/day8.txt", RunAllow);
    run_task!(day8::part2, "data/day8.txt", RunAllow);
    run_task!(day9::part1, "data/day9.txt", RunAllow);
    run_task!(day9::part2, "data/day9.txt", RunAllow);
    run_task!(day10::part1, "data/day10.txt", RunAllow);
    run_task!(day10::part2, "data/day10.txt", RunAllow);
    run_task!(day11::part1, "data/day11.txt", RunRestrict("47", 900000));
    run_task!(day11::part2, "data/day11.txt", RunRestrict("71", 8700000));
    run_task!(day12::part1, "data/day12.txt", RunAllow);
    run_task!(day12::part2, "data/day12.txt", RunRestrict("9227647", 131932));
    run_task!(day13::part1, "data/day13.txt", RunAllow);
    run_task!(day13::part2, "data/day13.txt", RunAllow);
    run_task!(day14::part1, "data/day14.txt", RunAllow);
    run_task!(day14::part2, "data/day14.txt", RunRestrict("19968", 51634545));
    run_task!(day15::part1, "data/day15.txt", RunAllow);
    run_task!(day15::part2, "data/day15.txt", RunAllow);
    run_task!(day16::part1, "data/day16.txt", RunAllow);
    run_task!(day16::part2, "data/day16.txt", RunRestrict("01010001101011001", 187259));
    run_task!(day17::part1, "data/day17.txt", RunAllow);
    run_task!(day17::part2, "data/day17.txt", RunAllow);
    run_task!(day18::part1, "data/day18.txt", RunAllow);
    run_task!(day18::part2, "data/day18.txt", RunRestrict("20000795", 415684));
    run_task!(day19::part1, "data/day19.txt", RunAllow);
    run_task!(day19::part2, "data/day19.txt", RunAllow);
    run_task!(day20::part1, "data/day20.txt", RunAllow);
    run_task!(day20::part2, "data/day20.txt", RunAllow);
    run_task!(day21::part1, "data/day21.txt", RunAllow);
    run_task!(day21::part2, "data/day21.txt", RunAllow);
    run_task!(day22::part1, "data/day22.txt", RunAllow);
    run_task!(day22::part2, "data/day22.txt", RunAllow);
    run_task!(day23::part1, "data/day23.txt", RunAllow);
    run_task!(day23::part2, "data/day23.txt", RunRestrict("479009308", 31364285));
    run_task!(day24::part1, "data/day24.txt", RunRestrict("490", 1189579));
    run_task!(day24::part2, "data/day24.txt", RunRestrict("744", 1225393));
    run_task!(day25::part1, "data/day25.txt", RunAllow);
    run_task!(day25::part2, "data/day25.txt", RunAllow);
}
