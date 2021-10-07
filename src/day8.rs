#[derive(Debug)]
enum Instruction {
    Rect(i32, i32),
    RotRow(i32, i32),
    RotCol(i32, i32)
}

fn parse_instruction(line: String) -> Instruction {
    let kw: Vec<_> = line.split_ascii_whitespace().collect();
    match kw[0] {
        "rect" => {
            let args: Vec<_> = kw[1].split("x").collect();
            let a: i32 = String::from(args[0]).parse().unwrap();
            let b: i32 = String::from(args[1]).parse().unwrap();
            Instruction::Rect(a,b)
        },
        "rotate" => {
            let tmp: Vec<_> = kw[2].split("=").collect();
            let a: i32 = String::from(tmp[1]).parse().unwrap();
            let b: i32 = String::from(kw[4]).parse().unwrap();
            match kw[1] {
                "row" => {
                    Instruction::RotRow(a,b)
                },
                "column" => {
                    Instruction::RotCol(a,b)
                },
                _ => panic!("Unknown rotation directive")
            }
        },
        _ => panic!("Unknown instruction")
    }
}

fn fill_rect(board: &mut Board, width: i32, height: i32) {
    for y in 0..height as usize {
        for x in 0..width as usize {
            board[y][x] = true;
        }
    }
}

fn rotate_row(board: &mut Board, row: i32, rotations: i32) {
    let mut temp_row: [bool; WIDTH] = [false; WIDTH];

    for x in 0..WIDTH {
        temp_row[(x + (rotations as usize)) % WIDTH] = board[row as usize][x];
    }
    for x in 0..WIDTH {
        board[row as usize][x] = temp_row[x];
    }
}

fn rotate_column(board: &mut Board, column: i32, rotations: i32) {
    let mut temp_column: [bool; HEIGHT] = [false; HEIGHT];
    for y in 0..HEIGHT {
        temp_column[(y + (rotations as usize)) % HEIGHT] = board[y][column as usize];
    }
    for y in 0..HEIGHT {
        board[y][column as usize] = temp_column[y];
    }
}

type Board = Vec<Vec<bool>>;
const WIDTH: usize = 50;
const HEIGHT: usize = 6;
#[allow(dead_code)]
pub fn part1(source: String) -> i32 {
    // init board
    let mut board: Board = Vec::new();
    for y in 0..HEIGHT {
        board.push(Vec::new());
        for _ in 0..WIDTH {
            board[y].push(false);
        }
    }

    for line in source.split("\r\n") {
        match parse_instruction(String::from(line)) {
            Instruction::Rect(w, h) => fill_rect(&mut board, w, h),
            Instruction::RotCol(c, r) => rotate_column(&mut board, c, r),
            Instruction::RotRow(r, rot) => rotate_row(&mut board, r, rot) 
        }
    }

    let mut count: i32 = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            count += if board[y][x] {1} else {0};
        }
    }
    count
}

#[allow(dead_code)]
pub fn part2(source: String) -> String {
    // init board
    let mut board: Board  = Vec::new();
    for y in 0..HEIGHT {
        board.push(Vec::new());
        for _ in 0..WIDTH {
            board[y].push(false);
        }
    }

    for line in source.split("\r\n") {
        match parse_instruction(String::from(line)) {
            Instruction::Rect(w, h) => fill_rect(&mut board, w, h),
            Instruction::RotCol(c, r) => rotate_column(&mut board, c, r),
            Instruction::RotRow(r, rot) => rotate_row(&mut board, r, rot) 
        }
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", if board[y][x] {
                " #"
            } else {
                "  "
            }
            );
        }
        print!("\n");
    }
    String::from("^ READ ABOVE ^")
}