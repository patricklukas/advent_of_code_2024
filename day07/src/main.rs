use rayon::prelude::*;
use std::fs;

fn main() {
    // Load File to string
    let input = fs::read_to_string("./input").expect("There has to be an input file");

    let solution: i64 = input
        .par_lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap();

            let target: i64 = left.trim().parse().unwrap();

            if check_equation(
                target,
                &right
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<i64>>(),
            ) {
                target
            } else {
                0
            }
        })
        .sum();

    println!("The solution is: {:?}", solution);
}

fn check_equation(target: i64, operands: &[i64]) -> bool {
    fn recurse(target: i64, current: i64, rest: &[i64]) -> bool {
        if rest.is_empty() {
            return current == target;
        }

        // Prune search if accumulated value is larger that the target
        // Should be safe since input are all > 0
        if target < current {
            return false;
        }

        let next = rest[0];
        let remaining = &rest[1..];

        // Try multiplication
        if recurse(target, current * next, remaining) {
            return true;
        }

        // Try concatenation
        if recurse(
            target,
            format!("{:?}{:?}", current, next).parse().unwrap(),
            remaining,
        ) {
            return true;
        }

        // Try addition
        if recurse(target, current + next, remaining) {
            return true;
        }

        false
    }

    recurse(target, operands[0], &operands[1..])
}
