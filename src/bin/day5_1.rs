use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day5")?;
    let input = BufReader::new(input);
    
    let mut max = 0;

    for line in input.lines() {
        let entry = line.unwrap();
        let binary_str = entry.replace("B","1").replace("F","0").replace("R","1").replace("L","0");
        let intval = isize::from_str_radix(&binary_str, 2).unwrap();
        if intval > max {
            max = intval;
        }
    }
    println!("{}", max);

    Ok(())
}
