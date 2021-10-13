use itertools::Itertools;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Clone)]
struct Node {
    point: Point,
    size: i32,
    avail: i32,
    used: i32
}


fn parse_line(line: &str) -> Node {
    let tokens: Vec<_> = line.split_ascii_whitespace().collect();
    let position_temp: Vec<i32> = tokens[0].split("-").skip(1).map(
        |token| token.as_bytes().iter().skip(1).map(|b| *b as char).collect::<String>().parse().unwrap()
    ).collect();

    let storage: Vec<i32> = tokens.into_iter().skip(1).take(3).map(
        |token| {
            let size = token.len();
            token.as_bytes().iter().take(size - 1).map(|b| *b as char).collect::<String>().parse().unwrap()
        }
    ).collect();
    Node {
        point: Point {
            x: position_temp[0],
            y: position_temp[1]
        },
        size: storage[0],
        used: storage[1],
        avail: storage[2]
    }
}

pub fn part1(source: String) -> usize {
    source.split("\r\n").skip(2).map(
        |line| parse_line(line)
    ).combinations(2).map(
        |nodes| {
            let a = &nodes[0];
            let b = &nodes[1];
            let mut viable = 0;
            if a.used > 0 && b.avail >= a.used {
                viable += 1;
            }
            if b.used > 0 && a.avail >= b.used {
                viable += 1;
            }
            viable
        }
    ).sum()
}
