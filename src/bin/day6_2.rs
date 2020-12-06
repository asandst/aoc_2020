use std::collections::HashSet;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    println!("{}", fs::read_to_string("input_day6").unwrap().split("\r\n\r\n").map(|g| g.split("\r\n").fold(None, |acc, x| Some(acc.unwrap_or(x.chars().collect::<HashSet<char>>()).intersection(&x.chars().collect()).cloned().collect()))).map(|s| s.unwrap().len()).sum::<usize>());
    Ok(())
}
