// Advent of Code Day 7 part 1

// I could use the Intcode computer from Day 5, although I made a few changes.
// When it returns a result, I use an Option, since I just read about them again.
// Inputs are now passed in as a Vector and they get popped off as they're fed in.


use std::{env, fs, process};

#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct InstructionResult {
    return_value: Option<isize>,
    next_instruction: usize,
    halt: bool,
}

fn process_arguments(args: &[String]) -> Result<Vec<isize>, &str> {
    if args.len() != 2 {
        return Err("Please specify a text file for the intcode program.");
    }
    let program_file = fs::read_to_string(&args[1]).expect("The file path was not valid.");
    Ok(program_file
        .trim()
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect())
}

fn main() {
    let mut final_vector: Vec<Vec<isize>> = Vec::new();
    heaps_alg(&mut vec![0, 1, 2, 3, 4], 5, &mut final_vector);

    let args: Vec<String> = env::args().collect();
    let intcode_program = process_arguments(&args).unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1);
    });


    let mut max_signal = 0;
    for phase_sequence in final_vector.iter() {
        let mut current_sequence = phase_sequence.clone();
        let mut input_signals = vec![0]; // defined by problem: Initial input is 0
        for _ in 1..=5 {
            let mut current_inputs = vec![input_signals.pop().unwrap(), current_sequence.pop().unwrap()];
            let mut program = intcode_program.clone();

            let mut instruction_pointer = 0;
            loop {
                let result = process_instruction(instruction_pointer, &mut program, &mut current_inputs);
                if result.halt {
                    break;
                }
                if let Some(value) = result.return_value {
                    input_signals.push(value);
                }
                instruction_pointer = result.next_instruction;
            }
        }

        let final_value = input_signals.pop().expect("There was no final value after running through the input sequence.");
        if final_value > max_signal {
            max_signal = final_value;
        }
    }

    println!("The highest possible signal appears to be {}.", max_signal);
}

// I wish I could say that I derived this on my own! But actually I just did a
// search and adapted this from this post:
// https://users.rust-lang.org/t/heaps-algorithm-incomplete/32585/3
fn heaps_alg(v: &mut [isize], n: usize, final_vector: &mut Vec<Vec<isize>>) {
    if n == 1 {
        final_vector.push(v.to_vec()); // this is probably not the best way to do this
        return;
    }

    for x in 0..n - 1 {
        heaps_alg(v, n - 1, final_vector);

        if n % 2 == 0 {
            v.swap(n - 1, x);
        } else {
            v.swap(n - 1, 0);
        }
    }

    heaps_alg(v, n - 1, final_vector);
}

fn process_instruction(
    address: usize,
    program: &mut Vec<isize>,
    program_input: &mut Vec<isize>,
) -> InstructionResult {
    let opcode = program[address];
    let mut short_opcode = opcode;

    let first_parameter_location: usize;
    let second_parameter_location: usize;

    // by default, parameters are in position mode
    let mut param_1_type = Mode::Position;
    let mut param_2_type = Mode::Position;
    // Originally there was a param_3_type here but it's never read, since the
    // third parameter is always in position mode, as I understand it.

    if opcode > 99 {
        // extract the last two digits
        short_opcode = opcode % 100;

        let remaining_opcode = (opcode / 100).to_string();
        let mut remaining_codes = remaining_opcode.chars().rev().peekable();

        if remaining_codes.next().unwrap() == '1' {
            param_1_type = Mode::Immediate;
        }
        if remaining_codes.peek() != None && remaining_codes.next().unwrap() == '1' {
            param_2_type = Mode::Immediate;
        }
    } else if opcode == 99 {
        return InstructionResult {
            return_value: None,
            next_instruction: address + 1,
            halt: true,
        };
    }

    first_parameter_location = if param_1_type == Mode::Immediate {
        (address + 1) as usize
    } else {
        *program
            .get(address + 1)
            .expect("Attempted to access parameter 1 in an instruction at the end of the program.")
            as usize
    };
    second_parameter_location = if param_2_type == Mode::Immediate {
        (address + 2) as usize
    } else {
        *program
            .get(address + 2)
            .expect("Attempted to access parameter 2 in an instruction at the end of the program.")
            as usize
    };

    match short_opcode {
        1 | 2 => {
            // add or multiply
            // by definition, this is always in position mode
            let storage_address = program[address + 3] as usize;

            let first_operand: isize = program[first_parameter_location];
            let second_operand: isize = program[second_parameter_location];

            if short_opcode == 1 {
                program[storage_address] = first_operand + second_operand;
            } else if short_opcode == 2 {
                program[storage_address] = first_operand * second_operand;
            }

            return InstructionResult {
                return_value: None,
                next_instruction: address + 4,
                halt: false,
            };
        }
        3 => {
            // input instruction (move)
            program[first_parameter_location] = program_input
                .pop()
                .expect("The program asked for more inputs than it was supplied.");
            return InstructionResult {
                return_value: None,
                next_instruction: address + 2,
                halt: false,
            };
        }
        4 => {
            // output
            let return_value = program[first_parameter_location];
            return InstructionResult {
                return_value: Some(return_value),
                next_instruction: address + 2,
                halt: false,
            };
        }
        5 => {
            // jump-if-true
            let next_instruction: usize = if program[first_parameter_location] != 0 {
                program[second_parameter_location] as usize
            } else {
                (address + 3)
            };

            return InstructionResult {
                return_value: None,
                next_instruction,
                halt: false,
            };
        }
        6 => {
            // jump-if-false
            let next_instruction: usize = if program[first_parameter_location] == 0 {
                program[second_parameter_location] as usize
            } else {
                (address + 3)
            };

            return InstructionResult {
                return_value: None,
                next_instruction,
                halt: false,
            };
        }
        7 => {
            // less-than
            let storage_address = program[address + 3] as usize;
            program[storage_address] =
                if program[first_parameter_location] < program[second_parameter_location] {
                    1
                } else {
                    0
                };

            return InstructionResult {
                return_value: None,
                next_instruction: address + 4,
                halt: false,
            };
        }
        8 => {
            // equals
            let storage_address = program[address + 3] as usize;
            program[storage_address] =
                if program[first_parameter_location] == program[second_parameter_location] {
                    1
                } else {
                    0
                };

            return InstructionResult {
                return_value: None,
                next_instruction: address + 4,
                halt: false,
            };
        }
        _ => {
            // I'll assume that an erroneous opcode has a width of only 1
            println!(
                "Error: received instruction with invalid opcode ({}) at address {}.",
                short_opcode, address
            );
            return InstructionResult {
                return_value: None,
                next_instruction: address + 1,
                halt: false,
            };
        }
    }
}
