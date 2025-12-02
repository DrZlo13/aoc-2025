use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "../aoc01-1/input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut counter: i32 = 50;
    let mut zeros_encountered: i32 = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let line = line.trim();
        let num = line[1..].parse::<i32>().unwrap();

        match line {
            s if s.starts_with("L") => {
                // mirror counter to advance right, that way it's easier to count zeros
                let mirrored = (100 - counter) % 100;
                zeros_encountered += (mirrored + num) / 100;

                // but actually move the counter left
                counter = (counter - num) % 100;
                if counter < 0 {
                    counter += 100;
                }
            }
            s if s.starts_with("R") => {
                let next = counter + num;
                zeros_encountered += next / 100;
                counter = next % 100;
            }
            _ => panic!("should not happen"),
        }
    }

    println!("Answer: {}", zeros_encountered);

    Ok(())
}

// between 6698 and 7171
