use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::time::Instant;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let input = File::open("input_day8")?;
    let input = BufReader::new(input);

    let mut program: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let line = line?;

        let parts = line.split(" ").collect::<Vec<&str>>();
        let instr = parts[0];
        let value = parts[1].parse::<i64>().unwrap();

        let t: Type = match instr {
            "acc" => Type::ACC,
            "jmp" => Type::JMP,
            "nop" => Type::NOP,
            _ => panic!(),
        };

        program.push(Instruction {
            instruction: t,
            value: value,
            visited: false,
        })
    }

    let program = program;

    let now = Instant::now();

    let mut state_part1 = State {
        instruction_position: 0,
        accumulator: 0,
        modified_code: false,
        program: program.clone(),
    };

    loop {
        let instr = &mut state_part1.program[state_part1.instruction_position as usize];
        if instr.visited {
            break;
        }

        instr.visited = true;

        match instr.instruction {
            Type::ACC => {
                state_part1.accumulator += instr.value;
                state_part1.instruction_position += 1
            }
            Type::JMP => {
                state_part1.instruction_position =
                    (state_part1.instruction_position as i64 + instr.value) as usize
            }
            Type::NOP => state_part1.instruction_position += 1,
            _ => panic!(),
        }
    }
    println!("Part 1 {}", state_part1.accumulator);

    let mut states: VecDeque<State> = VecDeque::new();
    states.push_back(State {
        instruction_position: 0,
        accumulator: 0,
        modified_code: false,
        program: program.clone(),
    });

    loop {
        let mut state = states.pop_front().unwrap();

        if state.instruction_position == state.program.len() {
            println!("Part 2 {}", state.accumulator);
            println!("{}", now.elapsed().as_micros());
            break;
        }

        let mut instr = &mut state.program[state.instruction_position];
        if instr.visited {
            continue;
        }

        instr.visited = true;

        match instr.instruction {
            Type::ACC => {
                state.instruction_position += 1;
                state.accumulator += instr.value;
                states.push_back(state);
            }
            Type::JMP => {
                if !state.modified_code {
                    let s2 = State {
                        instruction_position: state.instruction_position + 1,
                        accumulator: state.accumulator,
                        modified_code: true,
                        program: program.clone(),
                    };
                    states.push_back(s2);
                }

                state.instruction_position = (state.instruction_position as i64 + instr.value) as usize;
                states.push_back(state);
            }
            Type::NOP => {
                if !state.modified_code {
                    let s1 = State {
                        instruction_position: (state.instruction_position as i64 + instr.value)
                            as usize,
                        accumulator: state.accumulator,
                        modified_code: true,
                        program: program.clone(),
                    };
                    states.push_back(s1);
                }

                state.instruction_position += 1;
                states.push_back(state);
            }
        }
    }

    Ok(())
}

#[derive(Clone, Debug)]
struct Instruction {
    instruction: Type,
    value: i64,
    visited: bool,
}

#[derive(Clone, Debug)]
struct State {
    instruction_position: usize,
    accumulator: i64,
    modified_code: bool,
    program: Vec<Instruction>,
}

#[derive(Clone, Debug)]
enum Type {
    NOP,
    ACC,
    JMP,
}
