// Advent of Code 2019 Day 11 part 2

// Fortunately this was just a really quick modification of part 1. I suspected
// there would be a step like this!
// Once again, I'm impressed that the same Intcode program could be constructed
// to produce such different results based on an input change.

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

    // Part 2 specifies that the first spot is white
    hull.insert((0,0), 1);

    let (mut min_y, mut min_x, mut max_x, mut max_y) = (0,0,0,0);

    let mut current_x = 0;
    let mut current_y = 0;
    let mut current_direction = Direction::Up;
    loop {
        if current_x < min_x { min_x = current_x; }
        if current_y < min_y { min_y = current_y; }
        if current_x > max_x { max_x = current_x; }
        if current_y > max_y { max_y = current_y; }

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

    // I'm not sure why, but it ends up printing upside-down by default
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let color = hull.get(&(x,y));
            match color {
                Some(1) => print!("X"),
                _ => print!(" ")
            }
        }
        print!("\n");
    }
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
