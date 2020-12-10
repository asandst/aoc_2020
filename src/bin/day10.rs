use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::BTreeMap;

fn main() -> io::Result<()> {
    let input = File::open("input_day10")?;
    let input = BufReader::new(input);

    let mut numbers: Vec<i64> = Vec::new();
    numbers.push(0);
    numbers.extend(input.lines().map(|line| line.unwrap().parse::<i64>().unwrap()));
    numbers.sort();
    numbers.push(numbers.last().unwrap()+3);

    let diffs = numbers.iter().zip(numbers.iter().skip(1)).map(|(cur, next)| next - cur).collect::<Vec<i64>>();
    let num1 = diffs.iter().filter(|v| **v == 1).count();
    let num3 = diffs.iter().filter(|v| **v == 3).count();
    println!("Part1 {}", num1 * num3);

    let mut paths: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    for num in &numbers{
        for i in 1..4{
            if numbers.contains(&(num + i)) {
                paths.entry(*num).or_insert(Vec::new()).push(num + i);
            }
        }
    }

    let mut start = 0;
    let mut last = 0;
    let mut combinations = 1;
    for num in numbers {
        if paths.get(&last).unwrap().len() == 1 {
            combinations *= count_paths(start, num, &paths);
            start = num;
        }
        last = num;
    }

    println!("Part2 {}", combinations);
    Ok(())
}

fn count_paths(start: i64, stop: i64, paths: &BTreeMap<i64, Vec<i64>>) -> i64{
    let mut path_count = 0;
    if start == stop {
        return path_count + 1;
    } 
    for i in paths.get(&start).unwrap() {
        path_count += count_paths(*i, stop, paths);
    }
    return path_count;
}
