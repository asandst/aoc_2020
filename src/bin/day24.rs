
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::time::Instant;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let now = Instant::now();
    let input = File::open("input_day24")?;
    let input = BufReader::new(input).lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();
    let mut instructions = Vec::new();
    let mut map : HashSet<(i64, i64)>= HashSet::new();

    for line in input{
        let mut line = line;
        let mut dirs = Vec::new();
        while line.len() > 0 {
            let (chars, dir) = parse(&line);
            dirs.push(dir);
            line = line.split_off(chars);
        }
        instructions.push(dirs);
    }

    for instr in instructions{
        let mut pos = (0,0);
        for dir in instr{
            pos = dir.apply(pos);
        }
        if map.contains(&pos) {
            map.remove(&pos);
        } else {
            map.insert(pos);
        }
    }
    println!("Part1 {}", map.len());

    let mut current_map = map.clone();
    for _day in 0..100 {
        let mut next_map : HashSet<(i64, i64)>= HashSet::new();
        for (x, y) in &current_map{
            let adj_count = dirs().iter().map(|dir| current_map.contains(&dir.apply((*x, *y)))).filter(|&b| b).count();
            if adj_count ==1 || adj_count == 2{
                next_map.insert((*x, *y));
            }

            for (a_x, a_y) in dirs().iter().map(|dir| dir.apply((*x, *y))).filter(|pos| !current_map.contains(pos)){
                let adj_count = dirs().iter().map(|dir| current_map.contains(&dir.apply((a_x, a_y)))).filter(|&b| b).count();
                if adj_count == 2 {
                    next_map.insert((a_x, a_y));
                }
            }
        }
        current_map = next_map;
    }
    println!("Part2 {}", current_map.len());
    println!("{}", now.elapsed().as_millis());
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Dir{
    East,
    West,
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest
}

impl Dir{
    fn apply(&self, (x, y): (i64, i64)) -> (i64, i64) {
        match self{
            Dir::East => (x+1, y),
            Dir::West => (x-1, y),
            Dir::SouthEast => (x+(y%2).abs(), y+1),
            Dir::SouthWest => (x-((y+1)%2).abs(), y+1),
            Dir::NorthEast => (x+(y%2).abs(), y-1),
            Dir::NorthWest => (x-((y+1)%2).abs(), y-1),
        }
    }
}

fn dirs() -> [Dir;6] {
    [Dir::East, Dir::West, Dir::SouthEast, Dir::SouthWest, Dir::NorthEast, Dir::NorthWest]
}

fn parse(line: &String) -> (usize, Dir){
    if line.starts_with("e") {
        (1, Dir::East)
    } else if line.starts_with("w") {
        (1, Dir::West)
    } else if line.starts_with("se") {
        (2, Dir::SouthEast)
    } else if line.starts_with("sw") {
        (2, Dir::SouthWest)
    } else if line.starts_with("ne") {
        (2, Dir::NorthEast)
    } else {
        (2, Dir::NorthWest)
    }
}