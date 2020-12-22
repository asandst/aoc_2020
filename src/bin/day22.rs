
extern crate fnv;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::time::Instant;
use std::hash::{Hash, Hasher};
use fnv::FnvHashSet;
use fnv::FnvHashMap;
use fnv::FnvHasher;

fn main() -> io::Result<()> {
    let now = Instant::now();
    let input = File::open("input_day22")?;
    let input = BufReader::new(input).lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();
    let mut p1 : VecDeque<u8> = VecDeque::new();
    let mut p2 : VecDeque<u8> = VecDeque::new();
    let mut read_p2 = false;

    for line in input{
        if line == "Player 2:"{
            read_p2 = true;
        }else if !read_p2{
            if let Ok(value) = line.parse::<u8>(){
                p1.push_back(value);
            }
        } else {
            if let Ok(value) = line.parse::<u8>(){
                p2.push_back(value);
            }
        }
    }

    println!("Part1 {}", play1(p1.clone(), p2.clone()));
    println!("Part2 {}", play2(p1, p2, 1, &mut FnvHashMap::default()));
    println!("{}", now.elapsed().as_millis());
    
    Ok(())
}

fn play1(mut p1: VecDeque<u8>, mut p2: VecDeque<u8>) -> usize{
    loop {
        if p1.len() == 0{
            break calc_score(&p2);
        } else if p2.len() == 0{
            break calc_score(&p1);
        }
        let p1_top = p1.pop_front().unwrap();
        let p2_top = p2.pop_front().unwrap();
        update_decks(p1_top > p2_top, &mut p1, &mut p2, p1_top, p2_top);
    }
}

fn play2(mut p1: VecDeque<u8>, mut p2: VecDeque<u8>, depth: usize, global_cache: &mut FnvHashMap<u64, usize>) -> usize{
    let mut local_cache: FnvHashSet<u64> = FnvHashSet::default();

    let res = loop {
        if p1.len() == 0 {
            break 2;
        } else if p2.len() == 0 {
            break 1;
        }

        let mut hasher = FnvHasher::default();
        p1.hash(&mut hasher);
        p2.hash(&mut hasher);
        let hash = hasher.finish();
        if local_cache.contains(&hash){
            break 1;
        } else {
            local_cache.insert(hash);
        }

        let p1_top = p1.pop_front().unwrap();
        let p2_top = p2.pop_front().unwrap();

        if p1_top <= p1.len() as u8 && p2_top <= p2.len() as u8 {
            let p1_new = p1.iter().take(p1_top as usize).cloned().collect::<VecDeque<u8>>();
            let p2_new = p2.iter().take(p2_top as usize).cloned().collect::<VecDeque<u8>>();
            let mut hasher = FnvHasher::default();
            p1_new.hash(&mut hasher);
            p2_new.hash(&mut hasher);
            let hash = hasher.finish();

            let res = if global_cache.contains_key(&hash){
                global_cache[&hash]
            } else {
                let res = play2(p1_new, p2_new, depth + 1, global_cache);
                global_cache.insert(hash, res);
                res
            };
            
            update_decks(res == 1, &mut p1, &mut p2, p1_top, p2_top);
        } else {
            update_decks(p1_top > p2_top, &mut p1, &mut p2, p1_top, p2_top);
        }
    };

    if depth == 1{
        if p1.len() == 0 {
            calc_score(&p2)
        } else {
            calc_score(&p1)
        }
    } else {
        res
    }
}

#[inline]
fn calc_score(winner : &VecDeque<u8>) -> usize{
    winner.iter().rev().enumerate().map(|(i, card)| (i+1)* (*card as usize)).sum()
}

#[inline]
fn update_decks(p1_win: bool, p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>, p1_top: u8, p2_top: u8){
    if p1_win {
        p1.push_back(p1_top);
        p1.push_back(p2_top);
    } else {
        p2.push_back(p2_top);
        p2.push_back(p1_top);
    }
}