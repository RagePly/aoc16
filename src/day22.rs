use std::collections::VecDeque;
use itertools::Itertools;
use std::cmp::max;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}
fn point(x: i32, y: i32) -> Point {
    Point{
        x: x,
        y: y
    }
}

#[derive(Debug, Clone)]
struct Path {
    p: Point,
    prev: Vec<Point>,
    dist: i32
}

fn dist_to(p1: &Point, p2: &Point) -> i32 {
    (p2.x - p1.x).abs() + (p2.y - p1.y).abs()
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


enum FileType {
    Compatible,
    Empty,
    NonCompatible
}

struct File {
    file_t: FileType,
    position: Point
}


type Map = Vec<Vec<bool>>;

fn find_path(from: Point, goal: Point, map: &Map) -> (i32, i32) {
    let directions = [point(1,0), point(0, 1), point(-1, 0), point(0, -1)];

    let mut queue: VecDeque<Path> = VecDeque::new();
    let mut visited: Vec<Point> = Vec::new();
    queue.push_back(Path {
        p: from,
        prev: Vec::new(),
        dist: 0
    });

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current.p == goal {
            let mut x_steps = 0;
            let mut y_steps = 0;
            let mut prev = current.prev.clone();
            prev.push(current.p);
            prev.as_slice().windows(2).for_each(
                |w| {
                    x_steps += (w[0].x - w[1].x).abs();
                    y_steps += (w[0].y - w[1].y).abs();
                }
            );
            return (x_steps, y_steps);
        } else if visited.contains(&current.p){
            continue
        } else {
            let x = current.p.x;
            let y = current.p.y;
            let d = current.dist + 1;
            let mut prev = current.prev.clone();
            prev.push(current.p);
            visited.push(current.p);

            for p in directions.iter() {
                let new_x = p.x + x;
                let new_y = p.y + y;
                if new_x >= 0 && new_y >= 0 && 
                    new_x < map[0].len() as i32 && 
                    new_y < map.len() as i32 &&
                    map[new_y as usize][new_x as usize] { // because of the border, this shouldn't throw errors
                    queue.push_back(Path {
                        p: point(new_x, new_y),
                        prev: prev.clone(),
                        dist: d
                    });
                }
            }
        }

        queue.make_contiguous().sort_unstable_by(|a, b| (dist_to(&a.p, &goal) + a.dist).cmp(&(dist_to(&b.p, &goal) + b.dist)));
    }
    panic!("no path found")
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

pub fn part2(source: String) -> i32 {
    let mut files: Vec<File> = Vec::new();
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    source.split("\r\n").skip(2).map(
        |line| parse_line(line)
    ).combinations(2).for_each(
        |nodes| {
            let a = &nodes[0];
            let b = &nodes[1];
            max_x = max(max(a.point.x, b.point.x), max_x);
            max_y = max(max(a.point.y, b.point.y), max_y);

            files.push(File {
                file_t: if a.used > 0 && b.avail >= a.used {
                    FileType::Compatible
                } else if a.used == 0 {
                    FileType::Empty
                } else {
                    FileType::NonCompatible
                },
                position: a.point
            });
            files.push(File {
                file_t: if b.used > 0 && a.avail >= b.used {
                    FileType::Compatible
                } else if b.used == 0 {
                    FileType::Empty
                } else {
                    FileType::NonCompatible
                },
                position: b.point
            });
        }
    );

    let mut map: Vec<Vec<bool>> = (0..max_y + 1).map(|_| (0..max_x + 1).map(|_| false).collect()).collect();
    let mut empty_point: Point = Point { x: 0, y: 0};
    files.iter().for_each(
        |file| map[file.position.y as usize][file.position.x as usize] = match file.file_t {
            FileType::Compatible => true,
            FileType::Empty => {
                empty_point.x = file.position.x;
                empty_point.y = file.position.y;
                true
            },
            _ => map[file.position.y as usize][file.position.x as usize]
        }
    );

    let (x_1, y_1) = find_path(empty_point, point(max_x, 0), &map);
    let (x_2, y_2) = find_path(point(max_x - 1, 0), point(0, 0), &map);
    x_1 + y_1 + y_2 + 5*x_2    
}