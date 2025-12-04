use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "../aoc04-1/input.txt";

fn file_line_count(file_path: &Path) -> std::io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

fn file_line_width(file_path: &Path) -> std::io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    if let Some(Ok(line)) = reader.lines().next() {
        Ok(line.len())
    } else {
        Ok(0)
    }
}

fn get_value_at(field: &Vec<Vec<bool>>, x: isize, y: isize) -> u8 {
    let field_height = field.len() as isize;
    let field_width = field[0].len() as isize;

    if y < field_height && x < field_width && x >= 0 && y >= 0 {
        field[y as usize][x as usize] as u8
    } else {
        0
    }
}

fn get_ajanced_sum_at(field: &Vec<Vec<bool>>, x: isize, y: isize) -> u8 {
    let mut sum = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            sum += get_value_at(field, x + dx, y + dy);
        }
    }
    sum
}

fn decrease_field_at(field: &mut Vec<Vec<u8>>, x: isize, y: isize) {
    let field_height = field.len() as isize;
    let field_width = field[0].len() as isize;

    if y < field_height && x < field_width && x >= 0 && y >= 0 {
        field[y as usize][x as usize] = field[y as usize][x as usize].saturating_sub(1);
    }
}

fn main() -> std::io::Result<()> {
    let file_path = Path::new(INPUT_FILE);
    let line_count = file_line_count(file_path)?;
    let line_width = file_line_width(file_path)?;
    let mut field = vec![vec![false; line_width]; line_count];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        for (j, ch) in line.chars().enumerate() {
            field[i][j] = match ch {
                '@' => true,
                _ => false,
            };
        }
    }

    // Precompute sums
    let mut field_of_sums = vec![vec![0u8; line_width]; line_count];
    for y in 0..line_count as isize {
        for x in 0..line_width as isize {
            if field[y as usize][x as usize] {
                let ajanced_sum = get_ajanced_sum_at(&field, x, y);
                field_of_sums[y as usize][x as usize] = ajanced_sum;
            }
        }
    }

    let mut sum = 0;
    loop {
        let mut stage_sum = 0;
        for y in 0..line_count as isize {
            for x in 0..line_width as isize {
                if field[y as usize][x as usize] {
                    let ajanced_sum = field_of_sums[y as usize][x as usize];
                    if ajanced_sum < 4 {
                        stage_sum += 1;
                        field[y as usize][x as usize] = false;

                        // Update sums
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dx == 0 && dy == 0 {
                                    continue;
                                }
                                decrease_field_at(&mut field_of_sums, x + dx, y + dy);
                            }
                        }
                    }
                }
            }
        }

        if stage_sum == 0 {
            break;
        }

        sum += stage_sum;
    }

    println!("Result: {}", sum);

    Ok(())
}
