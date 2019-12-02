use std::fs;

fn main() {
    let opcodes = fs::read_to_string("../../modified_input.txt").expect("You got the file path wrong!");

    let mut opcodes: Vec<u32> = opcodes.trim().split(",").map(|item| item.parse().unwrap()).collect();

    let mut current_index = 0;
    loop {
        let operation = opcodes[current_index];
        if operation == 1 {
            let first_index = opcodes[current_index + 1] as usize;
            let second_index = opcodes[current_index + 2] as usize;
            let sum = opcodes[first_index] + opcodes[second_index];
            let save_here = opcodes[ current_index + 3] as usize;
            opcodes[save_here] = sum;
        } else if operation == 2 {
            let first_index = opcodes[current_index + 1] as usize;
            let second_index = opcodes[current_index + 2] as usize;
            let product = opcodes[first_index] * opcodes[second_index];
            let save_here = opcodes[ current_index + 3] as usize;
            opcodes[save_here] = product;
        } else if operation == 99 {
            break;
        }
        
        current_index += 4;
    }

    println!("Value at index 0: {}", opcodes[0]);
}
