extern crate rayon;
use std::collections::VecDeque;
use std::time::Instant;
use std::io::{self};
use rayon::prelude::*;

fn main() -> io::Result<()> {
    let now = Instant::now();

    //let cups = VecDeque::from(vec!{3,8,9,1,2,5,4,6,7});
    let cups = VecDeque::from(vec!{9,6,3,2,7,5,4,8,1});

    let mut current_cups = cups.clone();

    for _m in 0..100{
        let removed_three = current_cups.drain(1..=3).collect::<VecDeque<u32>>();
        let first = current_cups.front().unwrap();

        let mut value = first-1;
        if value == 0{
            value = 9;
        }

        while removed_three.contains(&value) {
            value -= 1;
            if value == 0{
                value = 9;
            }
        }

        let index = current_cups.iter().position(|&e| e == value).unwrap();
        for to_add in removed_three.iter().rev(){
            current_cups.insert(index+1, *to_add);
        }

        current_cups.rotate_left(1);
    }

    println!("Part1 {:?}", current_cups);

    let mut current_cups = cups.clone();

    for i in 10..=1_000_000{
        current_cups.push_back(i);
    }

    for _m in 0..10_000_000{
        let removed_three = current_cups.drain(1..=3).collect::<VecDeque<u32>>();
        let first = current_cups.front().unwrap();

        let mut value = first-1;
        if value == 0{
            value = 1_000_000;
        }

        while removed_three.contains(&value) {
            value -= 1;
            if value == 0{
                value = 1_000_000;
            }
        }

        let index = current_cups.par_iter().position_any(|&e| e == value).unwrap();

        for to_add in removed_three.iter().rev(){
            current_cups.insert(index+1, *to_add);
        }

        current_cups.rotate_left(1);
    }

    let one = current_cups.par_iter().position_any(|&e| e == 1).unwrap();
    let mut index_two = one +1;
    if index_two >= 1_000_000{
        index_two -= 1_000_000;
    }
    let mut index_three = one +2;
    if index_three >= 1_000_000{
        index_three -= 1_000_000;
    }
    let value2 = *current_cups.get(index_two).unwrap() as u64;
    let value3 = *current_cups.get(index_three).unwrap() as u64;
    let res = value2 * value3;
    println!("Part2 {}", res);
    println!("{}", now.elapsed().as_millis());
    
    Ok(())
}