use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("There has to be an input file");

    // bfs flood fill that returns the perimeter of each cell and the area filled
    // use a queue
    // actually can we just look at the char and then sum perimeter and area?

    let map: Vec<u32> = input.chars().flat_map(|ch| ch.to_digit(10)).collect();
}
