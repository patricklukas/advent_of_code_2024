use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn load_list_file(f: String) -> String {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    contents
}

fn split_file(content: String, left: &mut Vec<u32>, right: &mut Vec<u32>) {
    for line in content.lines() {
        match line.split_once("   ") {
            None => panic!(),
            Some((l, r)) => {
                left.push(l.parse::<u32>().unwrap());
                right.push(r.parse::<u32>().unwrap());
            }
        }
    }
}

fn calc_distance(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    let mut distance = 0;
    for i in 0..left.len() {
        distance += left[i].abs_diff(right[i]);
    }
    distance
}

fn calc_counts(num_vec: &Vec<u32>) -> HashMap<u32, u32> {
    let mut num_counts: HashMap<u32, u32> = HashMap::new();

    for n in num_vec {
        *num_counts.entry(*n).or_insert(0) += 1;
    }
    num_counts
}

fn main() {
    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();

    let liste: String = load_list_file("./input.txt".to_string());

    split_file(liste, &mut left, &mut right);

    left.sort_unstable();
    right.sort_unstable();

    let result = calc_distance(&left, &right);

    let counts = calc_counts(&right);

    let mut similarity: u32 = 0;

    for n in left {
        similarity += n * counts.get(&n).unwrap_or(&0);
    }

    println!("The distance is: {:?}", result);
    println!("the similarity score is: {:?}", similarity)
}
