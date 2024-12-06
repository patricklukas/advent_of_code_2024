use std::{collections::HashSet, fs};

fn next_dir(x: i32, y: i32) -> (i32, i32) {
    match (x, y) {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => (0, 0),
    }
}

fn idx(x: usize, y: usize, ncol: usize) -> usize {
    y * ncol + x
}

fn traverse(
    mut x: i32,
    mut y: i32,
    mut map: Vec<char>,
    ncol: usize,
    nrow: usize,
    get_visited: bool,
) -> Option<HashSet<usize>> {
    let max_steps = map.len();
    let mut steps = 0;
    let mut visited: HashSet<usize> = Default::default();
    let mut dir = (0, -1);

    loop {
        let next_x = x + dir.0;
        let next_y = y + dir.1;

        if next_x >= ncol as i32 || next_y >= nrow as i32 || next_x < 0 || next_y < 0 {
            return Some(visited);
        }

        let next_idx = idx(next_x as usize, next_y as usize, ncol);

        match *map.get(next_idx).unwrap() {
            '#' => {
                dir = next_dir(dir.0, dir.1);
            }
            c => {
                map[next_idx] = 'X';
                x = next_x;
                y = next_y;

                if c == '.' && get_visited {
                    visited.insert(next_idx);
                }
            }
        }
        steps += 1;
        if steps > max_steps {
            break;
        }
    }

    None
}

fn main() {
    // Load File to string
    let input = fs::read_to_string("./input").expect("There has to be an input file");

    let map: Vec<char> = input
        .chars()
        .filter(|c| *c == '#' || *c == '.' || *c == '^')
        .collect();

    let nrow = input.lines().count();
    let ncol = map.len() / nrow;

    let guard_index = map.iter().position(|&c| c == '^').unwrap();

    let guard_x = guard_index.rem_euclid(ncol) as i32;
    let guard_y = (guard_index / ncol) as i32;

    let visited = traverse(guard_x, guard_y, map.clone(), ncol, nrow, true).unwrap_or_default();

    let mut loops = 0;

    for i in visited.iter() {
        if *map.get(*i).unwrap() == '.' {
            let mut new_map = map.clone();
            new_map[*i] = '#';

            match traverse(guard_x, guard_y, new_map, ncol, nrow, false) {
                None => loops += 1,
                _ => {}
            }
        }
    }

    println!("Visited {:?} Positions", visited.len() + 1);
    println!(
        "Found {:?} Obstacle Positions that would generate a loop",
        loops
    )
}
