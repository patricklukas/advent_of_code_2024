use gcd::euclid_u32;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn crd(idx: usize, ncol: usize) -> (i32, i32) {
    // Convert a linear index into (x, y) coordinates based on the number of columns.
    ((idx % ncol) as i32, (idx / ncol) as i32)
}

fn main() {
    let input = fs::read_to_string("input").expect("There has to be an input file");
    let chars: Vec<_> = input.chars().filter(|&c| c != '\n').collect();
    let rows = input.lines().count();
    let cols = chars.len() / rows;
    let (max_x, max_y) = (cols as i32, rows as i32);

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes = HashSet::new();
    let mut antinodes_p2 = HashSet::new();

    // A closure to check if a given coordinate is within the grid bounds
    let in_bounds = |(x, y): (i32, i32)| x >= 0 && y >= 0 && x < max_x && y < max_y;

    // A closure to insert a coordinate into a set only if it is in bounds
    let try_insert = |set: &mut HashSet<(i32, i32)>, p: (i32, i32)| {
        if in_bounds(p) {
            set.insert(p);
        }
    };

    // A closure to generate an iterator over points along a straight line starting
    // from `start` and moving in `step` increments (dx, dy). We use `take_while`
    // to stop when we move out of the grid bounds.
    let line_points = |start: (i32, i32), step: (i32, i32)| {
        (0..) // An infinite sequence of integers starting from 0
            .map(move |n| (start.0 + n * step.0, start.1 + n * step.1))
            .take_while(|&p| in_bounds(p))
    };

    chars
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c != '.') // We only care about antenna positions, skip '.'
        .for_each(|(i, &c)| {
            let pos = crd(i, cols);
            let positions = antennas.entry(c).or_default();

            // For each previously encountered antenna of the same type,
            // we identify "antinodes" based on their relative positions
            for &(o_x, o_y) in positions.iter() {
                let (dx, dy) = (o_x - pos.0, o_y - pos.1);

                // Direct antinodes at a distance of one step away from both antennas.
                try_insert(&mut antinodes, (o_x + dx, o_y + dy));
                try_insert(&mut antinodes, (pos.0 - dx, pos.1 - dy));

                // Reduce the direction vector (dx, dy) to its simplest form using gcd
                // This gives the direction step for the infinite line in part 2
                let div = euclid_u32(dx.abs() as u32, dy.abs() as u32) as i32;
                let step = (dx / div, dy / div);

                // Consider both forward (step) and backward (-step) directions.
                // Use an array and iterate over it to avoid code duplication.
                [step, (-step.0, -step.1)]
                    .iter()
                    .flat_map(|&st| line_points(pos, st)) // Generate points along each line for both directions
                    .for_each(|p| {
                        antinodes_p2.insert(p);
                    });
            }

            // Finally, record the current antenna position so future antennas of the same type
            // can be compared against it.
            positions.push(pos);
        });

    println!("Result of part 1 is {:?}", antinodes.len());
    println!("Result of part 2 is {:?}", antinodes_p2.len());
}
