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

    let safe_reports = input_data
        .lines()
        .filter(|report| {
            let levels: Vec<i32> = report
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();

            let is_valid_sequence = |levels: &[i32]| {
                let is_ascending = levels
                    .windows(2)
                    .all(|w| (w[1] - w[0]).abs() >= 1 && (w[1] - w[0]).abs() <= 3 && w[1] > w[0]);
                let is_descending = levels
                    .windows(2)
                    .all(|w| (w[1] - w[0]).abs() >= 1 && (w[1] - w[0]).abs() <= 3 && w[1] < w[0]);
                is_ascending || is_descending
            };

            if is_valid_sequence(&levels) {
                return true;
            }

            (0..levels.len()).any(|i| {
                let mut modified_levels = levels.clone();
                modified_levels.remove(i);
                is_valid_sequence(&modified_levels)
            })
        })
        .count();

    print!("Safe Reports: {:?}\n", safe_reports);

    Ok(())
}
