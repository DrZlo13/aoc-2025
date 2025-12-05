use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "../aoc05-1/input.txt";

#[derive(Clone, Copy)]
struct Range {
    from: usize,
    to: usize,
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        self.from <= other.to && other.from <= self.to
    }

    fn len(&self) -> usize {
        self.to - self.from + 1
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

fn count_numbers_in_ranges(ranges: &[Range]) -> usize {
    // merge overlapping ranges
    let merged = merge_ranges(ranges);

    // sum up the length
    merged.iter().map(|r| r.len()).sum()
}

fn main() -> std::io::Result<()> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut ranges: Vec<Range> = vec![];

    for line in reader.lines() {
        let line = line?;
        if line == "" {
            break;
        }

        let range: Vec<&str> = line.split('-').collect();
        ranges.push(Range {
            from: range[0].parse().unwrap(),
            to: range[1].parse().unwrap(),
        });
    }

    let count = count_numbers_in_ranges(&ranges);
    println!("Answer: {}", count);

    Ok(())
}
