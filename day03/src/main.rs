use regex::Regex;
use std::fs;

fn main() {
    // Load File to string
    let hay = fs::read_to_string("./input.txt").expect("There has to be an input file");

    // Define Regex for valid mul instructions
    // Also match do() and don't()
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    let caps: Vec<&str> = re.find_iter(&hay).map(|m| m.as_str()).collect();

    let mut sum = 0;
    let mut multiplication_enabled = true;

    for cap in &caps {
        // Capture the "n" in "don't()"
        if cap.contains("n") {
            multiplication_enabled = false;
        // Capture the "d" in "do()"
        } else if cap.contains("d") {
            multiplication_enabled = true;
        // Else it has to be "mul(l,r)"
        } else if multiplication_enabled {
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
