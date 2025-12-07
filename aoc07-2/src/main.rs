use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "../aoc07-1/input.txt";

fn main() -> std::io::Result<()> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;

    let mut reader = BufReader::new(file);
    let mut first_line = String::new();

    reader.read_line(&mut first_line)?;

    let mut current: Vec<u64> = first_line
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();

    for line in reader.lines().skip(1).step_by(2) {
        let splitters = line?;
        for (i, c) in splitters.chars().enumerate() {
            if c == '^' {
                current[i - 1] += current[i];
                current[i + 1] += current[i];
                current[i] = 0;
            }
        }
    }

    let sum = current.iter().sum::<u64>();

    println!("Answer: {}", sum);

    Ok(())
}
