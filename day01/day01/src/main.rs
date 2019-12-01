// It's the first day of Advent of Code!
// I imagine that this is filled with non-idiomatic Rust, but I hope that I'll be
// able to back and read The Book having made whatever silly mistakes I've made here
// so I can learn the better way at some point.

// Things I need to probably need to read up on: error handling, iterators and collections

use std::fs;

// "simple" is the part 1 answer and "complex" is part 2's answer.
fn required_fuel(mass: i32) -> (i32, i32) {
    let simple = (mass / 3) - 2;
    let mut complex = simple;
    let mut additional_fuel = complex;

    while additional_fuel > 0 {
        additional_fuel = (additional_fuel / 3) - 2;
        complex += additional_fuel;
    }
    (simple, complex)
}

fn main() {
    let input_file = fs::read_to_string("../input.txt").expect("Couldn't read the file.");
    let fuel_values = input_file.lines();

    let mut total_simple_fuel = 0;
    let mut total_fuel = 0;
    for mass in fuel_values {
        // could I do this with map and collect to avoid writing a for loop?
        let fuel_requirements = required_fuel(mass.parse::<i32>().unwrap());
        total_simple_fuel += fuel_requirements.0;
        total_fuel += fuel_requirements.1;
    }

    println!(
        "In the simple calculation (part 1), the total required fuel will be: {}",
        total_simple_fuel
    );
    println!(
        "Taking into account the weight of the fuel, the total required fuel will be: {}",
        total_fuel
    );
}
