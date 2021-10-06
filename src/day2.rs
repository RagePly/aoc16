use std::fs;

#[allow(dead_code)]
pub fn part1() -> (i64, u128) {
    let filename = "data/day2.txt";
    let source = fs::read_to_string(filename).expect("file not found");
    let now = std::time::Instant::now();
    let number_paths: Vec<&str> = source.split("\n").collect();
    let mut x: i64 = 1;
    let mut y: i64 = 1;
    let mut exp: i64 = (10i64).pow(number_paths.len() as u32 - 1); 
    let mut code: i64 = 0;
    for path in number_paths {
        for direction in path.chars() {
            match direction {
                'U' => {
                    y = num::clamp(y - 1, 0, 2);
                },
                'D' => {
                    y = num::clamp(y + 1, 0, 2);
                },
                'R' => {
                    x = num::clamp(x + 1, 0, 2);
                },
                'L' => {
                    x = num::clamp(x - 1, 0, 2);
                },
                _ => () // probably catches the \r if \r\n
            }
        };
        code += (y * 3 + x + 1)*exp;
        exp /= 10;
    }; 

    return (code, now.elapsed().as_micros());
}

#[allow(dead_code)]
pub fn part2() -> (String, u128) {
    let filename = "data/day2.txt";
    let source = fs::read_to_string(filename).expect("file not found");
    let now = std::time::Instant::now();
    let code_array: [char; 13] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D'];
    let number_paths: Vec<&str> = source.split("\n").collect();
    let mut x: i64 = 0;
    let mut y: i64 = 2;
    let mut code: Vec<char> = Vec::new();
    for path in number_paths {
        for direction in path.chars() {
            let y_clamp = 2 - (x - 2).abs();
            let x_clamp = 2 - (y - 2).abs();
            match direction {
                'U' => {
                    y = num::clamp(y - 1, 2 - y_clamp, 2 + y_clamp);
                },
                'D' => {
                    y = num::clamp(y + 1, 2 - y_clamp, 2 + y_clamp);
                },
                'R' => {
                    x = num::clamp(x + 1, 2 - x_clamp, 2 + x_clamp);
                },
                'L' => {
                    x = num::clamp(x - 1, 2 - x_clamp, 2 + x_clamp);
                },
                _ => () // probably catches the \r if \r\n
            }
        };
        let index = 6 + (2 + 2 * (2 - y).abs()) * (- num::signum(2 - y)) - (2 - x);
        code.push(*code_array.get(index as usize).expect("Invalid index"));
    }; 

    return (code.into_iter().collect(), now.elapsed().as_micros());
}