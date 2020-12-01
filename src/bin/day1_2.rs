
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let input = File::open("input_day1_1")?;
    let input = BufReader::new(input);

    let mut map: HashSet<i64> = HashSet::new();

    for line in input.lines() {
        let entry = line.unwrap();
        let entry = entry.parse().unwrap();

        map.insert(entry);
    }

    for entry in &map {
        for entry2 in &map {
            let entry3 = 2020 - entry - entry2;

            if map.contains(&entry3) {
                println!("{}", entry * entry2 * entry3 );
                return Ok(())
            }
        }
    }

    println!("{}", "day1_1");
    Ok(())
}