#[macro_use]
extern crate matrix;

use matrix::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    
    let input = File::open("input_day3")?;
    let input = BufReader::new(input);

    let mut matrix = Compressed::zero((323, 32));


    for (i, line) in input.lines().enumerate() {
        let entry = line.unwrap();
        let chars = entry.chars();

        for (j, c) in chars.enumerate(){
            matrix.set((i,j), c as i64);
        }
    }

    let s1 = traverse_and_sum(&matrix, (1, 1));
    let s2 = traverse_and_sum(&matrix, (1, 3));
    let s3 = traverse_and_sum(&matrix, (1, 5));
    let s4 = traverse_and_sum(&matrix, (1, 7));
    let s5 = traverse_and_sum(&matrix, (2, 1));

    println!("{}", s1 * s2 * s3 * s4 * s5);

    Ok(())
}

fn traverse_and_sum(matrix : &Compressed<i64>, (i_steps, j_steps) : (usize, usize)) -> i64{
    let mut sum_trees = 0;
    let mut j = 0;

    for i in (0..matrix.rows()).step_by(i_steps){
        let c = matrix.get((i,j % 31));
        if c == '#' as i64{
            sum_trees+=1;
        }
        j+=j_steps;
    }
    sum_trees
}