use std::fs;

enum States {
    INITIAL,
    X,
    M,
    A,
}

impl States {
    pub fn reset(&mut self) {
        *self = States::INITIAL;
    }

    pub fn transition(&mut self, c: char) -> i32 {
        match *self {
            States::INITIAL => {
                if c == 'X' {
                    *self = States::X;
                }
            }
            States::X => {
                if c == 'M' {
                    *self = States::M;
                } else if c == 'X' {
                    *self = States::X;
                } else {
                    self.reset();
                }
            }
            States::M => {
                if c == 'A' {
                    *self = States::A
                } else if c == 'X' {
                    *self = States::X;
                } else {
                    self.reset();
                }
            }
            States::A => {
                if c == 'S' {
                    self.reset();
                    return 1;
                } else if c == 'X' {
                    *self = States::X;
                } else {
                    self.reset();
                }
            }
        };
        0
    }
}

fn idx(x: usize, y: usize, ncol: usize) -> usize {
    y * ncol + x
}

fn main() {
    // Load File to string
    let input = fs::read_to_string("./input.txt").expect("There has to be an input file");

    // Build vec from String
    let nrow = input.lines().count();
    let ncol = input.lines().next().unwrap().chars().count();
    println!("Found {} rows and {} columns", nrow, ncol);

    // This is a bit hacky, since I just looked up the num of lines and chars
    // const NROW: usize = 140;
    // const NCOL: usize = 140;
    let matrix: Vec<char> = input
        .chars()
        .filter(|c| *c == 'X' || *c == 'M' || *c == 'A' || *c == 'S')
        .collect();

    let mut xmas_count = 0;

    for y in 0..nrow {
        // States for each Direction
        let mut state_h: States = States::INITIAL;
        let mut state_h_bw: States = States::INITIAL;
        let mut state_v: States = States::INITIAL;
        let mut state_v_bw: States = States::INITIAL;
        let mut state_tlbr: States = States::INITIAL;
        let mut state_tlbr_bw: States = States::INITIAL;
        let mut state_bltr: States = States::INITIAL;
        let mut state_bltr_bw: States = States::INITIAL;

        let mut reset_d = false;
        let mut reset_d_bw = false;

        for x in 0..ncol {
            // State Transitions and count found Words
            // Horizontal
            xmas_count += state_h.transition(matrix[idx(x, y, ncol)]);
            // Vertical -> Switch x and y
            xmas_count += state_h_bw.transition(matrix[idx(y, x, ncol)]);
            // Horizontal Backwards
            xmas_count += state_v.transition(matrix[idx(ncol - x - 1, y, ncol)]);
            // Vertical Backwards
            xmas_count += state_v_bw.transition(matrix[idx(y, nrow - x - 1, ncol)]);

            // State Reset on Wrapping
            if x + y >= ncol && !reset_d {
                state_tlbr.reset();
                state_bltr.reset();
                reset_d = true;
            }

            // Calc diagonal x coords
            let diag_x_bw: i64 = ncol as i64 - 1 - x as i64 - y as i64;
            if diag_x_bw < 0 && !reset_d_bw {
                state_tlbr_bw.reset();
                state_bltr_bw.reset();
                reset_d_bw = true;
            }

            // Diagonal Top Left -> Bottom Right
            xmas_count += state_tlbr.transition(matrix[idx((x + y) % ncol, x, ncol)]);
            // Diagonal Bottom Left -> Top Right
            xmas_count += state_bltr.transition(matrix[idx((x + y) % ncol, ncol - x - 1, ncol)]);

            // Diagonal Top Left -> Bottom Right Backwards
            xmas_count += state_tlbr_bw.transition(
                matrix[idx(
                    ((diag_x_bw).rem_euclid(ncol as i64)) as usize,
                    ncol - 1 - x,
                    ncol,
                )],
            );
            // Diagonal Bottom Left -> Top Right Backwards
            xmas_count += state_bltr_bw
                .transition(matrix[idx(((diag_x_bw).rem_euclid(ncol as i64)) as usize, x, ncol)]);
        }
    }

    println!("Found XMAS {:?} times!", xmas_count);

    // Now let's find X-MAS!
    let mut cross_mas_count: i32 = 0;

    for y in 0..nrow - 2 {
        for x in 0..ncol - 2 {
            let word_tlbr = format!(
                "{}{}{}",
                matrix[idx(x, y, ncol)],
                matrix[idx(x + 1, y + 1, ncol)],
                matrix[idx(x + 2, y + 2, ncol)]
            );
            let word_bltr = format!(
                "{}{}{}",
                matrix[idx(x, y + 2, ncol)],
                matrix[idx(x + 1, y + 1, ncol)],
                matrix[idx(x + 2, y, ncol)]
            );

            if (word_tlbr == "MAS" || word_tlbr.chars().rev().collect::<String>() == "MAS")
                && (word_bltr == "MAS" || word_bltr.chars().rev().collect::<String>() == "MAS")
            {
                cross_mas_count += 1;
            }
        }
    }

    println!("Found {:?} X-MAS!", cross_mas_count);
}
