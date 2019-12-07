// Advent of Code Day 4 part 1

// Oh, those silly elves! Who would write a password down on a sticky note and
// then lose it? Clearly not myself!

// This is a fun puzzle to think about because it's simple to grasp but thinking
// of a way to solve it involves evaluating a lot of different ideas.

// Roughly speaking, this is the algorithm I came up with:
// - iterate through each number, checking if there are any repeating digits
// - finding a digit less than the previous one means that every following
// digit should be changed to that one, because no number between them will
// match the criteria
// - finding a number that ends in 9 means the next viable number needs to end
// with at least 23, at least if the number is greater than 111113

// I'm probably missing lots of optimizations, but it runs almost instantaneously.

use std::env;
use std::process;

// Command line processing swiped from Chapter 12 of The Book
// Realistically it's unnecessary, but it's probably a good exercise nonetheless.
fn parse_arguments(args: &[String]) -> Result<(u32, u32), &str> {
    if args.len() < 3 {
        return Err("Please supply two command-line arguments.");
    }
    let lower_bound = args[1]
        .parse::<u32>()
        .expect("Please specify an integer for the lower bound.");
    let upper_bound = args[2]
        .parse::<u32>()
        .expect("Please specify an integer for the upper bound.");
    if lower_bound > upper_bound {
        return Err(
            "The first argument (the lower bound) needs to be smaller than the second argument.",
        );
    }
    if lower_bound < 100_000
        || lower_bound > 999_999
        || upper_bound < 100_000
        || upper_bound > 999_999
    {
        return Err("Both arguments must be positive 6-digit numbers.");
    }
    Ok((lower_bound, upper_bound))
}

// I'm going to represent numbers as vectors of 6 digits to make it easier to
// compare digits. Maybe I'll realize there was a better way to do this as I
// learn more Rust!
// This was code I found here: https://stackoverflow.com/a/41536521
fn u32_to_digit_vec (num: u32) -> Vec<u32> {
    fn inner(n: u32, xs: &mut Vec<u32>) {
        if n >= 10 {
            inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    inner(num, &mut xs);
    xs
}

fn digit_vec_to_u32 (vec: &Vec<u32>) -> u32 {
    (vec[0] * 100_000) + (vec[1] * 10_000) + (vec[2] * 1_000) + (vec[3] * 100) + (vec[4] * 10) + vec[5]
}

fn main() {
    // This will be the first time I try to use command line arguments.
    let args: Vec<String> = env::args().collect();
    let (lower_bound, upper_bound) = parse_arguments(&args).unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1)
    });

    let mut potential_passwords_found: usize = 0;

    let mut current_candidate = u32_to_digit_vec(lower_bound);
    let mut next_candidate = current_candidate.clone();

    while digit_vec_to_u32(&current_candidate) < upper_bound {
        let mut consecutive_digits_found = false;
        let mut decreasing_digit_found = false;
        let mut new_digit: u32 = 0;

        for comparison_digit in 1..=5 {
            if decreasing_digit_found {
                next_candidate[comparison_digit] = new_digit;
            }
            else {
                if current_candidate[comparison_digit - 1] == current_candidate[comparison_digit] {
                    consecutive_digits_found = true;
                }
                else if current_candidate[comparison_digit - 1] > current_candidate[comparison_digit] {
                    decreasing_digit_found = true;
                    new_digit = current_candidate[comparison_digit - 1];
                    next_candidate[comparison_digit] = new_digit;
                }
            }
        }

        if consecutive_digits_found && !decreasing_digit_found {
            potential_passwords_found += 1;
        }

        if !decreasing_digit_found {
            if current_candidate[5] == 9 {
                let current_candidate_value = digit_vec_to_u32(&current_candidate);
                next_candidate = u32_to_digit_vec(current_candidate_value + 4);
            }
            else {
                next_candidate[5] += 1;
            }
        }
        current_candidate = next_candidate.clone();
    }

    println!("Found a total of {} potential passwords that fit the pattern.", potential_passwords_found);
}
