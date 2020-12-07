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

    let bag_regex = Regex::new(r"([0-9]+)(.+)").unwrap();
    let mut bag_contents: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut reverse_bag_contents: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let entry = line.unwrap();
        let vec = entry.split("contain").collect::<Vec<&str>>();
        let key = vec[0].trim();
        let contents = vec[1].split(",").collect::<Vec<&str>>();

        for content in contents{
            if content.trim() == "no other bags."{
                bag_contents.insert(key.to_string(), HashMap::new());
            } else {
                let caps = bag_regex.captures(&content).unwrap();
                let count: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                let name = caps.get(2).unwrap().as_str();
                let name = name.replace(".", "");
                let mut name = name.trim().to_string();
    
                if count == 1 {
                    name = name + "s";
                }

                let map = bag_contents.entry(key.to_string()).or_insert(HashMap::new());
                map.insert(name.to_string(), count);

                let rev_map = reverse_bag_contents.entry(name.to_string()).or_insert(HashSet::new());
                rev_map.insert(key.to_string());
            }
        }
    }

    let mut set = HashSet::new();
    can_contain_bag(&reverse_bag_contents, "shiny gold bags", &mut set);
    println!("Part 1 {}", set.len()-1);
    println!("Part 2 {}", count_contents(&bag_contents, "shiny gold bags") - 1);

    Ok(())
}

fn can_contain_bag(reverse_bag_contents: &HashMap<String, HashSet<String>>, key: &str, contains: &mut HashSet<String>){
    let def = HashSet::new();
    let possible = reverse_bag_contents.get(key).unwrap_or(&def);
    for bag in possible {
        can_contain_bag(reverse_bag_contents, bag, contains);
    }
    contains.insert(key.to_string());
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