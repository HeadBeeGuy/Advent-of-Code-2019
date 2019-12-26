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

// This is the first time I've extracted the processor into a separate module.

use std::{env, fs, process};

// I think I'm making this nomenclature unnecessarily complex, but this is the
// first time I've used modules.
mod intcode;
use intcode::intcode::IntcodeProcessor as intcode_processor;

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

    let mut part_1_processor = intcode_processor::new(&intcode_program);
    part_1_processor.supply_input(1);
    part_1_processor.execute_until_halt();
    if let Some(value) = part_1_processor.get_output() {
        println!("The first processor generated an output of: {}", value);
    }

    let mut part_2_processor = intcode_processor::new(&intcode_program);
    part_2_processor.supply_input(2);
    part_2_processor.execute_until_halt();
    if let Some(value) = part_2_processor.get_output() {
        println!("The second processor generated an output of: {}", value);
    }
}
