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
    run_task!(day12::part2, "data/day12.txt", RunAllow);
    run_task!(day13::part1, "data/day13.txt", RunAllow);
    run_task!(day13::part2, "data/day13.txt", RunAllow);
}
