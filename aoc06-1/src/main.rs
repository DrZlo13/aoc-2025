use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "input.txt";
const NUMS_COUNT: usize = 4; // number of lines with numbers, hardcoded for simplicity

#[derive(Debug)]
enum OperationType {
    None,
    Add,
    Mul,
}

#[derive(Debug)]
struct Operation {
    numbers: [i128; NUMS_COUNT],
    operation: OperationType,
}

fn main() -> std::io::Result<()> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut operations: Vec<Operation> = Vec::new();

    for (i, line) in lines.enumerate() {
        if i < NUMS_COUNT {
            let line = line?;
            let nums = line
                .split_whitespace()
                .flat_map(str::parse::<i128>)
                .collect::<Vec<_>>();

            for (n, num) in nums.iter().enumerate() {
                if operations.len() <= n {
                    operations.push(Operation {
                        numbers: [0; NUMS_COUNT],
                        operation: OperationType::None,
                    });
                }

                operations[n].numbers[i] = *num;
            }
        } else {
            let line = line?;
            let ops = line.trim().split_whitespace().collect::<Vec<_>>();

            for (n, op) in ops.iter().enumerate() {
                let op_char = op.chars().next().unwrap();
                operations[n].operation = match op_char {
                    '+' => OperationType::Add,
                    '*' => OperationType::Mul,
                    _ => OperationType::None,
                };
            }
        }
    }

    let mut sum = 0;
    for op in operations {
        let result;

        match op.operation {
            OperationType::Add => {
                result = op.numbers.iter().sum::<i128>();
            }
            OperationType::Mul => {
                result = op.numbers.iter().product::<i128>();
            }
            OperationType::None => {
                panic!("Unknown operation");
            }
        }

        sum += result;
    }

    println!("Answer: {}", sum);

    Ok(())
}
