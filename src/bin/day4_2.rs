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

    let mut current_passport: HashMap<String, String> = HashMap::new();

    for line in input.lines() {
        let entry = line.unwrap();
        let entry = entry.trim();

        if entry.is_empty() {
            vec.push(current_passport);
            current_passport = HashMap::new();

        } else {

            for attr in entry.split(" "){
                //println!("{}", attr);
                let mut split = attr.split(":");
                let key = split.next().unwrap();
                let value = split.next().unwrap();
                current_passport.insert(key.to_string(), value.to_string());
            }
        }
    }

    let mut valid = 0;

    let hair_color = Regex::new(r"#[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]").unwrap();
    let eye_color = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    let pid = Regex::new(r"[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]").unwrap();

    for passport in vec {
        let mut invalid = false;

        if passport.contains_key("byr"){
            let value = passport.get("byr").unwrap();
            let num = value.parse();

            if num.is_ok() {
                let v : i64 = num.unwrap();
                invalid |= v <1920 || v > 2002;
            } else {
                invalid = true;
            }
        }

        if passport.contains_key("iyr"){
            let value = passport.get("iyr").unwrap();
            let num = value.parse();

            if num.is_ok() {
                let v : i64 = num.unwrap();
                invalid |= v <2010 || v > 2020;
            } else {
                invalid = true;
            }
        }

        if passport.contains_key("eyr"){
            let value = passport.get("eyr").unwrap();
            let num = value.parse();

            if num.is_ok() {
                let v : i64 = num.unwrap();
                invalid |= v <2020 || v > 2030;
            } else {
                invalid = true;
            }
        }

        if passport.contains_key("hgt"){
            let value = passport.get("hgt").unwrap();
            
            if value.ends_with("cm"){
                let mut tmp = value.clone();
                tmp.pop();
                tmp.pop();
                let num = tmp.parse();
                if num.is_ok() {
                    let v : i64 = num.unwrap();
                    invalid |= v <150 || v > 193;
                } else {
                    invalid = true;
                }
                
            } else if value.ends_with("in") {
                let mut tmp = value.clone();
                tmp.pop();
                tmp.pop();
                let num = tmp.parse();
                if num.is_ok() {
                    let v : i64 = num.unwrap();
                    invalid |= v <59 || v > 76;
                } else {
                    invalid = true;
                }
            } else {
                invalid = true;
            }
        }

        if passport.contains_key("hcl"){
            let value = passport.get("hcl").unwrap();
            if !hair_color.is_match(value){
                invalid = true;
            }
        }

        if passport.contains_key("ecl"){
            let value = passport.get("ecl").unwrap();
            if !eye_color.is_match(value){
                invalid = true;
            }
        }

        if passport.contains_key("pid"){
            let value = passport.get("pid").unwrap();
            if !pid.is_match(value){
                invalid = true;
            }
        }

        if passport.contains_key("byr") && passport.contains_key("iyr") 
        && passport.contains_key("eyr") && passport.contains_key("hgt") 
        && passport.contains_key("hcl") && passport.contains_key("ecl") 
        && passport.contains_key("pid") && !invalid {
            valid += 1;
            println!("{:?}", passport);
        }
    }

    println!("{}", valid);

    Ok(())
}