use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::with_capacity(stones.len());
    for (k, v) in stones {
        let num = format!("{}", k);
        match k {
            0 => *new_stones.entry(1).or_default() += v,
            1 => *new_stones.entry(2024).or_default() += v,
            _ => {
                if num.len() % 2 > 0 {
                    *new_stones.entry(2024 * k).or_default() += v;
                } else {
                    let l: u64 = num[..num.len() / 2].parse().unwrap();
                    let r: u64 = num[num.len() / 2..].parse().unwrap();
                    *new_stones.entry(l).or_default() += v;
                    *new_stones.entry(r).or_default() += v;
                }
            }
        };
    }
    new_stones
}

fn main() {
    let input = fs::read_to_string("input").expect("There has to be an input file");
    let now = Instant::now();
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for s in input.split_whitespace() {
        let num: u64 = s.parse().expect("Input contains a non-integer value");
        *stones.entry(num).or_default() += 1;
    }

    let blinks = 75;
    for _ in 0..blinks {
        stones = blink(stones);
    }
    let num_stones: u64 = stones.values().sum();

    println!("Number of stones after {} blinks: {}", blinks, num_stones);
    println!("Took {:?}", now.elapsed());
}
