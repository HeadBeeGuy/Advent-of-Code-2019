// Advent of Code 2019 Day 11 part 1

// An approach to Day 10 was eluding me, so I started on Day 11. This was a fun
// problem. Putting the intcode processor in a module last time made it easy to
// dig it up this time around.

// It's an unsophisticated solution that just simulates the robot running
// around on a 2d grid and modifying a hash map of (x,y) coordinates.
// If it visits a particular spot again, it just overwrites the previous color.
// This makes finding the solution easy - how many key/value pairs are in the
// hash map?

use std::collections::HashMap;
use std::{env, fs, process};

mod intcode;
use intcode::intcode::IntcodeProcessor as intcode_processor;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    let args: Vec<String> = env::args().collect();
    let intcode_program = process_arguments(&args).unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1);
    });

    let mut paint_program = intcode_processor::new(&intcode_program);
    let mut hull: HashMap<(isize, isize), u8> = HashMap::new();

    let mut current_x = 0;
    let mut current_y = 0;
    let mut current_direction = Direction::Up;
    loop {
        let hull_color = hull.get(&(current_x, current_y)).unwrap_or(&0);
        paint_program.supply_input(*hull_color as isize);
        paint_program.execute_until_output();
        if paint_program.is_halted() {
            break;
        }
        hull.insert(
            (current_x, current_y),
            paint_program.get_output().unwrap_or(99) as u8,
        );

        paint_program.execute_until_output();
        if paint_program.is_halted() {
            break;
        }
        let updated_position = turn_and_advance(
            paint_program.get_output().unwrap_or(99) as u8,
            &current_direction,
            current_x,
            current_y,
        );
        current_direction = updated_position.0;
        current_x = updated_position.1;
        current_y = updated_position.2;
    }

    println!(
        "The paint program painted {} spots on the hull.",
        hull.len()
    );
}

fn turn_and_advance(
    turn_instruction: u8,
    current_direction: &Direction,
    current_x: isize,
    current_y: isize,
) -> (Direction, isize, isize) {
    let mut new_direction: Direction = Direction::Down;
    let mut new_x: isize = current_x;
    let mut new_y: isize = current_y;

    // Maybe there's a more compact way to do this, but it makes it easy to read.
    match turn_instruction {
        0 => {
            // left turn
            match *current_direction {
                Direction::Up => new_direction = Direction::Left,
                Direction::Right => new_direction = Direction::Up,
                Direction::Down => new_direction = Direction::Right,
                Direction::Left => new_direction = Direction::Down,
            }
        }
        1 => {
            // right turn
            match *current_direction {
                Direction::Up => new_direction = Direction::Right,
                Direction::Right => new_direction = Direction::Down,
                Direction::Down => new_direction = Direction::Left,
                Direction::Left => new_direction = Direction::Up,
            }
        }
        _ => println!("Provided invalid turn instruction: {}", turn_instruction),
    }

    match new_direction {
        Direction::Up => new_y = current_y + 1,
        Direction::Right => new_x = current_x + 1,
        Direction::Down => new_y = current_y - 1,
        Direction::Left => new_x = current_x - 1,
    }

    (new_direction, new_x, new_y)
}
