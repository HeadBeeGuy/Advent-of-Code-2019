// Advent of Code Day 8 part 2

// This one was straightforward! I used my old buddy, the HashMap, which I used
// with aplomb last year when I did everything in Ruby.

use std::collections::HashMap;
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

    let width = 25;
    let height = 6;

    let mut final_image: HashMap<(i8, i8), char> = HashMap::new();

    // Assure that the data corresponds to complete image layers
    assert_eq!((image_data.len() % (width * height)), 0);

    // This constructs images from the top layer down, so if there's already a 
    // color in a pixel, it never gets replaced - it's below another colored pixel.

    for image in image_data.chunks(width * height) {
        let mut current_x;
        let mut current_y = 1;

        for vertical_line in image.chunks(width) {
            current_x = 1;
            for pixel in vertical_line.iter() {
                match *pixel {
                    '0' => { // black 
                        final_image.entry((current_x, current_y)).or_insert(' ');
                    },
                    '1' => { // white
                        final_image.entry((current_x, current_y)).or_insert('X');
                    },
                    '2' => {}, // transparent - do nothing
                    _ => println!("Found a corrupted pixel: {}", pixel)
                }
                current_x += 1;
            }

            current_y += 1;
        }
    }

    for y in 1..=height {
        for x in 1..=width {
            match final_image.get(&(x as i8, y as i8)) {
                Some(v) => print!("{}", v),
                None => print!(" ")
            }
        }
        print!("\n");
    }
    print!("\n");
}
