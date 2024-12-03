use std::fs::File;
use std::io::{self, BufRead};

fn check_report(nums: Vec<i32>) -> i32 {
    // If there's less than two numbers, it's trivially monotonic
    if nums.len() < 2 {
        return 1;
    }

    // Determine the initial direction of the sequence (ascending or descending)
    let initial_diff = nums[1] - nums[0];

    // Check if the initial difference is valid
    if initial_diff == 0 || initial_diff.abs() > 3 {
        return 0;
    }

    let is_increasing = initial_diff > 0;

    // 1 3 2 5 6
    // 1 3 2 -> fallback
    // 3 2 5
    // 2 5 6 return 1

    // Iterate over adjacent pairs and check the conditions
    for window in nums.windows(2) {
        let diff = window[1] - window[0];

        // Differences must be within -3 to 3 and not zero
        if diff == 0 || diff.abs() > 3 || ((diff > 0) != is_increasing) {
            return 0;
        }
    }

    1
}

fn check_reports_with_removing(report: String) -> i32 {
    // Parse the report into a vector of integers
    let nums: Vec<i32> = report
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut valid_reports = 0;

    for i in 0..nums.len() {
        let mut report = nums.clone();
        report.remove(i);
        valid_reports += check_report(report)
    }

    std::cmp::min(1, valid_reports)
}

fn main() {
    // Load File
    let file = File::open("./input.txt".to_string()).unwrap();

    let mut valid_reports: i32 = 0;

    // Iterate over Lines
    for line in io::BufReader::new(file).lines().flatten() {
        valid_reports += check_reports_with_removing(line);
    }

    println!("There are {:?} valid reports", valid_reports);
}
