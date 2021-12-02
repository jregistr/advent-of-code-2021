use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs;

fn main() {
    let depths = load_sonar_depths("input.txt")
        .expect("Expected parsed list of depths");

    let increased = increased_from_prev(&depths);
    println!("Number of depths that increased from previous: {}", increased);

    let triple_inc = increased_triple(&depths);
    println!("Number that increased when using triples: {}", triple_inc);
}

fn increased_triple(depths: &[i32]) -> i32 {
    depths.windows(3)
        .map(|window| window.iter().sum::<i32>())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|&window| {
            if let [prev, cur] = window {
                return cur > prev;
            }
            false
        }).count() as i32
}

fn increased_from_prev(depths: &[i32]) -> i32 {
    let mut windows = depths.windows(2);

    let mut count = 0;
    while let Some([prev, cur]) = windows.next() {
        if cur > prev {
            count += 1;
        }
    }
    count
}

fn load_sonar_depths(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let f = fs::File::open(filename)?;
    let reader = BufReader::new(f);
    let mut buffer = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parsed = line.parse::<i32>()?;
        buffer.push(parsed);
    }
    Ok(buffer)
}
