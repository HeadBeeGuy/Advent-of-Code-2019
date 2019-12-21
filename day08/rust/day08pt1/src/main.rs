// Advent of Code Day 8 part 1
// I tried this a few different ways, including very bare-bones for loops, but
// they kept messing up. Then I discovered the chunk method, which was exactly
// what I was looking for!

use std::{env, fs, process};

fn process_arguments(args: &[String]) -> Result<Vec<char>, &str> {
    if args.len() != 2 {
        return Err("Argument error: Please run the program and specify the input file.");
    }

    let image_file =
        fs::read_to_string(&args[1]).expect("The path to the input file was not valid.");
    Ok(image_file.trim().chars().collect())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_data = process_arguments(&args).unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1);
    });

    // This seems to be part of the standard problem definition, so I won't bother
    // making them variable.
    let width = 25;
    let height = 6;

    // Assure that the data corresponds to complete images
    assert_eq!((image_data.len() % (width * height)), 0);

    let mut fewest_zero_count = usize::max_value();
    let mut fewest_zeros_formula = 0;

    for image in image_data.chunks(width * height) {
        let zero_count = image.iter().filter(|item| **item == '0').count();
        if zero_count < fewest_zero_count {
            fewest_zeros_formula = image.iter().filter(|item| **item == '1').count()
                * image.iter().filter(|item| **item == '2').count();
            fewest_zero_count = zero_count;
        }
    }

    println!(
        "The minimum number of zeroes on any page was {}. The number of ones times the number of twos was {}.",
        fewest_zero_count, fewest_zeros_formula
    );
}
