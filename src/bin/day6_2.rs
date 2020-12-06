extern crate itertools;
use std::collections::HashSet;
use std::fs;
use std::io;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input_day6").unwrap();
    println!("{:?}", input.split("\r\n\r\n").map(|g| g.split("\r\n").map(|p| p.chars().collect::<HashSet<char>>()).fold1(|x, y| x.intersection(&y).cloned().collect())).map(|s| s.unwrap().len()).sum::<usize>());
    Ok(())
}
