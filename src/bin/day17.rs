extern crate itertools;
use itertools::iproduct;
use itertools::izip;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day17")?;
    let input = BufReader::new(input).lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();
    let iter = -1..=1;

    let map = input.iter().enumerate().map(|(y, l)| l.chars().enumerate().filter(|(_, c)| *c == '#').map(move |(x, _)| vec!{x as i64,y as i64,0})).flatten().collect::<HashSet<Vec<i64>>>();
    let count = run(map, &iproduct!(iter.clone(), iter.clone(), iter.clone()).map(|(x, y, z)| vec!{x, y, z}).filter(|vec| vec.iter().any(|&v| v!= 0)).collect());
    println!("Part 1 {}", count);
    let map = input.iter().enumerate().map(|(y, l)| l.chars().enumerate().filter(|(_, c)| *c == '#').map(move |(x, _)| vec!{x as i64,y as i64, 0, 0})).flatten().collect::<HashSet<Vec<i64>>>();
    let count = run(map, &iproduct!(iter.clone(), iter.clone(), iter.clone(), iter.clone()).map(|(x, y, z, w)| vec!{x, y, z, w}).filter(|vec| vec.iter().any(|&v| v!= 0)).collect());
    println!("Part 2 {}", count);

    Ok(())
}

fn run(mut map: HashSet<Vec<i64>>, points: &Vec<Vec<i64>>) -> usize {
    for _ in 0..6{
        let mut new_map : HashSet<Vec<i64>> = HashSet::new();
        for active in &map {
            let points_gen = |v1 : Vec<i64>| points.iter().map(move |v2| izip!(v1.iter(), v2.iter()).map(|(value, offset)| value + offset).collect::<Vec<i64>>());
            let (on, off) : (Vec<Vec<i64>>, Vec<Vec<i64>>) =  points_gen(active.clone()).partition(|key| map.contains(key));
            off.iter().filter(|key| 3 == points_gen(key.clone().to_vec()).filter(|key| map.contains(key)).count()).for_each(|key| drop(new_map.insert(key.clone())));
            match on.iter().count() {
                2 | 3 => drop(new_map.insert(active.clone())),
                _ => ()
            }
        }

        map = new_map;
    }
    map.len()
}