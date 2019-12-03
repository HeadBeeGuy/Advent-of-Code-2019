// one potential 2d array implementation: https://stackoverflow.com/a/59043086
// I found this crate via this reddit comment:
// https://www.reddit.com/r/rust/comments/a76ylp/how_to_push_values_to_a_2_dimensional_vector_in/ec1c6di/

// Current plan: trace out the path of the first wire as a bunch of "true" values
// in an enormous 2d graph full of "false" values.
// Then, trace out the second path, noting every point at which there was already
// a "true" value. This is an intersection point.
// Once all intersection points are known, find the one closest to the start point.

use std::fs;

// We can't ues negative indices, so all coordinates will be pumped up by this
// offset. There's probably a better way to do this.
static OFFSET: usize = 5;
static MAXSIZE: usize = 3;

fn main() {
    let wire_file = fs::read_to_string("../../tiny_test_input.txt").expect("You got the file path wrong!");
    let wire_paths = wire_file.lines();

    use ndarray::Array2;
    let mut graph = Array2::from_elem((MAXSIZE, MAXSIZE), false);
    graph[[1, 2]] = true;
    dbg!(graph.view());
}
