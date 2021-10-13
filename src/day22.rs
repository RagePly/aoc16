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

pub fn part2(source: String) -> usize {
    use std::cmp::max;
    let nodes: Vec<_> = source.split("\r\n").skip(2).map(
        |line| parse_line(line)
    ).collect();

    let (max_x, max_y) = nodes.iter().scan((0,0), |state, node| 
        Some((max(state.0, node.point.x), max(state.1, node.point.y)))
    ).last().unwrap();

    let mut map: Vec<Vec<char>> = Vec::with_capacity((max_y + 1) as usize);
    for _ in 0..max_y + 1 {
        map.push(Vec::with_capacity((max_x + 1) as usize));
        for _ in 0..max_x + 1 {
            map.last_mut().unwrap().push('.');
        }
    }

    nodes.iter().for_each(
        |node| map[node.point.y as usize][node.point.x as usize] = if node.avail == 0 {
            '_'
        } else if node.point.x == 0 && node.point.y == 0 {
            'O'
        } else {
            '#'
        }
    );

    for y in 0..max_y {
        for x in 0..max_x {
            print!("{} ", map[y as usize][x as usize]);
        }
        println!();
    }

    0
}