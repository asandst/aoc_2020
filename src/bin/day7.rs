extern crate regex;
use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day7")?;
    let input = BufReader::new(input);

    let bag_regex = Regex::new(r"([0-9]+)\s*(.+)\s*").unwrap();
    let mut bag_contents: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut reverse_bag_contents: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let line = line?;
        let parts = line.split("contain").collect::<Vec<&str>>();
        let key = parts[0].trim().to_string();
        let contents = parts[1].split(",").collect::<Vec<&str>>();

        for content in contents{
            if content.trim() == "no other bags."{
                bag_contents.insert(key.clone(), HashMap::new());
            } else {
                let caps = bag_regex.captures(&content).unwrap();
                let count = caps.get(1).unwrap().as_str().parse().unwrap();
                
                let name = caps.get(2).unwrap().as_str().replace(".", "").to_string();
                let name = match count {
                    1 => name + "s",
                    _ => name
                };

                let map = bag_contents.entry(key.clone()).or_insert(HashMap::new());
                map.insert(name.clone(), count);

                let rev_map = reverse_bag_contents.entry(name).or_insert(HashSet::new());
                rev_map.insert(key.clone());
            }
        }
    }

    let mut bags = HashSet::new();
    can_contain_bag(&reverse_bag_contents, "shiny gold bags", &mut bags);
    println!("Part 1 {}", bags.len());
    println!("Part 2 {}", count_contents(&bag_contents, "shiny gold bags") - 1);

    Ok(())
}

fn can_contain_bag(reverse_bag_contents: &HashMap<String, HashSet<String>>, key: &str, contains: &mut HashSet<String>){
    let def = HashSet::new();
    let possible = reverse_bag_contents.get(key).unwrap_or(&def);
    for bag in possible {
        contains.insert(bag.to_string());
        can_contain_bag(reverse_bag_contents, bag, contains);
    }
}

fn count_contents(map: &HashMap<String, HashMap<String, usize>>, key: &str) -> usize {
    let mut total_count = 0;

    let def = HashMap::new();
    let possible = map.get(key).unwrap_or(&def);
    for (bag, count) in possible {
        total_count += count * count_contents(map, bag);
    }
    total_count + 1
}