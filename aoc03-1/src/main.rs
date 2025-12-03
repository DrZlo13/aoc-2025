use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "input.txt";

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
    let end = line.len() - 1;
    let (first, first_pos) = max_digit_in_chars(&line[..end]);
    let (second, _) = max_digit_in_chars(&line[first_pos + 1..]);
    first * 10 + second
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
