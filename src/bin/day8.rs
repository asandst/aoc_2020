use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day8")?;
    let input = BufReader::new(input);

    let mut program: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let line = line?;

        let parts = line.split(" ").collect::<Vec<&str>>();
        let instr = parts[0];
        let value = parts[1].parse::<i64>().unwrap();

        program.push(Instruction {
            instruction: instr.to_string(),
            value: value,
            visited: false,
        })
    }
    let program = program;

    let mut state_part1 =  State {
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

        match instr.instruction.as_str() {
            "acc" => {
                state_part1.accumulator += instr.value;
                state_part1.instruction_position += 1
            }
            "jmp" => state_part1.instruction_position = (state_part1.instruction_position as i64+ instr.value) as usize,
            "nop" => state_part1.instruction_position += 1,
            _ => panic!(),
        }
    }
    println!("Part 1 {}", state_part1.accumulator);

    let mut states: Vec<State> = Vec::new();
    states.push(State {
        instruction_position: 0,
        accumulator: 0,
        modified_code: false,
        program: program.clone(),
    });

    loop {
        let mut state = states.pop().unwrap();
        let state_program_clone = state.program.clone();

        if state.instruction_position == state.program.len() {
            println!("Part 2 {}", state.accumulator);
            break;
        }

        let instr = &mut state.program[state.instruction_position];
        if instr.visited {
            continue;
        }

        instr.visited = true;

        match instr.instruction.as_str() {
            "acc" => {
                let s1 = State {
                    instruction_position: state.instruction_position + 1,
                    accumulator: state.accumulator + instr.value,
                    modified_code: state.modified_code,
                    program: state.program,
                };
                states.push(s1);
            }
            "jmp" => {
                let s1 = State {
                    instruction_position: (state.instruction_position as i64 + instr.value) as usize,
                    accumulator: state.accumulator,
                    modified_code: state.modified_code,
                    program: state.program,
                };
                states.push(s1);
                if !state.modified_code {
                    let s2 = State {
                        instruction_position: state.instruction_position + 1,
                        accumulator: state.accumulator,
                        modified_code: true,
                        program: state_program_clone,
                    };
                    states.push(s2);
                }
            }
            "nop" => {
                if !state.modified_code {
                    let s1 = State {
                        instruction_position: (state.instruction_position as i64 + instr.value)
                            as usize,
                        accumulator: state.accumulator,
                        modified_code: true,
                        program: state_program_clone,
                    };
                    states.push(s1);
                }

                let s2 = State {
                    instruction_position: state.instruction_position + 1,
                    accumulator: state.accumulator,
                    modified_code: state.modified_code,
                    program: state.program,
                };
                states.push(s2);
            }
            _ => panic!(),
        }
    }

    Ok(())
}

#[derive(Clone, Debug)]
struct Instruction {
    instruction: String,
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
