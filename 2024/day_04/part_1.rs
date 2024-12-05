use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    const INPUT_FILENAME: &str = "input";

    // Read input data from the file
    let input_data = {
        let mut f = File::open(INPUT_FILENAME)?;
        let mut data = vec![];
        f.read_to_end(&mut data)?;
        String::from_utf8_lossy(&data).to_string()
    };

    let occurrences = find_occurrences(&input_data);
    println!("Total occurrences of 'XMAS': {}", occurrences.len());
    // for (i, j, direction) in occurrences {
    //     println!("Found at: ({}, {}) {}", i, j, direction);
    // }

    Ok(())
}

fn find_occurrences(data: &str) -> Vec<(usize, usize, &str)> {
    let mut occurrences = Vec::new();
    let word = "XMAS";
    let lines: Vec<&str> = data.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    // Helper function to check for matches in all directions
    let check_direction = |start_i: usize, start_j: usize, dir_i: isize, dir_j: isize| {
        let mut chars = Vec::new();
        for k in 0..word.len() {
            let new_i = start_i as isize + k as isize * dir_i;
            let new_j = start_j as isize + k as isize * dir_j;
            // Ensure we are within bounds for both dimensions
            if new_i < 0 || new_i >= height as isize || new_j < 0 || new_j >= width as isize {
                return false; // Out of bounds
            }
            // Check if the character exists before pushing
            if let Some(c) = lines[new_i as usize].chars().nth(new_j as usize) {
                chars.push(c);
            } else {
                return false;
            }
        }
        chars.iter().collect::<String>() == word
    };

    // Check each character in the grid
    for i in 0..height {
        for j in 0..width {
            // Check all 8 directions
            if check_direction(i, j, 0, 1) {
                // Right
                occurrences.push((i, j, "Right"));
            }
            if check_direction(i, j, 1, 0) {
                // Down
                occurrences.push((i, j, "Down"));
            }
            if check_direction(i, j, 0, -1) {
                // Left
                occurrences.push((i, j, "Left"));
            }
            if check_direction(i, j, -1, 0) {
                // Up
                occurrences.push((i, j, "Up"));
            }
            if check_direction(i, j, 1, 1) {
                // Down-right
                occurrences.push((i, j, "Down-right"));
            }
            if check_direction(i, j, 1, -1) {
                // Down-left
                occurrences.push((i, j, "Down-left"));
            }
            if check_direction(i, j, -1, 1) {
                // Up-right
                occurrences.push((i, j, "Up-right"));
            }
            if check_direction(i, j, -1, -1) {
                // Up-left
                occurrences.push((i, j, "Up-left"));
            }
        }
    }

    occurrences
}
