use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day11")?;
    let input = BufReader::new(input);

    let rows = input.lines().into_iter().map(|line| line.unwrap().chars().into_iter()
            .map(|c| match c {
                'L' => Seat::EMPTY,
                '.' => Seat::FLOOR,
                _ => panic!(),
            })
            .collect::<Vec<Seat>>())
        .collect();
    println!("Part1 {}", run_until_equilibrium_count_seats(&rows, 1, 4));
    println!("Part2 {}", run_until_equilibrium_count_seats(&rows, rows.len()as i64/2, 5));
    Ok(())
}

fn run_until_equilibrium_count_seats (rows: &Vec<Vec<Seat>>, search_len: i64, occupied_limit: i64) -> usize{
    let mut old = rows.clone();
    loop{
        let mut new = old.clone();
        for (i, row) in old.iter().enumerate(){
            for (j, seat) in row.iter().enumerate(){
                let occupied = sum_seen_occupied(i as i64, j as i64, &old, search_len);
                new[i as usize][j as usize] = match seat {
                    Seat::EMPTY => if occupied == 0 { Seat::OCCUPIED } else { Seat::EMPTY },
                    Seat::OCCUPIED => if occupied >= occupied_limit { Seat::EMPTY } else { Seat::OCCUPIED },
                    Seat::FLOOR => Seat::FLOOR
                };
            }
        }
        if old == new {
            break;
        }
        old = new;
    }
    old.iter().flat_map(|v| v.iter()).filter(|s| **s == Seat::OCCUPIED).count()
}

fn sum_seen_occupied(x: i64, y: i64, rows: &Vec<Vec<Seat>>, search_len: i64) -> i64 {
    let mut occupied = 0;
    for (x_offset, y_offset) in [(1,0), (-1,0), (0,1), (0,-1), (1,1), (-1,1), (1,-1), (-1,-1)].iter(){
        for offset in 1..=search_len{
            let x = x + x_offset*offset;
            let y = y + y_offset*offset;
            //FIXME negative numbers gets casted to large numbers that doesnt exist in Vec
            let done = rows.get(x as usize).and_then(|r| r.get(y as usize)).and_then( |seat| 
                match seat {
                    Seat::OCCUPIED => {
                        occupied += 1;
                        Some(())
                    },
                    Seat::EMPTY => Some(()),
                    Seat::FLOOR => None
                });

            if done.is_some() {
                break;
            }
        }
    }
    occupied
}

#[derive(Clone, Debug, PartialEq)]
enum Seat {
    EMPTY,
    OCCUPIED,
    FLOOR,
}