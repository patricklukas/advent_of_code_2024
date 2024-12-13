use regex::Regex;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input").expect("There has to be an input file");
    let now = Instant::now();
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let mut part_1 = 0;
    let mut part_2 = 0;

    // Iterate over matches and extract data
    for caps in re.captures_iter(&input) {
        let ax: i64 = caps[1].parse().unwrap();
        let ay: i64 = caps[2].parse().unwrap();
        let bx: i64 = caps[3].parse().unwrap();
        let by: i64 = caps[4].parse().unwrap();
        let px: i64 = caps[5].parse().unwrap();
        let py: i64 = caps[6].parse().unwrap();

        part_1 += calc_linear_combination(px, py, ax, ay, bx, by);
        part_2 += calc_linear_combination(px + 10000000000000, py + 10000000000000, ax, ay, bx, by);
    }

    println!("Part 1: {} Part 2: {}", part_1, part_2);
    println!("Took {:?}", now.elapsed());
}

fn calc_linear_combination(px: i64, py: i64, ax: i64, ay: i64, bx: i64, by: i64) -> i64 {
    let det = ax * by - ay * bx;
    if det == 0 {
        return 0;
    }

    let num_a = px * by - py * bx;
    let num_b = py * ax - px * ay;

    // Check if integer solution exists
    if num_a % det != 0 || num_b % det != 0 {
        return 0;
    }

    let a = num_a / det;
    let b = num_b / det;

    if a >= 0 && b >= 0 {
        return 3 * a + b;
    }
    0
}
