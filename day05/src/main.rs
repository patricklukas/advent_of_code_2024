use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn page_order(a: &i32, b: &i32, map: &HashMap<i32, Vec<i32>>) -> Ordering {
    match (map.get(a), map.get(b)) {
        (Some(v_a), _) if v_a.contains(b) => Ordering::Less,
        (_, Some(v_b)) if v_b.contains(a) => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

fn main() {
    // Load File to string
    let input = fs::read_to_string("./input").expect("There has to be an input file");

    let (s_rules, s_queues) = input.split_once("\n\n").unwrap();

    let rules: HashMap<i32, Vec<i32>> = s_rules
        .lines()
        // Get all parseable lines as k, v pair
        .filter_map(|line| {
            let (k, v) = line.split_once('|')?;
            let key = k.trim().parse().ok()?;
            let value = v.trim().parse().ok()?;
            Some((key, value))
        })
        // Fold all k, v pairs into an accumulator and store it
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_default().push(v);
            acc
        });

    let mut result_part_one = 0;
    let mut result_part_two = 0;

    for line in s_queues.lines() {
        let mut queue: Vec<i32> = line
            .split(",")
            .map(|page| page.trim().parse().unwrap())
            .collect();

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
