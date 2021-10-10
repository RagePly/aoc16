#[derive(Debug, Clone, Copy)]
enum Instruction {
    CopyValue(i32, usize),
    CopyRegister(usize, usize),
    Increment(usize),
    Decrement(usize),
    JumpRegister(usize, i32),
    JumpValue(i32, i32)
}

fn parse_register(token: &str) -> usize {
    match token {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Invalid register")
    }
}

type Program = Vec<Instruction>;

struct ProgramState {
    ip: i32,
    registers: [i32; 4],
    program: Program
}

fn parse_line(line: &str) -> Instruction {
    use Instruction::*;
    let tokens: Vec<&str> = line.split_ascii_whitespace().collect(); 
    match tokens[0] {
        "cpy" => {
            match tokens[1].parse::<i32>() {
                Ok(val) => CopyValue(val, parse_register(tokens[2])),
                Err(_) => CopyRegister(parse_register(tokens[1]), parse_register(tokens[2]))
            }
        },
        "inc" => Increment(parse_register(tokens[1])),
        "dec" => Decrement(parse_register(tokens[1])),
        "jnz" => {
            match tokens[1].parse::<i32>() {
                Ok(val) => JumpValue(val, tokens[2].parse::<i32>().unwrap()),
                Err(_) => JumpRegister(parse_register(tokens[1]), tokens[2].parse::<i32>().unwrap())
            }
        },
        invalid_instruction => panic!("Invalid instruction {}", invalid_instruction)
    }    
}

fn step_program(state: &mut ProgramState) -> bool {
    use Instruction::*;
    if state.ip >= state.program.len() as i32 {
        return false
    } else if state.ip < 0 {
        panic!("negative instruction pointer {}", state.ip);
    }

    let current_instruction = state.program[state.ip as usize];
    match current_instruction {
        CopyRegister(from_reg, to_reg) => state.registers[to_reg] = state.registers[from_reg],
        CopyValue(val, reg) => state.registers[reg] = val,
        Increment(reg) => state.registers[reg] += 1,
        Decrement(reg) => state.registers[reg] -= 1,
        JumpRegister(case_register, val) => if state.registers[case_register] != 0 { state.ip += val - 1; }
        JumpValue(case_value, val) => if case_value != 0 { state.ip += val - 1 }
    };
    state.ip += 1;
    true
}

pub fn part1(source: String) -> i32 {
    let program: Program = source.split("\r\n").map(|line| parse_line(line)).collect();
    let mut program_state = ProgramState {
        ip: 0,
        registers: [0; 4],
        program: program
    };


    while step_program(&mut program_state) {};
    program_state.registers[0]
}

pub fn part2(source: String) -> i32 {
    let program: Program = source.split("\r\n").map(|line| parse_line(line)).collect();
    let mut program_state = ProgramState {
        ip: 0,
        registers: [0, 0, 1, 0],
        program: program
    };


    while step_program(&mut program_state) {};
    program_state.registers[0]
}