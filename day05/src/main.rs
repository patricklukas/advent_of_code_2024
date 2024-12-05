use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

// Implement a cmp method for Person
fn page_order(a: &i32, b: &i32, map: &HashMap<i32, Vec<i32>>) -> Ordering {
    if map.get(a).unwrap_or(&Vec::<i32>::new()).contains(b) {
        Ordering::Less
    } else if map.get(b).unwrap_or(&Vec::<i32>::new()).contains(a) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn main() {
    // Load File to string
    let input = fs::read_to_string("./input").expect("There has to be an input file");

    let (s_rules, s_queues) = input.split_once("\n\n").unwrap();

    let mut rules = HashMap::<i32, Vec<i32>>::new();

    for line in s_rules.lines() {
        let (k, v) = line.split_once("|").unwrap();

        rules
            .entry(k.parse().unwrap())
            .or_insert(Vec::<i32>::new())
            .push(v.parse().unwrap());
    }

    let mut result_part_one = 0;
    let mut result_part_two = 0;

    for line in s_queues.lines() {
        let mut queue = Vec::<i32>::new();
        for page in line.split(",") {
            queue.push(page.parse().unwrap());
        }
        if queue.is_sorted_by(|a, b| page_order(a, b, &rules) == Ordering::Less) {
            result_part_one += queue[queue.len() / 2];
        } else {
            queue.sort_by(|a, b| page_order(a, b, &rules));
            result_part_two += queue[queue.len() / 2];
        }
    }

    println!("Part One: The Result is: {:?}", result_part_one);
    println!("Part Two: The Result is: {:?}", result_part_two);
}
