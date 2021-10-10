use std::collections::VecDeque;

fn is_open(x: i32, y: i32, n: i32) -> bool {
    let v = x*x + 3*x + 2 * x * y + y + y*y + n;
    v.count_ones() % 2 == 0
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32
}

struct Path {
    p: Point,
    dist: i32
}

fn path(x: i32, y: i32, d: i32) -> Path {
    Path {
        p: point(x, y),
        dist: d
    }
}

fn point(x: i32, y: i32) -> Point {
    Point {
        x: x,
        y: y
    }
}

fn dist_to(from: &Point, to: &Point) -> i32 {
    (to.x - from.x).abs() + (to.y - from.y).abs()
}


pub fn part1(source: String) -> i32 {
    let favorite_number: i32 = source.parse().unwrap();
    let goal = point(31, 39);

    let directions = [point(1,0), point(0, 1), point(-1, 0), point(0, -1)];

    let mut queue: VecDeque<Path> = VecDeque::new();
    let mut visited: Vec<Point> = Vec::new();
    queue.push_back(path(1,1,0));

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current.p == goal {
            return current.dist;
        } else if visited.contains(&current.p){
            continue
        } else {
            let x = current.p.x;
            let y = current.p.y;
            let d = current.dist + 1;
            visited.push(current.p);

            for p in directions.iter() {
                let new_x = p.x + x;
                let new_y = p.y + y;
                if new_x >= 0 && new_y >= 0 && is_open(new_x, new_y, favorite_number) {
                    queue.push_back(path(new_x, new_y, d));
                }
            }
        }

        queue.make_contiguous().sort_unstable_by(|a, b| dist_to(&a.p, &goal).cmp(&dist_to(&b.p, &goal)));
    }
    -1
}
pub fn part2(source: String) -> usize {
    let favorite_number: i32 = source.parse().unwrap();

    let directions = [point(1,0), point(0, 1), point(-1, 0), point(0, -1)];

    let mut queue: VecDeque<Path> = VecDeque::new();
    let mut visited: Vec<Point> = Vec::new();
    queue.push_back(path(1,1,0));

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current.dist > 50 {
            continue;
        } else if visited.contains(&current.p){
            continue
        } else {
            let x = current.p.x;
            let y = current.p.y;
            let d = current.dist + 1;
            visited.push(current.p);

            for p in directions.iter() {
                let new_x = p.x + x;
                let new_y = p.y + y;
                if new_x >= 0 && new_y >= 0 && is_open(new_x, new_y, favorite_number) {
                    queue.push_back(path(new_x, new_y, d));
                }
            }
        }
    }
    visited.len()
}