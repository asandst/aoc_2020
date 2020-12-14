extern crate regex;
use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day14")?;
    let input = BufReader::new(input);
    let input = input.lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();

    let mask_regex = Regex::new(r"mask = ([10X]+)").unwrap();
    let mem_regex = Regex::new(r"mem\[([0-9]+)\] = ([0-9]+)").unwrap();

    let mut memory : HashMap<u64, u64> = HashMap::new();
    let mut x_mask : u64 = 0;
    let mut value0_mask : u64 = 0;
    let mut value1_mask : u64 = 0;

    for line in &input {
        if mask_regex.is_match(&line){
            let caps = mask_regex.captures(&line).unwrap();
            let mask = caps.get(1).unwrap().as_str();

            let tmp_x_mask = mask.replace("1", "0").replace("X", "1");
            x_mask = isize::from_str_radix(&tmp_x_mask, 2).unwrap() as u64;

            let tmp_value0_mask = mask.replace("X", "1");
            value0_mask = isize::from_str_radix(&tmp_value0_mask, 2).unwrap() as u64;
            
            let tmp_value1_mask = mask.replace("X", "0");
            value1_mask = isize::from_str_radix(&tmp_value1_mask, 2).unwrap() as u64;
        } else if mem_regex.is_match(&line){
            let caps = mem_regex.captures(&line).unwrap();
            let addr:u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let value:u64 = caps.get(2).unwrap().as_str().parse().unwrap();

            let masked = value & x_mask & value0_mask | value1_mask;

            memory.insert(addr, masked);
        }
    }
    println!("Part 1 {}", memory.values().sum::<u64>());

    let mut memory : HashMap<u64, u64> = HashMap::new();
    let mut x_mask : u64 = 0;
    let mut x_inv_mask : u64 = 0;
    let mut value1_mask : u64 = 0;

    for line in input {
        if mask_regex.is_match(&line){
            let caps = mask_regex.captures(&line).unwrap();
            let mask = caps.get(1).unwrap().as_str();

            let tmp_x_mask = mask.replace("1", "0").replace("X", "1");
            x_mask = isize::from_str_radix(&tmp_x_mask, 2).unwrap() as u64;

            let tmp_x_inv_mask = mask.replace("0", "1").replace("X", "0");
            x_inv_mask = isize::from_str_radix(&tmp_x_inv_mask, 2).unwrap() as u64;

            let tmp_value1_mask = mask.replace("X", "0");
            value1_mask = isize::from_str_radix(&tmp_value1_mask, 2).unwrap() as u64;
        } else if mem_regex.is_match(&line){
            let caps = mem_regex.captures(&line).unwrap();
            let addr:u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let value:u64 = caps.get(2).unwrap().as_str().parse().unwrap();

            let x_masks = x_mask_to_all_variants(x_mask);
            for x in x_masks {
                let masked = addr & x_inv_mask | value1_mask | x;
                memory.insert(masked, value);
            }
        }
    }
    println!("Part 2 {}", memory.values().sum::<u64>());
    Ok(())
}

fn x_mask_to_all_variants(x_mask: u64) -> HashSet<u64>{
    let mut variants = HashSet::new();
    variants.insert(0);
    for bit in 0..36 {
        let bit_mask = 1<<bit;
        let x_bit = x_mask & bit_mask;
        if x_bit > 0 {
            let mut new_masks = HashSet::new();
            for o in &variants{
                new_masks.insert(o | bit_mask);
            }
            variants.extend(new_masks);
        }
    }
    variants
}