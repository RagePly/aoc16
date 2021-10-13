#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InstructionType {
    Copy,
    Increment,
    Decrement,
    Jump,
    Toggle,
    Output
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueType {
    Register(usize),
    Value(i32)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Argument {
    One(ValueType),
    Two(ValueType, ValueType)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    op: InstructionType,
    arguments: Argument
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

#[derive(Clone, PartialEq, Eq)]
struct ProgramState {
    ip: i32,
    registers: [i32; 4],
    program: Program
}

fn parse_line(line: &str) -> Instruction {
    use ValueType::*;
    use Argument::*;
    use InstructionType::*;
    let tokens: Vec<&str> = line.split_ascii_whitespace().collect(); 
    match tokens[0] {
        "cpy" => {
            match tokens[1].parse::<i32>() {
                Ok(val) => Instruction {
                    op: Copy,
                    arguments: Two(
                        Value(val), 
                        Register(parse_register(tokens[2]))
                    )
                },
                Err(_) => Instruction {
                    op: Copy,
                    arguments: Two(
                        Register(parse_register(tokens[1])), 
                        Register(parse_register(tokens[2]))
                    )
                }
            }
        },
        "inc" => Instruction {
            op: Increment,
            arguments: One(Register(parse_register(tokens[1])))
        },
        "dec" => Instruction {
            op: Decrement,
            arguments: One(Register(parse_register(tokens[1])))    
        },
        "jnz" => Instruction {
            op: Jump,
            arguments: Two(
                match tokens[1].parse::<i32>() {
                    Ok(val) => Value(val),
                    Err(_) => Register(parse_register(tokens[1]))                
                },
                match tokens[2].parse::<i32>() {
                    Ok(val) => Value(val),
                    Err(_) => Register(parse_register(tokens[2]))                
                }
            )
        },
        "tgl" => Instruction {
            op: Toggle,
            arguments: One(Register(parse_register(tokens[1])))
        },
        "out" => Instruction{
            op: Output,
            arguments: One(
                match tokens[1].parse::<i32>() {
                    Ok(v) => Value(v),
                    Err(_) => Register(parse_register(tokens[1]))
                }
            )
        },
        invalid_instruction => panic!("Invalid instruction {}", invalid_instruction)
    }    
}

fn step_program(state: &mut ProgramState) -> (bool, Option<i32>) {
    use Argument::*;
    use ValueType::*;
    use InstructionType::*;

    if state.ip >= state.program.len() as i32 {
        return (false, None)
    } else if state.ip < 0 {
        panic!("negative instruction pointer {}", state.ip);
    }

    let mut output: Option<i32> = None;
    let current_instruction = state.program[state.ip as usize];
    match current_instruction.op {
        Copy => match current_instruction.arguments {
            Two(Register(from_reg), Register(to_reg)) => state.registers[to_reg] = state.registers[from_reg],
            Two(Value(val), Register(reg)) => state.registers[reg] = val,
            _ => ()
        },
        Increment => match current_instruction.arguments {
            One(Register(reg)) => state.registers[reg] += 1,
            _ => ()
        },
        Decrement => match current_instruction.arguments {
            One(Register(reg)) => state.registers[reg] -= 1,
            _ => ()
        },
        Jump => {
            let case = match current_instruction.arguments {
                Two(Register(case_register), _) => state.registers[case_register] != 0,
                Two(Value(case_value), _) => case_value != 0 ,
                _ => false 
            };
            if case {
                match current_instruction.arguments {
                    Two(_, Register(reg)) => state.ip += state.registers[reg] - 1,
                    Two(_, Value(val)) => state.ip += val - 1,
                    _ => ()

                }
            }
        },
        Toggle => match current_instruction.arguments {
            One(Register(reg)) => {
                let i = state.ip + state.registers[reg];
                if 0 <= i && (i as usize) < state.program.len() {
                    let previous_instruction = state.program[i as usize];
                    let new_instruction = match previous_instruction.arguments {
                        One(arg) => Instruction {
                            op: match previous_instruction.op {
                                Increment => Decrement,
                                _ => Increment
                            },
                            arguments: One(arg)
                        },
                        Two(arg1, arg2) => Instruction {
                            op: match previous_instruction.op {
                                Jump => Copy,
                                _ => Jump
                            },
                            arguments: Two(arg1, arg2)
                        }
                    };
                    state.program[i as usize] = new_instruction;
                }
            },
            _ => ()
        },
        Output => match current_instruction.arguments {
            One(Register(r)) => output = Some(state.registers[r]),
            One(Value(v)) => output = Some(v),
            _ => ()
        }
    }
    state.ip += 1;
    (true, output)
}

pub fn part1(source: String) -> i32 {
    let program: Program = source.split("\r\n").map(|line| parse_line(line)).collect();
    for i in 0i32..100_100i32 {
        let mut past_one_states: Vec<ProgramState> = Vec::new();
        let mut program_state = ProgramState {
            ip: 0,
            registers: [i, 0, 0, 0],
            program: program.clone()
        };
        let mut output_should_be = 0;
        loop {
            let (is_running, output) = step_program(&mut program_state);
            if !is_running {
                break
            }
            match output {
                Some(v) => if v == output_should_be {
                    if v == 1 && past_one_states.contains(&program_state) {
                        return i
                    } else if v == 1 {
                        past_one_states.push(program_state.clone())
                    }
                    output_should_be = (output_should_be + 1) % 2; // 0 -> 1, 1 -> 0
                } else {
                    break
                },
                None => ()
            }
        } 
    }
    -1
}

pub fn part2(_source: String) -> String {
    String::from("Merry Christmas!")
}