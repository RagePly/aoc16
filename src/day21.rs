#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
enum Operation {
    SwapPosition(usize, usize),
    SwapLetters(u8, u8),
    RotateSteps(Direction, usize),
    RotateByIndexOf(u8),
    Reverse(usize, usize),
    Move(usize, usize)
}

fn parse_line(line: &str) -> Operation {
    use Operation::*;
    use Direction::*;
    let tokens: Vec<_> = line.split_ascii_whitespace().collect();
    match tokens[0] {
        "swap" => match tokens[1] {
            "position" => {
                let x: usize = tokens[2].parse().unwrap();
                let y: usize = tokens[5].parse().unwrap();
                SwapPosition(x, y)
            },
            _ => {
                let x = tokens[2].as_bytes();
                let y = tokens[5].as_bytes();
                SwapLetters(x[0], y[0])
            }
        },
        "rotate" => match tokens[1] {
            "based" => RotateByIndexOf(
                tokens[6].as_bytes()[0]
            ),
            dir => {
                let steps = tokens[2].parse::<usize>().unwrap();
                match dir {
                    "left" => RotateSteps(Left, steps),
                    _ => RotateSteps(Right, steps)
                }
            }
        }
        "reverse" => {
            let x: usize = tokens[2].parse().unwrap();
            let y: usize = tokens[4].parse().unwrap();
            Reverse(x, y)
        },
        "move" => {
            let x: usize = tokens[2].parse().unwrap();
            let y: usize = tokens[5].parse().unwrap();
            Move(x, y)
        }
        s => panic!("invalid operation \"{}\"", s)
    }
}

fn apply_operation(s: &mut Vec<u8>, operation: Operation) {
    use Operation::*;
    use Direction::*;
    match operation {
        SwapPosition(x, y) => {
            let z = s[x];
            s[x] = s[y];
            s[y] = z;
        },
        SwapLetters(a, b) => {
            let x = s.iter().position(|c| *c == a).unwrap();
            let y = s.iter().position(|c| *c == b).unwrap();
            s[x] = b;
            s[y] = a;
        },
        RotateSteps(direction, amount) => {
            let temp = s.clone();
            let len = s.len();
            match direction {
                Left => temp.into_iter().enumerate().for_each(
                    |(i, b)| s[(amount*len + i - amount) % len] = b
                ),
                Right => temp.into_iter().enumerate().for_each(
                    |(i, b)| s[(amount*len + i + amount) % len] = b
                )
            };
        },
        RotateByIndexOf(c) => {
            let i = s.iter().position(|b| *b == c).unwrap();
            let amount = 1 + i + if i >= 4 { 1 } else { 0 }; 
            let temp = s.clone();
            let len = s.len();
            temp.into_iter().enumerate().for_each(
                |(i, b)| s[(amount*len + i + amount) % len] = b
            );
        },
        Reverse(x, y) => {
            let temp = s.clone();
            temp.into_iter().skip(x).take(y - x + 1).enumerate().for_each(
                |(i, b)| s[y - i] = b 
            );
        },
        Move(x, y) => {
            if x < y {
                let c = s[x];
                for i in x..y {
                    s[i] = s[i+1];
                }
                s[y] = c;
            } else {
                let c = s[x];
                for i in 0..x-y {
                    s[x-i] = s[x-i-1];
                }
                s[y] = c;
            }
        }
    }
}

pub fn part1(source: String) -> String {
    let mut s: Vec<u8> = "abcdefgh".as_bytes().into_iter().map(|b| *b).collect();
    source.split("\r\n").map(|line| parse_line(line)).for_each(
        |op| apply_operation(&mut s, op)
    );
    s.into_iter().map(|b| b as char).collect()
}

fn apply_operation_rev(s: &mut Vec<u8>, operation: Operation) {
    use Operation::*;
    use Direction::*;
    match operation {
        SwapPosition(y, x) => {
            let z = s[x];
            s[x] = s[y];
            s[y] = z;
        },
        SwapLetters(b, a) => {
            let x = s.iter().position(|c| *c == a).unwrap();
            let y = s.iter().position(|c| *c == b).unwrap();
            s[x] = b;
            s[y] = a;
        },
        RotateSteps(direction, amount) => {
            let temp = s.clone();
            let len = s.len();
            match direction {
                Left => temp.into_iter().enumerate().for_each(
                    |(i, b)| s[(amount*len + i + amount) % len] = b
                ),
                Right => temp.into_iter().enumerate().for_each(
                    |(i, b)| s[(amount*len + i - amount) % len] = b
                )
            };
        },
        RotateByIndexOf(c) => {
            let i = s.iter().position(|b| *b == c).unwrap();
            let amount = if i % 2 == 0 { 
                if i == 0 {
                    9
                } else {
                    i / 2 + 5 
                }
            } else {
                i / 2 + 1
            };
            let temp = s.clone();
            let len = s.len();
            temp.into_iter().enumerate().for_each(
                |(i, b)| s[(amount*len + i - amount) % len] = b
            );
        },
        Reverse(x, y) => {
            let temp = s.clone();
            temp.into_iter().skip(x).take(y - x + 1).enumerate().for_each(
                |(i, b)| s[y - i] = b 
            );
        },
        Move(y, x) => {
            if x < y {
                let c = s[x];
                for i in x..y {
                    s[i] = s[i+1];
                }
                s[y] = c;
            } else {
                let c = s[x];
                for i in 0..x-y {
                    s[x-i] = s[x-i-1];
                }
                s[y] = c;
            }
        }
    }
}

pub fn part2(source: String) -> String {
    let mut s: Vec<u8> = "fbgdceah".as_bytes().into_iter().map(|b| *b).collect();
    let lines: Vec<_> = source.split("\r\n").map(|line| parse_line(line)).collect();
    lines.into_iter().rev().for_each(
        |op| apply_operation_rev(&mut s, op)
    );
    s.into_iter().map(|b| b as char).collect()
}