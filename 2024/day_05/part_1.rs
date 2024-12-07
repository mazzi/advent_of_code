use std::fs::File;
use std::io::{self, Read};

use std::collections::HashMap;

fn main() -> io::Result<()> {
    const INPUT_FILENAME: &str = "input";

    // Read input data from the file
    let input_data = {
        let mut f = File::open(INPUT_FILENAME)?;
        let mut data = vec![];
        f.read_to_end(&mut data)?;
        String::from_utf8_lossy(&data).to_string()
    };

    let parts: Vec<&str> = input_data.split("\n\n").collect();
    let rules_data = parts.get(0).unwrap_or(&"");
    let updates_data = parts.get(1).unwrap_or(&"");

    let rules = build_rules(&rules_data);
    let updates = get_updates(&updates_data);

    let valid_updates = get_valid_updates(updates, rules);

    println!("Result: {:?}", valid_updates);

    let sum = sum_middle_pages(&valid_updates);

    println!("Result: {:?}", sum);

    Ok(())
}

fn build_rules(input_data: &str) -> HashMap<i32, Vec<i32>> {
    let mut rules = HashMap::new();

    for line in input_data.lines() {
        if line.trim().is_empty() {
            return rules;
        }
        if let Some((key, value)) = line.split_once('|') {
            if let (Ok(parsed_key), Ok(parsed_value)) =
                (key.trim().parse::<i32>(), value.trim().parse::<i32>())
            {
                rules
                    .entry(parsed_key)
                    .or_insert_with(Vec::new)
                    .push(parsed_value);
            }
        }
    }

    rules
}

fn get_updates(updates_data: &str) -> Vec<Vec<i32>> {
    updates_data
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.trim().parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn get_valid_updates(updates: Vec<Vec<i32>>, rules: HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>> {
    let mut valid_updates = Vec::new();

    for update in &updates {
        let mut is_valid = true;

        for (key, allowed_values) in &rules {
            if update.contains(key) {
                let key_index = update.iter().position(|&u| u == *key).unwrap();

                for value in allowed_values {
                    if let Some(value_index) = update.iter().position(|&u| u == *value) {
                        if key_index >= value_index {
                            is_valid = false;
                            break;
                        }
                    }
                }
            }
            if !is_valid {
                break;
            }
        }

        if is_valid {
            valid_updates.push(update.clone());
        }
    }

    valid_updates
}

fn sum_middle_pages(valid_updates: &Vec<Vec<i32>>) -> i32 {
    let mut total_sum = 0;

    for update in valid_updates {
        let len = update.len();
        if len > 0 {
            let middle_index = len / 2;
            if len % 2 == 0 {
                // If even, sum the two middle elements
                total_sum += update[middle_index - 1] + update[middle_index];
            } else {
                // If odd, sum the middle element
                total_sum += update[middle_index];
            }
        }
    }

    total_sum
}
