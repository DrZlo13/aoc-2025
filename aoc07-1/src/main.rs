use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "input.txt";

fn main() -> std::io::Result<()> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut current: Vec<char>;
    let mut previous: Vec<char> = Vec::new();
    let mut split = 0;

    for (line_index, line) in lines.enumerate() {
        let line = line?;
        current = line.chars().collect();

        if line_index >= 1 {
            for i in 0..previous.len() {
                match current[i] {
                    '.' => match previous[i] {
                        'S' | '|' => {
                            current[i] = '|';
                        }
                        _ => {}
                    },
                    '^' => match previous[i] {
                        '|' => {
                            split += 1;
                            current[i - 1] = '|';
                            current[i + 1] = '|';
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        previous = current.clone();
    }

    println!("Answer: {}", split);

    Ok(())
}
