extern crate regex;
use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    
    let input = File::open("input_day16")?;
    let input = BufReader::new(input);
    let input = input.lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();

    let field_valid_ranges_regex = Regex::new(r"([a-z ]+): ([0-9]+)\-([0-9]+) or ([0-9]+)\-([0-9]+)").unwrap();
    let csv_regex = Regex::new(r"([0-9]+,)+[0-9]+").unwrap();
    let mut found_nearby_tickets = false;
    let mut intervals : Vec<(usize, usize)> = Vec::new();
    let mut valid_tickets : Vec<Vec<usize>> = Vec::new();
    let mut ticket_fields : Vec<TicketField> = Vec::new();
    let mut my_ticket = Vec::new();
    let mut invalid_sum = 0;

    for line in &input {
        if field_valid_ranges_regex.is_match(&line){
            let caps = field_valid_ranges_regex.captures(&line).unwrap();
            let name = caps.get(1).unwrap().as_str();
            let number_lower_1 :usize= caps.get(2).unwrap().as_str().parse().unwrap();
            let number_upper_1 :usize = caps.get(3).unwrap().as_str().parse().unwrap();

            let number_lower_2 :usize = caps.get(4).unwrap().as_str().parse().unwrap();
            let number_upper_2 :usize = caps.get(5).unwrap().as_str().parse().unwrap();

            intervals.push((number_lower_1, number_upper_1));
            intervals.push((number_lower_2, number_upper_2));

            let mut i : Vec<(usize, usize)> = Vec::new();
            i.push((number_lower_1, number_upper_1));
            i.push((number_lower_2, number_upper_2));

            ticket_fields.push(TicketField {intervals: i, name: name.to_string()})
        } else if csv_regex.is_match(&line){
            let ticket = line.split(",").map(|n| n.parse().unwrap()).collect::<Vec<usize>>();

            if found_nearby_tickets {
                let mut invalid_ticket = false;
                for value in &ticket {
                    let mut ok = false;
                    for (min, max) in &intervals {
                        if value >= min && value <= max {
                            ok = true;
                            break;
                        }
                    }

                    if !ok {
                        invalid_sum += value;
                        invalid_ticket = true;
                    }
                }

                if !invalid_ticket {
                    valid_tickets.push(ticket);
                }
            } else {
                my_ticket = ticket.clone();
                valid_tickets.push(ticket);
            }
        } else if line == "nearby tickets:"{
            found_nearby_tickets = true;
        }
    }
    println!("Part 1 {}", invalid_sum);
    let now = Instant::now();
    let mut solver = Solver {map: HashMap::new(), valid_tickets, ticket_fields};
    let solution = solver.solve(Vec::new());

    let mut prod = 1;
    for (i, sol) in solution.unwrap().iter().enumerate(){
        if sol.name.starts_with("departure"){
            prod *= my_ticket[i];
        }
    }
    println!("Part 2 {:?}", prod);
    println!("{:?}", now.elapsed().as_micros());

    Ok(())
}

struct Solver{
    map : HashMap<usize, bool>,
    valid_tickets : Vec<Vec<usize>>,
    ticket_fields : Vec<TicketField>
}

impl Solver{
    fn solve(&mut self, solution: Vec<usize>) -> Option<Vec<TicketField>> {
        let len = self.ticket_fields.len();
        
        let mut left : Vec<usize> = Vec::new();
        for i in 0..len{
            if !solution.contains(&i) {
                left.push(i);
            }
        }
    
        left.sort_by_key(|&k| self.count(k));
    
        for l in left {
            let mut new_solution = solution.clone();
            new_solution.push(l);
            let res = self.check(&new_solution);
            if res == Result::Full {
                let mut sol : Vec<TicketField> = Vec::new();
                for i in new_solution{
                    sol.push(self.ticket_fields[i].clone());
                }
    
                return Some(sol);
            } else if res == Result::Partial{
                let sol = self.solve(new_solution);
                if sol.is_some(){
                    return sol;
                }
            }
        }
    
        None
    }

    fn check(&mut self, solution: &Vec<usize>) -> Result {
        for (j, &i) in solution.iter().enumerate(){
            if !self.check_column(i, j){
                return Result::Fail;
            }
        }
        
        if solution.len() == self.ticket_fields.len(){
            Result::Full
        } else {
            Result::Partial
        }
    }

    fn check_column(&mut self, ticket_field_index: usize, i: usize) -> bool {
        let ticket_field = &self.ticket_fields[ticket_field_index];
        let key = self.ticket_fields.len()*ticket_field_index + i;
        let valid_tickets = &self.valid_tickets;
        
        *self.map.entry(key).or_insert_with(|| valid_tickets.iter().all(|ticket| ticket_field.intervals.iter().any(|(min, max)| ticket[i] >= *min && ticket[i] <= *max)))
    }

    fn count(&mut self, ticket_field_index: usize) -> usize{
        let mut count = 0;
        for i in 0..self.valid_tickets[0].len(){
            if self.check_column(ticket_field_index, i){
                count += 1;
            }
        }
        count
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Result{
    Partial,
    Full,
    Fail
}

#[derive(Clone, Debug)]
struct TicketField{
    intervals : Vec<(usize, usize)>,
    name : String
}