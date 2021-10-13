use std::collections::VecDeque;
use itertools::Itertools;

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

#[derive(Debug, Clone, Copy)]
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

fn dist_to(p1: &Point, p2: &Point) -> i32 {
    (p2.x - p1.x).abs() + (p2.y - p1.y).abs()
}

// numbers are pass-through
type Map = Vec<Vec<bool>>;

fn find_path(from: Point, goal: Point, map: &Map) -> i32 {
    let directions = [point(1,0), point(0, 1), point(-1, 0), point(0, -1)];

    let mut queue: VecDeque<Path> = VecDeque::new();
    let mut visited: Vec<Point> = Vec::new();
    queue.push_back(Path {
        p: from,
        dist: 0
    });

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
                if map[new_y as usize][new_x as usize] { // because of the border, this shouldn't throw errors
                    queue.push_back(path(new_x, new_y, d));
                }
            }
        }

        queue.make_contiguous().sort_unstable_by(|a, b| (dist_to(&a.p, &goal) + a.dist).cmp(&(dist_to(&b.p, &goal) + b.dist)));
    }
    panic!("no path found")
}

fn parse_row(line: &str) -> Vec<bool> {
    line.as_bytes().iter().map(|b| *b != '#' as u8).collect()
}

type Location = (usize, Point);

fn get_points(line: &str, current_row: usize) -> Vec<Location>{
    let mut pts: Vec<Location> = Vec::new();
    line.as_bytes().iter().enumerate().for_each(
        |(i, b)| match (*b as char).to_digit(10) {
            Some(n) => pts.push((n as usize, point(i as i32, current_row as i32))),
            None => ()
        }
    );
    pts
}


type Node = Vec<i32>;
type Graph = Vec<Node>;

fn get_distance_between(a: usize, b: usize, graph: &Graph) -> i32 {
    graph[a][b]
}


pub fn part1(source: String) -> i32 {
    let mut all_points: Vec<Location> = Vec::new();
    let map: Map = source.split("\r\n").enumerate().map(
        |(i, line)| {
            all_points.append(&mut get_points(line, i));
            parse_row(line)
        }
    ).collect();

    let mut graph: Graph = Graph::with_capacity(all_points.len());
    for _ in 0..all_points.len() {
        let mut node: Node = Node::with_capacity(all_points.len());
        for _ in 0..all_points.len() {
            node.push(0);
        }
        graph.push(node);
    }

    all_points.into_iter().combinations(2).for_each(
        |pair| {
            let dist = find_path(pair[0].1, pair[1].1, &map);
            graph[pair[0].0 as usize][pair[1].0 as usize] = dist;
            graph[pair[1].0 as usize][pair[0].0 as usize] = dist;
        }
    );

    (0..graph.len()).permutations(graph.len()).filter(|path| path[0] == 0).map(
        |path| path.into_iter().fold((0i32, None), |(l, prev_p), p| match prev_p {
                Some(pp) => (l + get_distance_between(pp, p, &graph), Some(p)),
                None => (l, Some(p))
            }).0
    ).min().unwrap()
}

pub fn part2(source: String) -> i32 {
    let mut all_points: Vec<Location> = Vec::new();
    let map: Map = source.split("\r\n").enumerate().map(
        |(i, line)| {
            all_points.append(&mut get_points(line, i));
            parse_row(line)
        }
    ).collect();

    let mut graph: Graph = Graph::with_capacity(all_points.len());
    for _ in 0..all_points.len() {
        let mut node: Node = Node::with_capacity(all_points.len());
        for _ in 0..all_points.len() {
            node.push(0);
        }
        graph.push(node);
    }

    all_points.into_iter().combinations(2).for_each(
        |pair| {
            let dist = find_path(pair[0].1, pair[1].1, &map);
            graph[pair[0].0 as usize][pair[1].0 as usize] = dist;
            graph[pair[1].0 as usize][pair[0].0 as usize] = dist;
        }
    );

    (0..graph.len()).permutations(graph.len()).filter(|path| path[0] == 0).map(
        |path| path.into_iter().chain(vec!(0)).fold((0i32, None), |(l, prev_p), p| match prev_p {
                Some(pp) => (l + get_distance_between(pp, p, &graph), Some(p)),
                None => (l, Some(p))
            }).0
    ).min().unwrap() // ! can be done faster if you use a queue to generate new points
}