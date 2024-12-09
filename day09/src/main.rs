use std::fs;
use std::iter::repeat;
use std::time::Instant;

fn part_1(mut disk: Vec<usize>) -> u64 {
    let mut l = 0;
    let mut r = disk.len() - 1;

    while l < r {
        if disk[l] == 0 {
            while l < r && disk[r] == 0 {
                r -= 1;
            }
            disk.swap(l, r);
        }
        l += 1;
    }

    disk.iter()
        .enumerate()
        .map(|(pos, &file_id)| (pos * (file_id.saturating_sub(1))) as u64)
        .sum()
}

fn part_2(mut data: Vec<(usize, usize)>) -> u64 {
    // Store last looked index for each data size
    let mut idx = [0_usize; 10];
    let mut i = data.len() - 1;

    while i > *idx.iter().min().unwrap() {
        // Look for data
        if data[i].0 != 0 {
            let size = data[i].1;
            while i > idx[size] {
                // Look for empty space large enough to fit data
                if data[idx[size]].0 == 0 && data[idx[size]].1 >= size {
                    data[idx[size]].1 -= size;
                    data.insert(idx[size], data[i]);
                    data[i + 1].0 = 0;
                    break;
                }
                idx[size] += 1;
            }
        }
        i -= 1;
    }

    // Fancy functional way of creating our checksum
    data.iter()
        .flat_map(|&(d, s)| repeat(d.saturating_sub(1)).take(s))
        .enumerate()
        .map(|(pos, file_id)| (pos * file_id) as u64)
        .sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("There has to be an input file");
    let chars: Vec<_> = input.trim().chars().collect();

    let mut disk = Vec::new();
    let mut data = Vec::new();

    for (i, &c) in chars.iter().enumerate() {
        let num = c.to_digit(10).unwrap() as i32;
        if i % 2 == 0 {
            // Add 1 to file idx so we don't confuse it with free space
            disk.extend(repeat(i / 2 + 1).take(num as usize));
            data.push((i / 2 + 1, num as usize));
        } else {
            disk.extend(repeat(0).take(num as usize));
            data.push((0, num as usize));
        }
    }

    let mut now = Instant::now();
    println!("Checksum Part 1: {:?}", part_1(disk));
    println!("Took {:?}", now.elapsed());

    now = Instant::now();
    println!("Checksum Part 2: {:?}", part_2(data));
    println!("Took {:?}", now.elapsed());
}
