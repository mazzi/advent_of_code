use regex::Regex;

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

    let disable_pattern = Regex::new(r"don't\(\).*?do\(\)|$").unwrap();
    let input_data_wo_new_lines = input_data.replace("\n", "");
    let input_data = disable_pattern.replace_all(&input_data_wo_new_lines, "");

    let result: i32 = {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let mut sum = 0;

        for cap in re.captures_iter(&input_data) {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            sum += x * y;
        }
        sum
    };

    print!("Result: {:?}\n", result);

    Ok(())
}
