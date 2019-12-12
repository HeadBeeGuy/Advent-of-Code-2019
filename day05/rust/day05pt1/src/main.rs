// Advent of Code 2019 Day 5 part 1

// This ended up being a huge undertaking. I had several nights in bed thinking
// about Intcode when I was trying to sleep. I started dabbling in structs and
// enums. Something still isn't functioning right!

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

fn process_arguments(args: &[String]) -> Result<Vec<isize>, &str> {
    if args.len() != 2 {
        return Err("Please specify an incode input file as an argument to the program.");
    }
    let program_file = fs::read_to_string(&args[1]).expect("The file path was not valid.");
    Ok(program_file
        .trim()
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut intcode_program = process_arguments(&args).unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1);
    });

    // This is the input as specified by the problem definition
    // Maybe I should make it a command-line argument if future problems dictate.
    intcode_program[1] = 1;
    // println!("{:?}", intcode_program);
    let mut current_index = 0;
    loop {
        let result = process_instruction(current_index, &mut intcode_program);
        if result.halt {
            break;
        }
        current_index += result.width;
    }
    println!("{:?}", intcode_program);
}

fn process_instruction(address: usize, program: &mut Vec<isize>) -> InstructionResult {
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
            // instructions 3 and 4 don't seem to use Immediate Mode
            let value_and_address = program[address + 1];
            program[value_and_address as usize] = value_and_address;
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
