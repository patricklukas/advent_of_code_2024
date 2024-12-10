use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = fs::read_to_string("input").expect("There has to be an input file");
    let map: Vec<u32> = input.chars().flat_map(|ch| ch.to_digit(10)).collect();

    // Input is square
    let cols = (map.len() as f64).sqrt() as usize;

    let mut paths: HashMap<usize, usize> = HashMap::new();
    let mut peaks: HashMap<usize, HashSet<usize>> = HashMap::new();

    let res: (usize, usize) = map
        .iter()
        .enumerate()
        .filter(|&(_, &n)| n == 0) // Consider only trailheads (height 0)
        .map(|(i, _)| dfs(0, i, cols, &map, &mut peaks, &mut paths))
        .fold((0, 0), |acc, (a, b)| (acc.0 + a, acc.1 + b));

    println!("Part 1: {:?}\nPart 2: {:?}", res.0, res.1);
    println!("Took {:?}", now.elapsed());
}

fn dfs(
    val: u32,
    i: usize,
    cols: usize,
    map: &Vec<u32>,
    peaks: &mut HashMap<usize, HashSet<usize>>,
    paths: &mut HashMap<usize, usize>,
) -> (usize, usize) {
    // If already visited return
    if let Some(&path) = paths.get(&i) {
        return (1, path);
    }

    // If this is a peak, create entries and return
    if val == 9 {
        peaks.insert(i, HashSet::from([i]));
        paths.insert(i, 1);
        return (1, 1); // A peak contributes 1 to the score
    }

    let neighbors = [
        i.checked_sub(1).filter(|_| i % cols > 0),
        i.checked_add(1).filter(|_| i % cols < cols - 1),
        i.checked_sub(cols),
        i.checked_add(cols),
    ];

    let mut local_peaks = HashSet::new();
    let mut local_paths = 0;

    for idx in neighbors.into_iter().filter_map(|idx| idx) {
        if let Some(&neighbor_val) = map.get(idx) {
            if neighbor_val == val + 1 {
                // Recursive DFS call
                local_paths += dfs(neighbor_val, idx, cols, map, peaks, paths).1;

                // Collect peaks reachable from neighbors
                if let Some(neighbor_peaks) = peaks.get(&idx) {
                    local_peaks.extend(neighbor_peaks);
                }
            }
        }
    }
    // Update visited maps
    peaks.insert(i, local_peaks);
    paths.insert(i, local_paths);

    (peaks[&i].len(), local_paths)
}
