use std::fs;
use std::iter::repeat;

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

fn part_2(mut files: Vec<i32>, mut free: Vec<i32>) -> u64 {
    let mut moved_files: Vec<Vec<(i32, i32)>> = vec![Vec::new(); files.len()];

    // Move files last to first
    let mut i = files.len() - 1;
    while i > 0 {
        // Attempt to find a free segment left to right that can fit this entire file
        let mut j = 0;
        while j < i {
            if free[j] >= files[i] && files[i] > 0 {
                // Reduce the free segment by the size of the moved file
                free[j] -= files[i];

                // Record the move
                moved_files[j].push((i as i32, files[i]));

                // The original file segment is now empty and expands the free space in front of it
                free[i - 1] += files[i];
                files[i] = 0;
                break;
            }
            j += 1;
        }
        i -= 1;
    }

    // Reconstruct the disk layout
    let mut disk = Vec::new();
    for (i, &fcount) in files.iter().enumerate() {
        // Unmoved file blocks
        disk.extend(repeat(i as i32).take(fcount as usize));

        // Moved files in this segment
        for &(idx, count) in &moved_files[i] {
            disk.extend(repeat(idx).take(count as usize));
        }

        // Remaining free space
        disk.extend(repeat(0).take(free[i] as usize));
    }

    // Compute the checksum
    disk.iter()
        .enumerate()
        .map(|(pos, &file_id)| pos as u64 * file_id as u64)
        .sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("There has to be an input file");
    let chars: Vec<_> = input.trim().chars().collect();

    let mut files = Vec::with_capacity(chars.len() / 2);
    let mut free = Vec::with_capacity(chars.len() / 2);
    let mut disk = Vec::new();

    for (i, &c) in chars.iter().enumerate() {
        let num = c.to_digit(10).unwrap() as i32;
        if i % 2 == 0 {
            files.push(num);
            // Add 1 to file idx so we don't confuse it with free space
            disk.extend(repeat(i / 2 + 1).take(num as usize));
        } else {
            free.push(num);
            disk.extend(repeat(0).take(num as usize));
        }
    }
    // Cheeky push of an empty free space so files and free are the same size
    free.push(0);

    use std::time::Instant;

    let now = Instant::now();
    println!("Checksum Part 1: {:?}", part_1(disk));
    println!("Took {:?}", now.elapsed());

    let now = Instant::now();
    println!("Checksum Part 2: {:?}", part_2(files, free));
    println!("Took {:?}", now.elapsed());
}
