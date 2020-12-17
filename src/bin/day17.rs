extern crate itertools;
use itertools::iproduct;
use itertools::izip;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day17")?;
    let input = BufReader::new(input);
    let input = input.lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();
    let iter = -1..=1;

    let mut map : HashSet<Vec<i64>> = HashSet::new();
    for (y, l) in input.iter().enumerate(){
        for (x, c) in l.chars().enumerate(){
            if c == '#'{
                map.insert(vec!{x as i64,y as i64,0});
            }
         }
    }
    
    let count = run(map, &iproduct!(iter.clone(), iter.clone(), iter.clone()).map(|(x, y, z)| vec!{x, y, z}).filter(|vec| vec.iter().any(|&v| v!= 0)).collect());
    println!("Part 1 {}", count);

    let mut map : HashSet<Vec<i64>> = HashSet::new();
    for (y, l) in input.iter().enumerate(){
        for (x, c) in l.chars().enumerate(){
            if c == '#'{
                map.insert(vec!{x as i64, y as i64, 0, 0});
            }
         }
    }
    let count = run(map, &iproduct!(iter.clone(), iter.clone(), iter.clone(), iter.clone()).map(|(x, y, z, w)| vec!{x, y, z, w}).filter(|vec| vec.iter().any(|&v| v!= 0)).collect());
    println!("Part 2 {}", count);

    Ok(())
}

fn run(mut map: HashSet<Vec<i64>>, points: &Vec<Vec<i64>>) -> usize {
    for _ in 0..6{
        let mut new_map : HashSet<Vec<i64>> = HashSet::new();

        for active in &map {
            let mut active_neighbors = 0;
            for v1 in points{
                let key1 = izip!(active.iter(), v1.iter()).map(|(value, offset)| value + offset).collect::<Vec<i64>>();
                if map.contains(&key1){
                    active_neighbors += 1;
                } else {
                    let active_neighbors2 = points.iter().map(|v2| izip!(active.iter(), v1.iter(), v2.iter()).map(|(value, offset1, offset2)| value + offset1 + offset2).collect::<Vec<i64>>()).filter(|key| map.contains(key)).count();
                    if active_neighbors2 == 3 {
                        new_map.insert(key1.clone());
                    }
                }
            }

            if active_neighbors >=2 && active_neighbors <=3{
                new_map.insert(active.clone());
            }
        }

        map = new_map;
    }
    map.len()
}