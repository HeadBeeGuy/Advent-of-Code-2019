// Advent of Code Day 9 parts 1 and 2

// Fortunately isize on a 64-bit system (that is to say, i64) seems to be large
// enough to contain the numbers this puzzle uses.

// I refactored my Intcode processor to use a hash map as its addressing system
// so it could address instructions out of range of the initial program. That
// led to a messier addressing system so I tried to abstract getting and setting
// values into functions.

// I was stuck for quite a while on this one until I finally realized that I
// wasn't adding the relative offset at the correct point. I'm amazed that the
// intcode program apparently outputs instructions that aren't working correctly.
// I can only imagine how Mr. Wastl came up with that!

// Completion takes about a second on my laptop since part 2 is apparently
// designed to be computationally expensive. After spending as much time on this
// as I have, I really don't want to think about optimizations yet!

// I should really abstract the Intcode processor into some kind of library so I
// can run tests on it to check for regressions.

use std::collections::HashMap;
use std::collections::VecDeque;
use std::{env, fs, process};

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
    let args: Vec<String> = env::args().collect();
    let intcode_program = process_arguments(&args).unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1);
    });

    let mut part_1_processor = IntcodeProcessor::new(&intcode_program);
    part_1_processor.supply_input(1);
    part_1_processor.execute_until_halt();
    if let Some(value) = part_1_processor.output {
        println!("The first processor generated an output of: {}", value);
    }

    let mut part_2_processor = IntcodeProcessor::new(&intcode_program);
    part_2_processor.supply_input(2);
    part_2_processor.execute_until_halt();
    if let Some(value) = part_2_processor.output {
        println!("The second processor generated an output of: {}", value);
    }
}

#[derive(Debug, PartialEq)]
enum InstructionMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
struct InstructionResult {
    return_value: Option<isize>,
    next_instruction: isize,
    halt: bool,
}

#[derive(Debug)]
struct IntcodeProcessor {
    intcode_program: HashMap<isize, isize>,
    input_queue: VecDeque<isize>,
    instruction_pointer: isize,
    relative_base: isize,
    output: Option<isize>,
    halted: bool,
}

impl IntcodeProcessor {
    fn new(base_program: &Vec<isize>) -> IntcodeProcessor {
        let mut intcode_program: HashMap<isize, isize> = HashMap::new();
        for item in base_program.iter().enumerate() {
            intcode_program.insert(item.0 as isize, *item.1);
        }

        let input_queue: VecDeque<isize> = VecDeque::new();
        IntcodeProcessor {
            intcode_program,
            input_queue,
            instruction_pointer: 0,
            relative_base: 0,
            output: None,
            halted: false,
        }
    }

    #[allow(dead_code)]
    fn execute_until_output(&mut self) {
        loop {
            let result = self.process_instruction();
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

    fn execute_until_halt(&mut self) {
        loop {
            let result = self.process_instruction();
            self.instruction_pointer = result.next_instruction;
            if result.halt {
                self.halted = true;
                break;
            }
            if let Some(value) = result.return_value {
                self.output = Some(value);
            }
        }
    }

    fn supply_input(&mut self, input_value: isize) {
        self.input_queue.push_back(input_value);
    }

    // when supplied a parameter, determine where to store it in the intcode program
    fn find_storage_address(
        &self,
        parameter_number: isize,
        instruction_mode: InstructionMode,
    ) -> isize {
        match instruction_mode {
            InstructionMode::Position => *self
                .intcode_program
                .get(&(self.instruction_pointer + parameter_number))
                .unwrap_or(&0),
            InstructionMode::Relative => *self
                .intcode_program
                .get(&(self.instruction_pointer + parameter_number))
                .unwrap_or(&0) + self.relative_base,
            InstructionMode::Immediate => {
                // Day 5 specifies this should never happen, but just in case...
                self.instruction_pointer + parameter_number
            }
        }
    }

    // when supplied a parameter number, determine what value the operation should use
    fn find_value(&self, parameter_number: isize, instruction_mode: InstructionMode) -> isize {
        match instruction_mode {
            InstructionMode::Position => *self
                .intcode_program
                .get(
                    self.intcode_program
                        .get(&(self.instruction_pointer + parameter_number))
                        .unwrap_or(&0),
                )
                .unwrap_or(&0),
            InstructionMode::Relative => *self
                .intcode_program
                .get(
                    &(self.relative_base + self.intcode_program
                        .get(&(self.instruction_pointer + parameter_number))
                        .unwrap_or(&0)),
                )
                .unwrap_or(&0),
            InstructionMode::Immediate => *self
                .intcode_program
                .get(&(self.instruction_pointer + parameter_number))
                .unwrap_or(&0),
        }
    }

    fn write_value(&mut self, location: isize, value: isize) {
        self.intcode_program.insert(location, value);
    }

    fn process_instruction(&mut self) -> InstructionResult {
        let opcode = *self
            .intcode_program
            .get(&self.instruction_pointer)
            .unwrap_or(&99);
        let mut short_opcode = opcode;

        // by default, parameters are in position mode
        let mut param_1_type = InstructionMode::Position;
        let mut param_2_type = InstructionMode::Position;
        let mut param_3_type = InstructionMode::Position;
        if opcode == 99 {
            return InstructionResult {
                return_value: None,
                next_instruction: self.instruction_pointer + 1,
                halt: true,
            };
        } else if opcode > 99 {
            // extract the last two digits
            short_opcode = opcode % 100;

            let remaining_opcode = (opcode / 100).to_string();
            let mut remaining_codes = remaining_opcode.chars().rev();

            let first_parameter_mode = remaining_codes.next();
            match first_parameter_mode {
                Some('1') => param_1_type = InstructionMode::Immediate,
                Some('2') => param_1_type = InstructionMode::Relative,
                _ => {}
            }
            let second_parameter_mode = remaining_codes.next();
            match second_parameter_mode {
                Some('1') => param_2_type = InstructionMode::Immediate,
                Some('2') => param_2_type = InstructionMode::Relative,
                _ => {}
            }
            let third_parameter_mode = remaining_codes.next();
            match third_parameter_mode {
                Some('1') => param_3_type = InstructionMode::Immediate,
                Some('2') => param_3_type = InstructionMode::Relative,
                _ => {}
            }
        }

        match short_opcode {
            1 | 2 => {
                // add or multiply
                let save_location = self.find_storage_address(3, param_3_type);
                if short_opcode == 1 {
                    self.write_value(
                        save_location,
                        self.find_value(1, param_1_type) + self.find_value(2, param_2_type),
                    )
                } else if short_opcode == 2 {
                    self.write_value(
                        save_location,
                        self.find_value(1, param_1_type) * self.find_value(2, param_2_type),
                    )
                }
                return InstructionResult {
                    return_value: None,
                    next_instruction: self.instruction_pointer + 4,
                    halt: false,
                };
            }
            3 => {
                // input instruction (move)
                let input = self
                    .input_queue
                    .pop_front()
                    .expect("The program asked for more inputs than it was supplied.");
                self.write_value(self.find_storage_address(1, param_1_type), input);
                return InstructionResult {
                    return_value: None,
                    next_instruction: self.instruction_pointer + 2,
                    halt: false,
                };
            }
            4 => {
                // output
                let return_value = self.find_value(1, param_1_type);
                return InstructionResult {
                    return_value: Some(return_value),
                    next_instruction: self.instruction_pointer + 2,
                    halt: false,
                };
            }
            5 => {
                // jump-if-true
                let next_instruction = if self.find_value(1, param_1_type) != 0 {
                    self.find_value(2, param_2_type)
                } else {
                    (self.instruction_pointer + 3)
                };
                return InstructionResult {
                    return_value: None,
                    next_instruction,
                    halt: false,
                };
            }
            6 => {
                // jump-if-false
                let next_instruction = if self.find_value(1, param_1_type) == 0 {
                    self.find_value(2, param_2_type)
                } else {
                    (self.instruction_pointer + 3)
                };
                return InstructionResult {
                    return_value: None,
                    next_instruction,
                    halt: false,
                };
            }
            7 => {
                // less-than
                let save_value =
                    if self.find_value(1, param_1_type) < self.find_value(2, param_2_type) {
                        1
                    } else {
                        0
                    };
                self.write_value(self.find_storage_address(3, param_3_type), save_value);

                return InstructionResult {
                    return_value: None,
                    next_instruction: self.instruction_pointer + 4,
                    halt: false,
                };
            }
            8 => {
                // equals
                let save_value =
                    if self.find_value(1, param_1_type) == self.find_value(2, param_2_type) {
                        1
                    } else {
                        0
                    };
                self.write_value(self.find_storage_address(3, param_3_type), save_value);

                return InstructionResult {
                    return_value: None,
                    next_instruction: self.instruction_pointer + 4,
                    halt: false,
                };
            }
            9 => {
                // modify relative base
                self.relative_base += self.find_value(1, param_1_type);
                return InstructionResult {
                    return_value: None,
                    next_instruction: self.instruction_pointer + 2,
                    halt: false,
                };
            }
            _ => {
                println!(
                    "Error: received instruction with invalid opcode ({}) at address {}.",
                    short_opcode, self.instruction_pointer
                );
                return InstructionResult {
                    return_value: None,
                    next_instruction: self.instruction_pointer + 1,
                    halt: false,
                };
            }
        }
    }
}
