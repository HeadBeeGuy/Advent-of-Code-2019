// Advent of Code Day 13 part 1

// This generates the correct answer, but it's sort of an appetizer for part 2,
// which I'd heard about but couldn't see until I'd generated this answer for
// part 1.

// I'd imported the "console" crate because I'd seen that someone in the
// solution thread was using it, but it looks like it'll take some time to learn
// how to use it, and part 1 is just the initial game state so it's not
// necessary yet.

// Part 2 looks like it's going to be a challenge!

use std::collections::HashMap;
use std::{env, fs, process};

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

    let mut rendering_instructions = intcode_processor::new(&intcode_program);
    let mut game_screen: HashMap<(i8, i8), i8> = HashMap::new();

    let mut max_x: i8 = 1;
    let mut max_y: i8 = 1;

    let mut block_count: i32 = 0;

    while !rendering_instructions.is_halted() {
        let x = rendering_instructions
            .execute_until_output()
            .expect("The program was looking for an x value, but the machine halted early.")
            as i8;
        let y = rendering_instructions
            .execute_until_output()
            .expect("The program was looking for a y value, but the machine halted early.")
            as i8;

        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }

        let tile = rendering_instructions
            .execute_until_output()
            .expect("The program was looking for a tile value, but the machine halted early.")
            as i8;
        game_screen.insert((x, y), tile);
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            let print_this = game_screen.get(&(x, y));
            match print_this {
                Some(0) => print!(" "),
                Some(1) => print!("X"),
                Some(2) => {
                    print!("B");
                    block_count += 1;
                },
                Some(3) => print!("="),
                Some(4) => print!("O"),
                Some(x) => print!("{}", x),
                _ => {}
            }
        }
        print!("\n");
    }
    print!("\n");

    println!("The total number of blocks drawn was: {}.", block_count);
}
