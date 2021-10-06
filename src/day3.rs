use std::fs;

#[allow(dead_code)]
pub fn part1() -> (i64, u128) {
    let filename = "data/day3.txt";
    let source = fs::read_to_string(filename).expect("File not found");
    let now = std::time::Instant::now();
    let mut count = 0;
    
    for line in source.split("\n") {
        let sides = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let a = *sides.get(0).unwrap();
        let b = *sides.get(1).unwrap();
        let c = *sides.get(2).unwrap();
        count += if a + b > c && a + c > b && b + c > a {
            1
        } else {
            0
        };
    }
    return (count, now.elapsed().as_micros());
}

fn count_valid(triangle_stream: Vec<i64>) -> i64 {
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    let mut count: i64 = 0;
    let mut clock = 0;

    for c in triangle_stream.into_iter() {
        match clock {
            0 => {
                a = c
            },
            1 => {
                b = c
            },
            _ => {
                count += if a + b > c && a + c > b && b + c > a {
                    1
                } else {
                    0
                };
            }
        }
        clock = (clock + 1) % 3;
    }
    return count;
}

#[allow(dead_code)]
pub fn part2() -> (i64, u128) {
    let filename = "data/day3.txt";
    let source = fs::read_to_string(filename).expect("File not found");
    let now = std::time::Instant::now();
  
    let mut a: Vec<i64> = Vec::new();
    let mut b: Vec<i64> = Vec::new();
    let mut c: Vec<i64> = Vec::new();

    for line in source.split("\n") {
        let sides = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        a.push(*sides.get(0).unwrap());
        b.push(*sides.get(1).unwrap());
        c.push(*sides.get(2).unwrap());
    }
    return (count_valid(a) + count_valid(b) + count_valid(c), now.elapsed().as_micros());
}