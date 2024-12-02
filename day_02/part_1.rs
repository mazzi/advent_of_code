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

            let is_valid_transition =
                |w: &[i32]| (w[1] - w[0]).abs() >= 1 && (w[1] - w[0]).abs() <= 3;

            let is_ascending = levels
                .windows(2)
                .all(|w| is_valid_transition(w) && w[1] > w[0]);
            let is_descending = levels
                .windows(2)
                .all(|w| is_valid_transition(w) && w[1] < w[0]);

            is_ascending || is_descending
        })
        .count();

    print!("Safe Reports: {:?}\n", safe_reports);

    Ok(())
}
