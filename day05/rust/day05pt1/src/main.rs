// Advent of Code 2019 Day 5 part 1

// This ended up being a huge undertaking. I had several nights in bed thinking
// about Intcode when I was trying to sleep. I started dabbling in structs and
// enums.

// Thanks to the help of some kind folks on the Advent of Code subreddit, it
// finally works! I wasn't handling "input" correctly.

// I wonder if this is what it's like to try to write an emulator.

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
    width: usize,
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

    let mut current_index = 0;
    loop {
        let result = process_instruction(current_index, &mut intcode_program, program_input);
        if result.halt {
            break;
        }
        current_index += result.width;
    }
}

fn process_instruction(
    address: usize,
    program: &mut Vec<isize>,
    program_input: isize,
) -> InstructionResult {
    let opcode = program[address];
    let mut short_opcode = opcode;

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
            width: 1,
            halt: true,
        };
    }

    match short_opcode {
        1 | 2 => {
            // by definition, this is always in position mode
            let storage_address = program[address + 3] as usize;

            let first_operand: isize;
            let second_operand: isize;

            if param_1_type == Mode::Immediate {
                first_operand = program[address + 1];
            } else {
                first_operand = program[program[address + 1] as usize];
            }
            if param_2_type == Mode::Immediate {
                second_operand = program[address + 2];
            } else {
                second_operand = program[program[address + 2] as usize];
            }

            if short_opcode == 1 {
                program[storage_address] = first_operand + second_operand;
            } else if short_opcode == 2 {
                program[storage_address] = first_operand * second_operand;
            }

            return InstructionResult {
                return_value: 0,
                width: 4,
                halt: false,
            };
        }
        3 => {
            let save_location = program[address + 1] as usize;
            program[save_location] = program_input;
            return InstructionResult {
                return_value: 0,
                width: 2,
                halt: false,
            };
        }
        4 => {
            let return_value = program[program[address + 1] as usize];
            println!("Return value generated: {}", return_value);
            return InstructionResult {
                return_value,
                width: 2,
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
                width: 1,
                halt: false,
            };
        }
    }
}
