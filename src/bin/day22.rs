
use std::collections::VecDeque;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day22")?;
    let input = BufReader::new(input).lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();
    let mut p1 : VecDeque<usize> = VecDeque::new();
    let mut p2 : VecDeque<usize> = VecDeque::new();
    let mut read_p2 = false;

    for line in input{
        if line == "Player 2:"{
            read_p2 = true;
        }else if !read_p2{
            if let Ok(value) = line.parse::<usize>(){
                p1.push_back(value);
            }
        } else {
            if let Ok(value) = line.parse::<usize>(){
                p2.push_back(value);
            }
        }
    }

    println!("Part1 {}", play1(p1.clone(), p2.clone()));
    println!("Part2 {}", play2(p1, p2, 1, HashSet::new()));
    Ok(())
}

fn play1(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> usize{
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

fn play2(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>, depth: usize, mut memory: HashSet<(VecDeque<usize>, VecDeque<usize>)>) -> usize{
    let res = loop {
        if memory.contains(&(p1.clone(), p2.clone())){
            break 1;
        } else {
            memory.insert((p1.clone(), p2.clone()));
        }

        if p1.len() == 0{
            break 2;
        } else if p2.len() == 0{
            break 1;
        }

        let p1_top = p1.pop_front().unwrap();
        let p2_top = p2.pop_front().unwrap();

        if p1_top <= p1.len() && p2_top <= p2.len(){
            let mut p1_new = p1.clone();
            p1_new.truncate(p1_top);
            let mut p2_new = p2.clone();
            p2_new.truncate(p2_top);

            let res = play2(p1_new, p2_new, depth + 1, HashSet::new());
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

fn calc_score(winner : &VecDeque<usize>) -> usize{
    winner.iter().rev().enumerate().map(|(i, card)| (i+1)*card).sum()
}

fn update_decks(p1_win: bool, p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>, p1_top: usize, p2_top: usize){
    if p1_win {
        p1.push_back(p1_top);
        p1.push_back(p2_top);
    } else {
        p2.push_back(p2_top);
        p2.push_back(p1_top);
    }
}