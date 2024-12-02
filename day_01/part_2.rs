use std::fs::File;
use std::io::Read;

const INPUT_FILENAME: &str = "input";

fn main() -> std::io::Result<()> {
    let input_data = {
        let mut f = File::open(INPUT_FILENAME)?;
        let mut data = vec![];
        f.read_to_end(&mut data)?;
        String::from_utf8_lossy(&data).to_string()
    };

    let (left, right): (Vec<i32>, Vec<i32>) = input_data
        .lines()
        .filter_map(|line| {
            let mut words = line.split_whitespace();
            if let (Some(first), Some(second)) = (words.next(), words.next()) {
                // Attempt to parse both numbers and check for success
                if let (Ok(first_num), Ok(second_num)) =
                    (first.parse::<i32>(), second.parse::<i32>())
                {
                    Some((first_num, second_num))
                } else {
                    eprintln!("Warning: Could not parse numbers: {} {}", first, second);
                    None
                }
            } else {
                eprintln!(
                    "Warning: Line does not contain exactly two numbers: {}",
                    line
                );
                None
            }
        })
        .unzip();

    let similarity_score: i32 = left
        .iter()
        .map(|&num| {
            let count = right.iter().filter(|&&x| x == num).count() as i32;
            num * count
        })
        .sum();

    println!("Similarity score: {}", similarity_score);

    Ok(())
}
