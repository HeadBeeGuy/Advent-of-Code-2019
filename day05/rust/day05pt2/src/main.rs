// Advent of Code 2019 Day 5 part 2

// The big change I made for part 2 is that now every instruction returns the
// address of the next instruction.
// After working on part 1 so much, this was much easier to come up with!
// I'm impressed by the clever intcode programs in the example.

use std::env;
use std::fs;
use std::process;

#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct InstructionResult {
    return_value: isize,
    next_instruction: usize,
    halt: bool,
}

fn process_arguments(args: &[String]) -> Result<(Vec<isize>, isize), &str> {
    if args.len() != 3 {
        return Err("Please use the following arguments when you run the program: an intcode file and an integer input.");
    }
    let program_file = fs::read_to_string(&args[1]).expect("The file path was not valid.");
    let program_input = &args[2]
        .parse::<isize>()
        .expect("Please confirm that the program input is an integer");
    Ok((
        program_file
            .trim()
            .split(",")
            .map(|item| item.parse().unwrap())
            .collect(),
        *program_input,
    ))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (mut intcode_program, program_input) = process_arguments(&args).unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1);
    });

    let mut instruction_pointer = 0;
    loop {
        let result = process_instruction(instruction_pointer, &mut intcode_program, program_input);
        if result.halt {
            break;
        }
        instruction_pointer = result.next_instruction;
    }
}

fn process_instruction(
    address: usize,
    program: &mut Vec<isize>,
    program_input: isize,
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

        // This is probably inefficient - convert the (up to 2) remaining digits
        // to a string, then parse them backwards because they go right-to-left.
        // As of yet, parameter 3 doesn't seem to apply to anything.
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
            return_value: 0,
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
                return_value: 0,
                next_instruction: address + 4,
                halt: false,
            };
        }
        3 => {
            // move
            program[first_parameter_location] = program_input;
            return InstructionResult {
                return_value: 0,
                next_instruction: address + 2,
                halt: false,
            };
        }
        4 => {
            // output
            let return_value = program[first_parameter_location];
            println!("Return value generated: {}", return_value);
            return InstructionResult {
                return_value,
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
                return_value: 0,
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
                return_value: 0,
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
                return_value: 0,
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
                return_value: 0,
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
                return_value: 0,
                next_instruction: address + 1,
                halt: false,
            };
        }
    }
}
