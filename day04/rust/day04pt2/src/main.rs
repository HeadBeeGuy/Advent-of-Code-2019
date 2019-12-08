// Advent of Code Day 4 part 2

// Based on part 1, with only slight modifications -read that for more info!

// I peeped at a solution that a more experienced Rust programmer wrote and I was
// amazed at how simple it was and all of the different language functions
// it used. I'm embarrassed by my silly method here, but I guess I have to take
// baby steps to learn how to use the language to its potential!

use std::collections::HashMap;
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

fn u32_to_digit_vec(num: u32) -> Vec<u32> {
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

fn digit_vec_to_u32(vec: &Vec<u32>) -> u32 {
    (vec[0] * 100_000)
        + (vec[1] * 10_000)
        + (vec[2] * 1_000)
        + (vec[3] * 100)
        + (vec[4] * 10)
        + vec[5]
}

fn main() {
    // This will be the first time I try to use command line arguments.
    let args: Vec<String> = env::args().collect();
    let (lower_bound, upper_bound) = parse_arguments(&args).unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1)
    });

    let mut potential_passwords_found: usize = 0;
    let mut passwords_checked: usize = 0;

    let mut current_candidate = u32_to_digit_vec(lower_bound);
    let mut next_candidate = current_candidate.clone();

    while digit_vec_to_u32(&current_candidate) < upper_bound {
        let mut double_digits_found = false;
        let mut decreasing_digit_found = false;
        let mut new_digit: u32 = 0;
        let mut digit_counts: HashMap<u32, u32> = HashMap::new();

        digit_counts.insert(current_candidate[0], 1);
        for comparison_digit in 1..=5 {
            let count = digit_counts
                .entry(current_candidate[comparison_digit])
                .or_insert(0);
            *count += 1;

            if decreasing_digit_found {
                next_candidate[comparison_digit] = new_digit;
            } else {
                if current_candidate[comparison_digit - 1] > current_candidate[comparison_digit] {
                    decreasing_digit_found = true;
                    new_digit = current_candidate[comparison_digit - 1];
                    next_candidate[comparison_digit] = new_digit;
                }
            }
        }

        // Is there some sort of Ruby-style .contains? method in Rust?
        // I haven't been able to find one.
        for count in digit_counts.values() {
            if *count == 2 {
                double_digits_found = true;
            }
        }

        if double_digits_found && !decreasing_digit_found {
            potential_passwords_found += 1;
        }

        if !decreasing_digit_found {
            if current_candidate[5] == 9 {
                let current_candidate_value = digit_vec_to_u32(&current_candidate);
                next_candidate = u32_to_digit_vec(current_candidate_value + 4);
            } else {
                next_candidate[5] += 1;
            }
        }
        current_candidate = next_candidate.clone();
        passwords_checked += 1;
    }

    println!(
        "Found a total of {} potential passwords that fit the pattern. {} passwords checked.",
        potential_passwords_found, passwords_checked
    );
}
