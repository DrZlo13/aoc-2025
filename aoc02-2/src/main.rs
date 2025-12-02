use fancy_regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "../aoc02-1/input.txt";

fn process_pair(start: i64, end: i64) -> i64 {
    let mut sum = 0;

    // between start and end, must be only repetitions of the same substring
    let re = Regex::new(r"^(.+)(?:\1)+$").unwrap();

    for num in start..=end {
        let num_str = num.to_string();
        if re.is_match(&num_str).unwrap() {
            sum += num;
        }
    }

    sum
}

fn process_line(line: &str) -> i64 {
    let mut sum = 0;

    let parts = line.split(",");
    for part in parts {
        let pair = part.trim();
        let nums: Vec<&str> = pair.split("-").collect();
        let start = nums[0].parse::<i64>().unwrap();
        let end = nums[1].parse::<i64>().unwrap();

        sum += process_pair(start, end);
    }

    sum
}

fn main() -> std::io::Result<()> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines().flatten() {
        sum += process_line(&line);
    }

    println!("Total sum: {}", sum);

    Ok(())
}
