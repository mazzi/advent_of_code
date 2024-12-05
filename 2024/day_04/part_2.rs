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
    println!("Total occurrences of 'X-MAS': {}", occurrences.len());

    Ok(())
}

fn find_occurrences(data: &str) -> Vec<(usize, usize)> {
    let mut occurrences = Vec::new();
    let lines: Vec<&str> = data.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    // Helper function to check for matches in all directions
    let check_x_mas = |i: usize, j: usize| {
        // M  M
        // S  S
        if lines[i - 1 as usize].chars().nth(j - 1 as usize) == Some('M')
            && lines[i - 1 as usize].chars().nth(j + 1 as usize) == Some('M')
            && lines[i + 1 as usize].chars().nth(j - 1 as usize) == Some('S')
            && lines[i + 1 as usize].chars().nth(j + 1 as usize) == Some('S')
        {
            return true;
        }

        // S  S
        // M  M
        if lines[i - 1 as usize].chars().nth(j - 1 as usize) == Some('S')
            && lines[i - 1 as usize].chars().nth(j + 1 as usize) == Some('S')
            && lines[i + 1 as usize].chars().nth(j - 1 as usize) == Some('M')
            && lines[i + 1 as usize].chars().nth(j + 1 as usize) == Some('M')
        {
            return true;
        }

        // M  S
        // M  S
        if lines[i - 1 as usize].chars().nth(j - 1 as usize) == Some('M')
            && lines[i - 1 as usize].chars().nth(j + 1 as usize) == Some('S')
            && lines[i + 1 as usize].chars().nth(j - 1 as usize) == Some('M')
            && lines[i + 1 as usize].chars().nth(j + 1 as usize) == Some('S')
        {
            return true;
        }

        // S  M
        // S  M
        if lines[i - 1 as usize].chars().nth(j - 1 as usize) == Some('S')
            && lines[i - 1 as usize].chars().nth(j + 1 as usize) == Some('M')
            && lines[i + 1 as usize].chars().nth(j - 1 as usize) == Some('S')
            && lines[i + 1 as usize].chars().nth(j + 1 as usize) == Some('M')
        {
            return true;
        }

        return false;
    };

    // Check each character in the grid without the borders
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            // We pivot around A
            if lines[i as usize].chars().nth(j as usize) != Some('A') {
                continue;
            }

            if check_x_mas(i, j) {
                occurrences.push((i, j));
            }
        }
    }

    occurrences
}
