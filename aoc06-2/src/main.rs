use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "../aoc06-1/input.txt";

#[derive(Debug, PartialEq)]
enum OperationType {
    None,
    Add,
    Mul,
}

#[derive(Debug)]
struct Operation {
    numbers: Vec<i128>,
    operation: OperationType,
}

impl Operation {
    fn new() -> Self {
        Operation {
            numbers: Vec::new(),
            operation: OperationType::None,
        }
    }
}

fn main() -> std::io::Result<()> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);
    let lines = reader.lines();
    let grid: Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();

    // rotate grid 90 degrees counter-clockwise
    let mut rotated_grid: Vec<Vec<char>> = vec![vec![' '; grid.len()]; grid[0].len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            rotated_grid[grid[0].len() - j - 1][i] = grid[i][j];
        }
    }

    let mut operations: Vec<Operation> = Vec::new();

    let mut operation: Operation = Operation::new();
    for row in rotated_grid {
        let mut line: String = row.into_iter().collect();
        if line.trim().is_empty() {
            continue;
        }

        if line.contains('+') {
            operation.operation = OperationType::Add;
            line = line.replace('+', "");
        } else if line.contains('*') {
            operation.operation = OperationType::Mul;
            line = line.replace('*', "");
        }

        let num = line.trim().parse::<i128>().unwrap();
        operation.numbers.push(num);

        if operation.operation != OperationType::None {
            operations.push(operation);
            operation = Operation::new();
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
