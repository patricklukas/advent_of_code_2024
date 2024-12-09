use std::fs;
use std::iter::repeat;

fn part_1(files: &[i32], free: &[i32]) -> u64 {
    let mut files = files.to_vec();
    let free = free.to_vec();

    let mut fragmented = Vec::new();
    let mut idx_last_file = files.len() - 1;

    for (i, &free_block) in free.iter().enumerate() {
        // Move out current file blocks
        fragmented.extend(repeat(i).take(files[i] as usize));
        files[i] = 0;

        // Fill from the right with free space
        let mut still_free = free_block;
        while still_free > 0 && idx_last_file > 0 {
            let file_size = files[idx_last_file];
            if still_free >= file_size {
                // Use up the entire last segment
                fragmented.extend(repeat(idx_last_file).take(file_size as usize));
                still_free -= file_size;
                files[idx_last_file] = 0;

                if idx_last_file == 0 {
                    break;
                }
                idx_last_file -= 1;
            } else {
                // Partially fill with what's needed
                fragmented.extend(repeat(idx_last_file).take(still_free as usize));
                files[idx_last_file] -= still_free;
                still_free = 0;
            }
        }
    }

    fragmented
        .iter()
        .enumerate()
        .map(|(pos, &file_id)| pos as u64 * file_id as u64)
        .sum()
}

fn part_2(files: &Vec<i32>, free: &Vec<i32>) -> u64 {
    let mut files = files.clone();
    let mut free = free.clone();
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

    for (i, &c) in chars.iter().enumerate() {
        let num = c.to_digit(10).unwrap() as i32;
        if i % 2 == 0 {
            files.push(num);
        } else {
            free.push(num);
        }
    }

    // Cheeky push of an empty free space so files and free are the same size
    free.push(0);

    println!("Checksum Part 1: {:?}", part_1(&files, &free));
    println!("Checksum Part 2: {:?}", part_2(&files, &free));
}
