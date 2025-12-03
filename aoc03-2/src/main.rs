use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "../aoc03-1/input.txt";

fn max_digit_in_chars(chars: &str) -> (i64, usize) {
    let mut max_digit = 0;
    let mut max_digit_pos = 0;

    for (i, ch) in chars.chars().enumerate() {
        if let Some(digit) = ch.to_digit(10) {
            if digit as i64 > max_digit {
                max_digit = digit as i64;
                max_digit_pos = i;
            }
        }
    }
    (max_digit, max_digit_pos)
}

fn process_line(line: &str) -> i64 {
    let count = 12;
    let len = line.len();

    let mut total = 0;
    let mut multiplier = i64::pow(10, count as u32 - 1);
    let mut start = 0;
    let mut end = len - (count - 1);

    for _ in 0..count {
        let (max_digit, pos) = max_digit_in_chars(&line[start..end]);
        total += max_digit * multiplier;
        multiplier /= 10;
        start += pos + 1;
        end += 1;
    }

    total
}

fn main() -> std::io::Result<()> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines().flatten() {
        sum += process_line(&line.trim());
    }

    println!("Total sum: {}", sum);

    Ok(())
}
