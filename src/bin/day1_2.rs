use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    
    let input = File::open("input_day1_1")?;
    let input = BufReader::new(input);

    let mut all: HashSet<i64> = HashSet::new();
    let mut small: HashSet<i64> = HashSet::new();

    for line in input.lines() {
        let entry = line.unwrap();
        let entry = entry.parse().unwrap();

        all.insert(entry);

        if entry < 1010{
            small.insert(entry);
        }
    }
    
    for entry in &small {
        for entry2 in &small {
            let entry3 = 2020 - entry - entry2;

            if all.contains(&entry3) {
                println!("{}", entry * entry2 * entry3 );
                return Ok(())
            }
        }
    }

    Ok(())
}
