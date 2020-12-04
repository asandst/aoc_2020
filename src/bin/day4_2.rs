extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day4")?;
    let input = BufReader::new(input);

    let mut vec = Vec::new();

    let mut current_passport: HashMap<String, String> = HashMap::new();

    for line in input.lines() {
        let entry = line.unwrap();
        let entry = entry.trim();

        if entry.is_empty() {
            vec.push(current_passport);
            current_passport = HashMap::new();
        } else {
            for attr in entry.split(" ") {
                let mut split = attr.split(":");
                let key = split.next().unwrap();
                let value = split.next().unwrap();
                current_passport.insert(key.to_string(), value.to_string());
            }
        }
    }

    let mut valid = 0;

    let hair_color = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let eye_color = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
    let pid = Regex::new(r"^[0-9]{9}$").unwrap();

    for passport in vec {
        let mut invalid = false;

        invalid |= check_invalid_number(&passport, "byr", (1920, 2002), &|v| v.parse().ok());
        invalid |= check_invalid_number(&passport, "iyr", (2010, 2020), &|v| v.parse().ok());
        invalid |= check_invalid_number(&passport, "eyr", (2020, 2030), &|v| v.parse().ok());

        invalid |= check_invalid_number(&passport, "hgt", (150, 193), &|v| {
            if v.ends_with("cm") {
                return v[..v.chars().count() - 2].parse().ok();
            } else {
                return None;
            }
        }) && check_invalid_number(&passport, "hgt", (59, 76), &|v| {
            if v.ends_with("in") {
                return v[..v.chars().count() - 2].parse().ok();
            } else {
                return None;
            }
        });

        invalid |= check_invalid(&passport, "hcl", &hair_color);
        invalid |= check_invalid(&passport, "ecl", &eye_color);
        invalid |= check_invalid(&passport, "pid", &pid);

        if !invalid {
            valid += 1;
        }
    }
    
    println!("{}", valid);
    Ok(())
}

fn check_invalid(passport: &HashMap<String, String>, key: &str, regex: &Regex) -> bool {
    return match passport.get(key) {
        Some(v) => !regex.is_match(v),
        None => true,
    };
}

fn check_invalid_number(
    passport: &HashMap<String, String>,
    key: &str,
    (min, max): (i64, i64),
    parser: &dyn Fn(&String) -> Option<i64>,
) -> bool {
    return match passport.get(key) {
        Some(v) => match parser(v) {
            Some(v) => v < min || v > max,
            None => true,
        },
        None => true,
    };
}