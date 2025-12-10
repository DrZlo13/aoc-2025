use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "input.txt";

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn from_string(s: &str) -> Self {
        let coords: Vec<i32> = s
            .split(',')
            .map(|part| part.trim().parse().unwrap())
            .collect();
        Point::new(coords[0], coords[1])
    }

    fn rect_area(&self, other: &Point) -> u64 {
        let width = (other.x - self.x).abs() + 1;
        let height = (other.y - self.y).abs() + 1;
        (width as u64) * (height as u64)
    }
}

fn main() -> std::io::Result<()> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);
    let mut points: Vec<Point> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        points.push(Point::from_string(&line));
    }

    // very simple O(N^2) algorithm, but can be optimized to O(N^2 log N)
    // by sorting points by axis and using binary search
    let mut max_area: u64 = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let area: u64 = points[i].rect_area(&points[j]);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Answer: {}", max_area);

    Ok(())
}
