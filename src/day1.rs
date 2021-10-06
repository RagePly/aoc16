use std::fs;
use std::time::Instant;

#[allow(dead_code)]
pub fn part1() -> (i32, u128) {
    let filename = "data/day1.txt";
    let contents = fs::read_to_string(filename).expect("File not found!");
    let now = Instant::now();
    let paths = contents.split(", ");
    let mut rotation: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for directive in paths {
        let cs: Vec<char> = directive.chars().collect();
        let dir = cs[0];
        let amount: i32 = cs[1..].iter().collect::<String>().parse().expect("Not a number apparently...");
        rotation = match dir {
            'L' => (rotation + 3) % 4,
            'R' => (rotation + 1) % 4,
            _ => {
                println!("Invalid character, but I'll keep going...");
                rotation
            } 
        };
        x += amount * (rotation % 2) * (1 - (rotation / 2) * 2);
        y += amount * ((1+rotation) % 2) * (1 - (rotation / 2) * 2);
    }
    let duration = now.elapsed().as_micros();
    return (x.abs() + y.abs(), duration);
}

#[allow(dead_code)]
pub fn part2() -> (i32, u128) {
    let filename = "data/day1.txt";
    let contents = fs::read_to_string(filename).expect("File not found!");
    let now = Instant::now();
    let paths = contents.split(", ");
    let mut rotation: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut visited_positions: Vec<(i32, i32)> = Vec::new();
    visited_positions.push((0,0));

    for directive in paths {
        let cs: Vec<char> = directive.chars().collect();
        let dir = cs[0];
        let amount: i32 = cs[1..].iter().collect::<String>().parse().expect("Not a number apparently...");

        rotation = match dir {
            'L' => (rotation + 3) % 4,
            'R' => (rotation + 1) % 4,
            _ => {
                println!("Invalid character, but I'll keep going...");
                rotation
            } 
        };

        let direction_sign = 1 - (rotation / 2) * 2;
        let dx_increment = direction_sign * (rotation % 2);
        let dy_increment = direction_sign * ((1 + rotation) % 2);

        for i_x in 0..(if dx_increment == 0 { 1 } else { amount }) {
            for i_y in 0..(if dy_increment == 0 { 1 } else { amount }) 
            {
                let pos_x = x + (i_x + 1) * dx_increment;
                let pos_y = y + (i_y + 1) * dy_increment;

                if visited_positions.contains(&(pos_x, pos_y)) {
                    let duration = now.elapsed().as_micros();
                    return (pos_x.abs() + pos_y.abs(), duration);
                } else {
                    visited_positions.push((pos_x, pos_y));
                }
            }
        }
        x += amount * dx_increment;
        y += amount * dy_increment;
    }
    let duration = now.elapsed().as_micros();
    return (-1, duration);
}