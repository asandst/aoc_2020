use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let input = File::open("input_day9")?;
    let input = BufReader::new(input);
    let mut numbers: Vec<i64> = Vec::new();

    for line in input.lines() {
        let entry = line.unwrap();
        let number = entry.parse::<i64>().unwrap();
        numbers.push(number);
    }

    let numbers = numbers;
    let mut deque: VecDeque<i64> = VecDeque::new();
    let mut part1 = 0;
    for number in numbers.iter() {
        deque.push_back(*number);
        if deque.len() > 25 {
            if deque.iter().any(|i| deque.contains(&(number - i))){
                deque.pop_front().unwrap();
            } else {
                part1 = *number;
                println!("Part1 {}", part1);
                break;
            }
        } 
    }

    let mut sum = 0;
    let mut deque_part2: VecDeque<i64> = VecDeque::new();
    for number in numbers.iter() {
        deque_part2.push_back(*number);
        sum += number;

        while sum > part1 {
            sum -= deque_part2.pop_front().unwrap();
        }

        if sum == part1{
            let min = deque_part2.iter().min().unwrap();
            let max = deque_part2.iter().max().unwrap();
            println!("Part2 {}", min + max);
            break;
        }
    }

    Ok(())
}
