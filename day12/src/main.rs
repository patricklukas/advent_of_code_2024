use std::time::Instant;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("input").expect("There has to be an input file");
    let now = Instant::now();
    let map: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
    let cols = (map.len() as f64).sqrt() as usize;
    let mut visited = HashSet::with_capacity(map.len());

    let mut part_1 = 0;
    let mut part_2 = 0;
    for i in 0..map.len() {
        if !visited.contains(&i) {
            let res = flood_fill(i, cols, &map, &mut visited);
            part_1 += res.0;
            part_2 += res.1;
        }
    }

    println!("Part 1: {:?} Part 2: {:?}", part_1, part_2);
    println!("Took {:?}", now.elapsed());
}

fn flood_fill(
    idx: usize,
    cols: usize,
    map: &Vec<char>,
    visited: &mut HashSet<usize>,
) -> (u32, u32) {
    let mut s = VecDeque::from([idx]);
    let c = map[idx];

    let mut area = 0;
    let mut perimeter = 0;
    let mut sides = 0;

    while let Some(cur) = s.pop_front() {
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);
        area += 1;

        sides += count_corners(cur, cols, &map);

        let neighbors = [
            cur.checked_sub(1).filter(|&n| n / cols == cur / cols),
            cur.checked_add(1).filter(|&n| n / cols == cur / cols),
            cur.checked_sub(cols),
            cur.checked_add(cols),
        ];

        for &neighbor in neighbors.iter() {
            if let Some(n) = neighbor {
                if n >= map.len() || map[n] != c {
                    perimeter += 1;
                } else if map[n] == c {
                    s.push_back(n);
                }
            } else {
                perimeter += 1;
            }
        }
    }

    (area * perimeter, area * sides)
}

fn count_corners(idx: usize, cols: usize, map: &Vec<char>) -> u32 {
    let c = map[idx]; // Current cell character

    let mut corners = 0;

    // Function to safely get the character of a neighbor or return a sentinel value for out-of-bounds
    let neighbor_char = |neighbor: Option<usize>| -> char {
        neighbor.map_or('#', |n| map.get(n).copied().unwrap_or('#')) // Use '#' as the out-of-bounds marker
    };

    // Neighbors in 8 directions
    let neighbors = [
        idx.checked_sub(1).filter(|&n| n / cols == idx / cols), // Left
        idx.checked_sub(cols),                                  // Up
        idx.checked_add(1)
            .filter(|&n| n / cols == idx / cols && n < map.len()), // Right
        idx.checked_add(cols).filter(|&n| n < map.len()),       // Down
        idx.checked_sub(cols + 1)
            .filter(|&n| n / cols + 1 == idx / cols), // Up-Left
        idx.checked_sub(cols - 1)
            .filter(|&n| n / cols + 1 == idx / cols), // Up-Right
        idx.checked_add(cols - 1)
            .filter(|&n| n >= cols && n / cols - 1 == idx / cols && n < map.len()), // Down-Left
        idx.checked_add(cols + 1)
            .filter(|&n| n >= cols && n / cols - 1 == idx / cols && n < map.len()), // Down-Right
    ];

    // Loop through neighbors in pairs to detect corners
    for &(n1, n2, n3) in [
        (0, 4, 1), // Left + Up-Left + Up
        (1, 5, 2), // Up + Up-Right + Right
        (2, 7, 3), // Right + Down-Right + Down
        (3, 6, 0), // Down + Down-Left + Left
    ]
    .iter()
    {
        let char_1 = neighbor_char(neighbors[n1]);
        let char_2 = neighbor_char(neighbors[n2]);
        let char_3 = neighbor_char(neighbors[n3]);

        // Check for corners: adjacent cells differ from the current cell
        if char_1 != c && char_3 != c {
            corners += 1;
        }
        if char_1 == c && char_3 == c && char_2 != c {
            corners += 1;
        }
    }

    corners
}
