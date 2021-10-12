use std::collections::VecDeque;
use md5::compute;

struct Point {
    x: i32,
    y: i32
}

impl Point {

    fn origin() -> Point {
        Point {
            x: 0,
            y: 0
        }
    }

    fn is_valid(&self) -> bool {
        0 <= self.x && self.x < 4 && 0 <= self.y && self.y < 4
    }

    fn has_reached_vault(&self) -> bool {
        self.x == 3 && self.y == 3
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct Path {
    passcode: String,
    position: Point
}

type PathQueue = VecDeque<Path>;

const DOORS: [&str; 4] = ["U", "D", "L", "R"];
const DIRECTIONS: [Point; 4] = [ 
    Point { x:  0, y: -1 }, // up
    Point { x:  0, y:  1 }, // down 
    Point { x: -1, y:  0 }, // left 
    Point { x:  1, y:  0 }  // right
    ];

pub fn part1(source: String) -> String {
    let mut path_queue: PathQueue = PathQueue::new();
    path_queue.push_back(Path {
        passcode: source.clone(),
        position: Point::origin() 
    });

    loop {
        let current_path = path_queue.pop_front().expect("can't reach end");
        if current_path.position.has_reached_vault() {
            return current_path.passcode.replace(source.as_str(), "")
        }  

        let doors: Vec<_> = format!("{:x?}", compute(current_path.passcode.clone())).as_bytes().iter().take(4).map(
            |b| {
                if ('0' as u8 <= *b && *b <= '9' as u8) || *b == 'a' as u8 {
                    false
                } else {
                    true
                }
            }
        ).collect();

        for (i, door) in doors.into_iter().enumerate() {
            if door {
                let new_point = current_path.position.add(&DIRECTIONS[i]);
                if new_point.is_valid() {
                    let new_passcode = current_path.passcode.clone() + DOORS[i];
                    path_queue.push_back(
                        Path {
                            passcode: new_passcode,
                            position: new_point
                        }
                    );
                }
            }
        }
    }
}

pub fn part2(source: String) -> usize {
    let mut path_queue: PathQueue = PathQueue::new();
    path_queue.push_back(Path {
        passcode: source.clone(),
        position: Point::origin() 
    });
    let mut longest_path: usize = 0;

    loop {
        let current_path = match path_queue.pop_front() {
            Some(path) => path,
            None => return longest_path
        };

        if current_path.position.has_reached_vault() {
            longest_path = current_path.passcode.len() - source.len();
            continue // paths only pass through the vault once
        }  

        let doors: Vec<_> = format!("{:x?}", compute(current_path.passcode.clone())).as_bytes().iter().take(4).map(
            |b| {
                if ('0' as u8 <= *b && *b <= '9' as u8) || *b == 'a' as u8 {
                    false
                } else {
                    true
                }
            }
        ).collect();

        for (i, door) in doors.into_iter().enumerate() {
            if door {
                let new_point = current_path.position.add(&DIRECTIONS[i]);
                if new_point.is_valid() {
                    let new_passcode = current_path.passcode.clone() + DOORS[i];
                    path_queue.push_back(
                        Path {
                            passcode: new_passcode,
                            position: new_point
                        }
                    );
                }
            }
        }
    }
}