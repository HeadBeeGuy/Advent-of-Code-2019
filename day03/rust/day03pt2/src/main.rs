// Advent of Code 2019 Day 3 part 2

// The general explanation for this is in part 1.
// Fortunately I didn't have to modify the part 1 code very much.
// I changed the graph so that now every entry is just the number of steps the
// wire in part 1 took to get to that position.

use std::fs;

const OFFSET: i32 = 50_000;
const MAXSIZE: i32 = 100_000;

fn main() {
    let wire_file =
        fs::read_to_string("../../input.txt").expect("The path to the input file is invalid.");
    let mut wire_paths = wire_file.lines();
    // Only two wires to worry about - at least in part 1!
    let first_wire: Vec<&str> = wire_paths
        .next()
        .expect("The first wire path isn't in the file.")
        .trim()
        .split(",")
        .collect();
    let second_wire: Vec<&str> = wire_paths
        .next()
        .expect("The second wire path isn't in the file.")
        .trim()
        .split(",")
        .collect();

    use ndarray::Array2;
    let mut graph = Array2::<i32>::zeros((MAXSIZE as usize, MAXSIZE as usize));
    let mut current_x: i32 = OFFSET;
    let mut current_y: i32 = OFFSET;
    let mut first_wire_steps: i32 = 0;
    let mut second_wire_steps: i32 = 0;
    let mut minimum_steps = i32::max_value();
    let mut minimum_step_position = (OFFSET, OFFSET);

    for instruction in first_wire {
        let mut delta_x: i32 = 0;
        let mut delta_y: i32 = 0;
        let (direction, distance) = instruction.split_at(1);
        let parsed_distance: u32 = distance
            .parse::<u32>()
            .expect("Encountered a non-standard distance in a direction.");
        match direction {
            "U" => delta_y = 1,
            "D" => delta_y = -1,
            "R" => delta_x = 1,
            "L" => delta_x = -1,
            _ => panic!("Encountered non-standard direction: {}", direction),
        }

        for _ in 1..=parsed_distance {
            first_wire_steps += 1;
            current_x += delta_x;
            current_y += delta_y;
            // if this is the second time we've traversed this spot, leave it alone
            if graph[[current_x as usize, current_y as usize]] == 0 {
                graph[[current_x as usize, current_y as usize]] = first_wire_steps;
            }
        }
    }

    current_x = OFFSET;
    current_y = OFFSET;
    for instruction in second_wire {
        let mut delta_x: i32 = 0;
        let mut delta_y: i32 = 0;
        let (direction, distance) = instruction.split_at(1);
        let parsed_distance: u32 = distance
            .parse::<u32>()
            .expect("Encountered a non-standard distance in a direction.");
        match direction {
            "U" => delta_y = 1,
            "D" => delta_y = -1,
            "R" => delta_x = 1,
            "L" => delta_x = -1,
            _ => panic!("Encountered non-standard direction: {}", direction),
        }

        for _ in 1..=parsed_distance {
            second_wire_steps += 1;
            current_x += delta_x;
            current_y += delta_y;
            if graph[[current_x as usize, current_y as usize]] != 0 {
                let combined_steps =
                    graph[[current_x as usize, current_y as usize]] + second_wire_steps;
                if combined_steps < minimum_steps {
                    minimum_steps = combined_steps;
                    minimum_step_position = (current_x, current_y);
                }
            }
        }
    }

    println!(
        "The point at {},{} has the minimum number of combined steps: {}.",
        minimum_step_position.0, minimum_step_position.1, minimum_steps
    );
}
