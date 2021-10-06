use std::fs;

#[allow(dead_code)]
const FILENAME: &str = "data/dayX.txt";

#[allow(dead_code)]
pub fn part1() -> (String, u128) {
    let source = fs::read_to_string(FILENAME).unwrap();
    let now = std::time::Instant::now();

    (String::from(""), now.elapsed().as_micros())
}

#[allow(dead_code)]
pub fn part2() -> (String, u128) {
    let source = fs::read_to_string(FILENAME).unwrap();
    let now = std::time::Instant::now();

    (String::from(""), now.elapsed().as_micros())
}