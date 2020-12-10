use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::BTreeMap;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let input = File::open("input_day10")?;
    let input = BufReader::new(input);
    let mut numbers: Vec<i64> = Vec::new();

    for line in input.lines() {
        let entry = line.unwrap();
        let number = entry.parse::<i64>().unwrap();
        numbers.push(number);
    }
    numbers.push(0);
    numbers.sort();

    let vals = numbers.iter();
    let next_vals = numbers.iter().skip(1);

    let diffs = vals
        .zip(next_vals)
        .map(|(cur, next)| next - cur)
        .collect::<Vec<i64>>();

    let num1 = diffs.iter().filter(|v| **v == 1).count();
    let num3 = diffs.iter().filter(|v| **v == 3).count() + 1;
    println!("Part1 {}", num1 * num3);

    let mut paths: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    let numbers_hash = numbers.iter().map(|v| *v).collect::<HashSet<i64>>();

    for num in &numbers{
        for i in 1..4{
            if numbers_hash.contains(&(num + i)) {
                paths.entry(*num).or_insert(Vec::new()).push(num + i);
            }
        }
    }
    paths.insert(*numbers.last().unwrap(), Vec::new());

    let mut counts: Vec<i64> = Vec::new();
    let mut start = 0;
    let mut last = 0;
    for num in numbers {
        if paths.get(&last).unwrap().len() <= 1 {
            let mut count = 0;
            count_paths_util(start, num, &mut count, &paths);
            counts.push(count);
            start = num;
        }
        last = num;
    }

    let combs = counts.iter().map(|i| *i as i128).fold(1, |x, y| x * y);
    println!("Part2 {}", combs);

    Ok(())
}

fn count_paths_util(start: i64, stop: i64, path_count: &mut i64, paths: &BTreeMap<i64, Vec<i64>>) {
    if start == stop {
        *path_count += 1;
    } else if start > stop{
    }else {
        for i in paths.get(&start).unwrap() {
            count_paths_util(*i, stop, path_count, paths);
        }
    }
}
