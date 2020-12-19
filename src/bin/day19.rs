use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day19")?;
    let input = BufReader::new(input).lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();
    let mut grammar: HashMap<usize, VecDeque<VecDeque<usize>>> = HashMap::new();
    let mut to_validate: VecDeque<VecDeque<usize>> = VecDeque::new();
    let mut found_empty = false;

    let mut literals: HashMap<String, usize> = HashMap::new();

    for line in input{
        if line.is_empty(){
            found_empty = true;
        }
        if !found_empty {
            let parts = line.split(":").collect::<Vec<&str>>();
            let key : usize = parts[0].parse().unwrap();

            let mut v1: VecDeque<VecDeque<usize>> = VecDeque::new();
            if parts[1].contains("|") {
                let parts2 = parts[1].split("|").collect::<Vec<&str>>();
                for p in parts2{
                    let mut v2: VecDeque<usize> = VecDeque::new();
                    let num_strings = p.split_whitespace().collect::<Vec<&str>>();
                    for num_string in num_strings{
                        let num = num_string.parse().unwrap();
                        v2.push_back(num);
                    }
                    v1.push_back(v2);
                }
            } else if parts[1].contains("\""){
                let string = parts[1].replace("\"", "").trim().to_string();
                literals.insert(string, key);
            } else {
                let mut v2: VecDeque<usize> = VecDeque::new();
                let num_strings = parts[1].split_whitespace().collect::<Vec<&str>>();
                for num_string in num_strings{
                    let num = num_string.parse().unwrap();
                    v2.push_back(num);
                }
                v1.push_back(v2);
                
            }
            grammar.insert(key, v1);
        }else {
            let mut row: VecDeque<usize> = VecDeque::new();
            for c in line.chars(){
                row.push_back(literals[&c.to_string()]);
            }
            to_validate.push_back(row);
        }
    }

    let mut count = 0;
    for row in &to_validate{
        let solver = Solver {grammar: grammar.clone(), input: row.clone(), literals: literals.clone()};
        let solution = solver.solve(VecDeque::from(vec!{0}));

        if solution.is_some(){
            count += 1;
        }
    }
    println!("Part 1 {}", count);

    let mut new_8 : VecDeque<VecDeque<usize>> = VecDeque::new();
    new_8.push_back(VecDeque::from(vec!{42}));
    new_8.push_back(VecDeque::from(vec!{42, 8}));
    grammar.insert(8, new_8);
    let mut new_11 : VecDeque<VecDeque<usize>> = VecDeque::new();
    new_11.push_back(VecDeque::from(vec!{42, 31}));
    new_11.push_back(VecDeque::from(vec!{42, 11, 31}));
    grammar.insert(11, new_11);

    let mut count = 0;
    for row in &to_validate{
        let solver = Solver {grammar: grammar.clone(), input: row.clone(), literals: literals.clone()};
        let solution = solver.solve(VecDeque::from(vec!{0}));
        if solution.is_some(){
            count += 1;
        }
    }
    println!("Part 2 {}", count);


    Ok(())
}

enum Result{
    Partial,
    Full,
    Fail
}

struct Solver{
    grammar: HashMap<usize, VecDeque<VecDeque<usize>>>,
    input: VecDeque<usize>,
    literals: HashMap<String, usize>
}

impl Solver{
    fn solve(&self, mut solution: VecDeque<usize>) -> Option<VecDeque<usize>> {
        let mut index = 0;
        for (i, &num) in  solution.iter().enumerate(){
            if !self.literals.values().any(|&v| v == num){
                index = i;
                break;
            }
        }
        
        let ele = solution.remove(index).unwrap();
        let left = &self.grammar[&ele];
        left.iter().find_map(|l| {
            let mut new_solution = solution.clone();
            let mut l = l.clone();
            while let Some(e) = l.pop_back(){
                new_solution.insert(index, e);
            }
             
            match self.check(&new_solution){
                Result::Full => Some(new_solution),
                Result::Partial => self.solve(new_solution),
                Result::Fail => None
            }
        })
    }

    fn check(&self, solution: &VecDeque<usize>) -> Result {
        if self.input == *solution{
            return Result::Full
        }else {
            for (i, &s) in solution.iter().enumerate(){
                if !self.literals.values().any(|&v| v == s){
                    return Result::Partial;
                }
                if solution.len() > self.input.len() || self.input[i] != s{
                    return Result::Fail;
                }
            };
            return Result::Partial;
        }
    }
}