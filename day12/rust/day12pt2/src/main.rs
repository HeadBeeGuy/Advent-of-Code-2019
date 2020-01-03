// Advent of Code Day 12 part 2

// I did something I did a few times last year, and that's look at an existing
// solution on a blog post before writing my own. Here's what I based this on:
// https://dhconnelly.com/advent-of-code-2019-commentary.html#day-12

// It's always kind of a bummer to base my solution on someone else's, even if
// they're a pro and making their solution freely available. I wish I were clever
// enough to come up with this on my own!

// I originally used the naive original implementation from part 1, but in true
// Advent of Code fashion, it would have taken an absurd amount of time. My final
// answer was something like 500 trillion and even this far more efficient method
// takes about a second and a half on my laptop. I haven't learned about threads
// or async in Rust yet, so maybe I could speed this up by running the three
// calculations in parallel.

use num::Integer;
use regex::Regex;
use std::{env, fs, process};

#[derive(Debug, PartialEq, Clone)]
struct Moon {
    pos_x: i32,
    pos_y: i32,
    pos_z: i32,
    v_x: i32,
    v_y: i32,
    v_z: i32,
}

fn process_arguments(args: &[String]) -> Result<(Vec<i32>, Vec<i32>, Vec<i32>), &str> {
    if args.len() != 2 {
        return Err("Please specify a text file for the moon data.");
    }
    let moon_format =
        Regex::new(r"<[xyz]=(-?[0-9]+),\s[xyz]=(-?[0-9]+),\s[xyz]=(-?[0-9]+)>").unwrap();
    let seed_file = fs::read_to_string(&args[1]).expect("The file path was not valid.");

    let mut x_coordinates: Vec<i32> = Vec::new();
    let mut y_coordinates: Vec<i32> = Vec::new();
    let mut z_coordinates: Vec<i32> = Vec::new();

    for cap in moon_format.captures_iter(&seed_file) {
        let new_x = &cap[1]
            .parse::<i32>()
            .expect("A moon had an invalid starting value in its position.");
        let new_y = &cap[2]
            .parse::<i32>()
            .expect("A moon had an invalid starting value in its position.");
        let new_z = &cap[3]
            .parse::<i32>()
            .expect("A moon had an invalid starting value in its position.");

        x_coordinates.push(*new_x);
        y_coordinates.push(*new_y);
        z_coordinates.push(*new_z);
    }
    Ok((x_coordinates, y_coordinates, z_coordinates))
}

fn steps_until_axis_repeats(moon_positions: &Vec<i32>) -> i64 {
    // Assumptions:
    // Every moon starts with zero velocity.
    // There are four moons.

    let mut steps: i64 = 0;
    let initial_positions = moon_positions.clone();
    let mut current_positions = moon_positions.clone();
    let mut moon_velocities: Vec<i32> = vec![0, 0, 0, 0];

    loop {
        steps += 1;

        // apply gravitational pull from other moons
        for current_moon in 0..4 {
            for other_moon in 0..4 {
                if !(current_moon == other_moon) {
                    if current_positions[current_moon] < current_positions[other_moon] {
                        moon_velocities[current_moon] += 1;
                    } else if current_positions[current_moon] > current_positions[other_moon] {
                        moon_velocities[current_moon] -= 1;
                    }
                }
            }
        }

        for current_moon in 0..4 {
            current_positions[current_moon] += moon_velocities[current_moon];
        }

        if (current_positions == initial_positions) && (moon_velocities == vec![0, 0, 0, 0]) {
            break;
        }
    }
    steps
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (x_coordinates, y_coordinates, z_coordinates) =
        process_arguments(&args).unwrap_or_else(|err| {
            println!("Argument error: {}", err);
            process::exit(1);
        });

    let x = steps_until_axis_repeats(&x_coordinates);
    let y = steps_until_axis_repeats(&y_coordinates);
    let z = steps_until_axis_repeats(&z_coordinates);

    println!(
        "History repeats itself. In this case, it will take {} steps.",
        x.lcm(&(y.lcm(&z)))
    );
}
