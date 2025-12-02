use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut counter = 50;
    let mut zeros_encountered = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let line = line.trim();
        let num = line[1..].parse::<i32>().unwrap();

        match line {
            s if s.starts_with("L") => {
                counter -= num;
            }
            s if s.starts_with("R") => {
                counter += num;
            }
            _ => panic!("should not happen"),
        }

        counter %= 100;

        if counter == 0 {
            zeros_encountered += 1;
        }
    }

    println!("Answer: {}", zeros_encountered);

    Ok(())
}
