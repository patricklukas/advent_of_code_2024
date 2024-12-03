use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Load File
    let mut hay = String::new();
    let _ = File::open("./input.txt".to_string())
        .unwrap()
        .read_to_string(&mut hay);

    // Define Regex for valid mul instructions
    // Also match do() and don't()
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    let caps: Vec<&str> = re.find_iter(&hay).map(|m| m.as_str()).collect();

    let mut sum = 0;
    let mut multiplication_enabled = true;

    for cap in &caps {
        if cap.contains("don") {
            multiplication_enabled = false;
            // println!("Multiplication Off!");
        } else if cap.contains("do") {
            multiplication_enabled = true;
            // println!("Multiplication On!");
        } else if multiplication_enabled {
            // println!("{}", cap);
            let (l, r) = cap.split_once(",").unwrap();
            let l: i32 = l
                .chars()
                .filter(|char| char.is_digit(10))
                .collect::<String>()
                .parse()
                .expect("This should be an integer");
            let r: i32 = r
                .chars()
                .filter(|char| char.is_digit(10))
                .collect::<String>()
                .parse()
                .expect("This should be an integer");

            sum += l * r;
        }
    }

    println!("The sum is {:?}", sum)
}
