use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day6")?;
    let input = BufReader::new(input);

    let mut current_group: HashSet<char> = HashSet::new();
    let mut sum = 0;

    for line in input.lines() {
        let entry = line.unwrap();
        let entry = entry.trim();

        if entry.is_empty() {
            sum += current_group.len();
            current_group = HashSet::new();
        } else {
            current_group.extend(entry.chars());
        }
    }

    println!("{}", sum);

    Ok(())
}