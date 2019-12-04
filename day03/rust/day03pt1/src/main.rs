// I found the ndarray crate via this reddit comment:
// https://www.reddit.com/r/rust/comments/a76ylp/how_to_push_values_to_a_2_dimensional_vector_in/ec1c6di/

// The idea: trace out the path of the first wire as a bunch of "true" values
// in an enormous 2d graph full of "false" values.
// Then, trace out the second path, noting every point at which there was already
// a "true" value. This is an intersection point.
// All we need to do in part 1 is just find the closest of those to the origin!

use std::fs;

// We can't use negative indices, so all coordinates will be pumped up by this
// offset. The origin point becomes [OFFSET, OFFSET]
const OFFSET: i32 = 50_000;
// The graph size needs to be declared ahead of time, so I made it enormous.
// This is inefficient, of course - a better approach would be to make the graph
// only as big as it absolutely needs to be, and putting the offset right in the
// middle of that.
const MAXSIZE: i32 = 100_000;

fn manhattan_distance_to_origin(x: i32, y: i32) -> i32 {
    (x - OFFSET).abs() + (y - OFFSET).abs()
}

fn main() {
    let wire_file = fs::read_to_string("../../input.txt")
        .expect("The path to the input file is invalid.");
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
    let mut graph = Array2::from_elem((MAXSIZE as usize, MAXSIZE as usize), false);
    let mut current_x: i32 = OFFSET;
    let mut current_y: i32 = OFFSET;
    let mut shortest_distance: i32 = i32::max_value();

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
            current_x += delta_x;
            current_y += delta_y;
            graph[[current_x as usize, current_y as usize]] = true;
        }
    }

    // I'm inefficiently repeating the traversal code instead of putting it into
    // a function. It doesn't need to modify the graph, just check to see if the
    // first traversal visited this spot or not.
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
            current_x += delta_x;
            current_y += delta_y;
            if graph[[current_x as usize, current_y as usize]] == true {
                let manhattan_distance = manhattan_distance_to_origin(current_x, current_y);
                if manhattan_distance < shortest_distance {
                    shortest_distance = manhattan_distance;
                }
            }
        }
    }
    
    println!("The closest intersection to the origin has a distance of {}.", shortest_distance);
}
