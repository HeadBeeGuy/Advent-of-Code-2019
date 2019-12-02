// This is probably not good Rust at all - reading vectors with no error handling!
// There must be a more elegant way to do it! But it finds the solution very
// quickly nonetheless.
use std::fs;

fn main() {
    let base_opcodes = fs::read_to_string("../../input.txt").expect("You got the file path wrong!");
    let base_opcodes: Vec<u32> = base_opcodes
        .trim()
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut opcodes = base_opcodes.clone();

            opcodes[1] = noun;
            opcodes[2] = verb;

            let mut current_index = 0;
            loop {
                let operation = opcodes[current_index];
                if operation == 1 {
                    let first_index = opcodes[current_index + 1] as usize;
                    let second_index = opcodes[current_index + 2] as usize;
                    let sum = opcodes[first_index] + opcodes[second_index];
                    let save_here = opcodes[current_index + 3] as usize;
                    opcodes[save_here] = sum;
                } else if operation == 2 {
                    let first_index = opcodes[current_index + 1] as usize;
                    let second_index = opcodes[current_index + 2] as usize;
                    let product = opcodes[first_index] * opcodes[second_index];
                    let save_here = opcodes[current_index + 3] as usize;
                    opcodes[save_here] = product;
                } else if operation == 99 {
                    break;
                }

                current_index += 4;
            }

            if opcodes[0] == 19690720 {
                println!("Found the correct pair! It's {} and {}.", noun, verb);
                println!("100 * noun + verb is: {}", (100 * noun + verb));
                break;
            }
        }
    }
}
