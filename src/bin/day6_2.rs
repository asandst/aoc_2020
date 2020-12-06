use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day6")?;
    let input = BufReader::new(input);

    let mut current_group: HashSet<char> = HashSet::new();
    let mut started = false;

    let mut sum = 0;

    for line in input.lines() {
        let entry = line.unwrap();
        let entry = entry.trim();

        if entry.is_empty() {
            sum += current_group.len();
            started = false;
        } else {
            if !started{
                current_group = entry.chars().collect();
                started = true;
            } else {
                let person: HashSet<char> = entry.chars().collect();
                current_group = current_group.intersection(&person).cloned().collect();
            }
        }
    }

    println!("{}", sum);
    Ok(())
}