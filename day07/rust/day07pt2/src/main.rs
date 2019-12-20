// Advent of Code Day 7 part 2

// Phew! I'm kind of embarrassed to put this up, but it works. I had trouble
// understanding the puzzle until I read this reddit thread:
// https://old.reddit.com/r/adventofcode/comments/e7aqcb/2019_day_7_part_2_confused_with_the_question/?st=k4b9g9q6&sh=314142b3

// I'm used to object-oriented class-based languages like Ruby, so I turned the
// amplifiers into individual execution units with their own states. It's probably
// much messier than it should be. I was reading up on structs and how to make
// functions for them, so that's the tool I tried out this time.

// There's probably much to be improved about this, but I'm quite a few days behind
// now. I'm kind of amazed that it works at all, and quickly at that!

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

#[derive(Debug)]
struct AmplifierUnit {
    phase_setting: isize,
    intcode_program: Vec<isize>,
    input_queue: Vec<isize>,
    instruction_pointer: usize,
    output: Option<isize>,
    halted: bool,
}

impl AmplifierUnit {
    fn new(phase_setting: isize, intcode_program: &Vec<isize>) -> AmplifierUnit {
        AmplifierUnit {
            phase_setting,
            intcode_program: intcode_program.clone(),
            input_queue: vec![phase_setting],
            instruction_pointer: 0,
            output: None,
            halted: false,
        }
    }

    fn execute_until_output(&mut self) {
        // Oof! This is a bad hack! The first 5 runs will have the inputs in
        // reverse order, since they function by popping off the last value.
        // I should be more clever about this, but this makes it work!
        if self.input_queue.len() == 2 {
            self.input_queue.swap(0, 1);
        }
        // if this unit is resuming execution, un-halt it and clear its return value
        self.halted = false;
        self.output = None;
        loop {
            let result = process_instruction(
                self.instruction_pointer,
                &mut self.intcode_program,
                &mut self.input_queue,
            );
            self.instruction_pointer = result.next_instruction;
            if result.halt {
                self.halted = true;
                break;
            }
            if let Some(value) = result.return_value {
                self.output = Some(value);
                break;
            }
        }
    }

    fn provide_input(&mut self, input_value: isize) {
        self.input_queue.push(input_value);
    }
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
    let mut all_permutations: Vec<Vec<isize>> = Vec::new();
    heaps_alg(&mut vec![5, 6, 7, 8, 9], 5, &mut all_permutations);

    let args: Vec<String> = env::args().collect();
    let intcode_program = process_arguments(&args).unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1);
    });

    let mut max_signal = 0;
    for phase_sequence in all_permutations.iter() {
        let mut amplifiers: Vec<AmplifierUnit> = Vec::new();
        for unit_number in phase_sequence {
            amplifiers.push(AmplifierUnit::new(*unit_number, &intcode_program.clone()))
        }

        let mut next_input: isize = 0; // initial input

        // This is a hacky way to do this, but apparently you can't have a mutable
        // iterator and a cycling iterator at the same time.
        let mut current_index: usize = 0;

        loop {
            let this_amp = &mut amplifiers[current_index];
            &this_amp.provide_input(next_input);
            &this_amp.execute_until_output();

            if this_amp.halted {
                break;
            }
            next_input = this_amp.output.unwrap();
            if current_index >= 4 {
                current_index = 0;
            } else {
                current_index += 1;
            }
        }
        if next_input > max_signal {
            max_signal = next_input;
        }
    }

    println!("The max possible signal appears to be: {}", max_signal);
}

// Unchanged from part 1. It would be much better if it just returned the
// all_permutations vector, but I wonder if that would involve writing yet
// another function.
fn heaps_alg(v: &mut [isize], n: usize, all_permutations: &mut Vec<Vec<isize>>) {
    if n == 1 {
        all_permutations.push(v.to_vec()); // this is probably not the best way to do this
        return;
    }

    for x in 0..n - 1 {
        heaps_alg(v, n - 1, all_permutations);

        if n % 2 == 0 {
            v.swap(n - 1, x);
        } else {
            v.swap(n - 1, 0);
        }
    }

    heaps_alg(v, n - 1, all_permutations);
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
