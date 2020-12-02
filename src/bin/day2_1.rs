extern crate regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]+): ([a-z]+)").unwrap();

    let input = File::open("input_day2")?;
    let input = BufReader::new(input);

    let mut sum_correct = 0;

    for line in input.lines() {
        let entry = line.unwrap();

        let caps = re.captures(&entry).unwrap();

        let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let c = caps.get(3).unwrap().as_str();
        let password = caps.get(4).unwrap().as_str();

        let num_matches = password.matches(c).count();

        if num_matches >= min && num_matches <= max {
            sum_correct+= 1;
        }
    }

    println!("{}", sum_correct);
    Ok(())
}
