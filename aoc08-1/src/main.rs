use partitions::PartitionVec;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "input.txt";
const CONNECTIONS: usize = 1000;

// const INPUT_FILE: &str = "input_example.txt";
// const CONNECTIONS: usize = 10;

#[derive(Debug)]
struct JBox {
    x: i32,
    y: i32,
    z: i32,
}

impl JBox {
    fn new(x: i32, y: i32, z: i32) -> Self {
        JBox { x, y, z }
    }

    fn from_string(s: &str) -> Self {
        let coords: Vec<i32> = s
            .split(',')
            .map(|part| part.trim().parse().unwrap())
            .collect();
        JBox::new(coords[0], coords[1], coords[2])
    }

    fn distance_squared_to(&self, other: &Self) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

fn main() -> std::io::Result<()> {
    let file_path = Path::new(INPUT_FILE);
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);
    let mut points: Vec<JBox> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        points.push(JBox::from_string(&line));
    }

    let mut distances: Vec<(i64, (usize, usize))> = Vec::new();
    let mut sets: PartitionVec<usize> = PartitionVec::new();

    for i in 0..points.len() {
        sets.push(i);

        for j in (i + 1)..points.len() {
            let dist = points[i].distance_squared_to(&points[j]);
            distances.push((dist, (i, j)));
        }
    }
    distances.sort_unstable_by_key(|k| k.0);

    for (_, (i, j)) in distances.iter().take(CONNECTIONS) {
        sets.union(*i, *j);
    }

    let mut sorted_sizes = sets.all_sets().map(|set| set.count()).collect::<Vec<_>>();
    sorted_sizes.sort_unstable_by(|a, b| b.cmp(a));

    let answer: usize = sorted_sizes.iter().take(3).product();
    println!("Answer: {}", answer);

    Ok(())
}
