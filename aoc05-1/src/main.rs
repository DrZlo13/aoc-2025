use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "input.txt";

#[derive(Clone, Copy)]
struct Range {
    from: usize,
    to: usize,
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        self.from <= other.to && other.from <= self.to
    }

    fn merge(&self, other: &Range) -> Range {
        Range {
            from: min(self.from, other.from),
            to: max(self.to, other.to),
        }
    }
}

fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut sorted = ranges.to_vec();
    sorted.sort_by_key(|r| r.from);

    let mut merged = vec![sorted[0]];

    for current in sorted.iter().skip(1) {
        let last = merged.last_mut().unwrap();

        if last.overlaps(current) || last.to + 1 == current.from {
            *last = last.merge(current);
        } else {
            merged.push(*current);
        }
    }

    merged
}

fn count_values_in_ranges(ranges: &[Range], values: &[usize]) -> usize {
    // merge overlapping ranges
    let merged = merge_ranges(ranges);

    // count values that fall in any range
    values
        .iter()
        .filter(|&&value| {
            // binary search to find potential range
            let idx = merged.binary_search_by(|r| {
                if value < r.from {
                    std::cmp::Ordering::Greater
                } else if value > r.to {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });

            idx.is_ok()
        })
        .count()
}

enum Mode {
    Pairs,
    Values,
}

fn main() -> std::io::Result<()> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut ranges: Vec<Range> = vec![];
    let mut values: Vec<usize> = vec![];
    let mut mode = Mode::Pairs;

    for line in reader.lines() {
        let line = line?;
        if line == "" {
            mode = Mode::Values;
            continue;
        }

        match mode {
            Mode::Pairs => {
                let range: Vec<&str> = line.split('-').collect();
                ranges.push(Range {
                    from: range[0].parse().unwrap(),
                    to: range[1].parse().unwrap(),
                });
            }
            Mode::Values => {
                values.push(line.parse().unwrap());
            }
        }
    }

    let count = count_values_in_ranges(&ranges, &values);
    println!("Answer: {}", count);

    Ok(())
}
