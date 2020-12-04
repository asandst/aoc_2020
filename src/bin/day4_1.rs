extern crate regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use regex::Regex;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let input = File::open("input_day4")?;
    let input = BufReader::new(input);

    let mut vec = Vec::new();

    let mut current_password: HashMap<String, String> = HashMap::new();

    for line in input.lines() {
        let entry = line.unwrap();
        let entry = entry.trim();

        if entry.is_empty() {
            vec.push(current_password);
            current_password = HashMap::new();

        } else {

            for attr in entry.split(" "){
                //println!("{}", attr);
                let mut split = attr.split(":");
                let key = split.next().unwrap();
                let value = split.next().unwrap();
                current_password.insert(key.to_string(), value.to_string());
            }
        }
    }

    let mut valid = 0;

    for password in vec {
        if password.contains_key("byr") && password.contains_key("iyr") 
        && password.contains_key("eyr") && password.contains_key("hgt") 
        && password.contains_key("hcl") && password.contains_key("ecl") 
        && password.contains_key("pid") {
            valid += 1;
            println!("{:?}", password);
        }
    }

    println!("{}", valid);

    Ok(())
}