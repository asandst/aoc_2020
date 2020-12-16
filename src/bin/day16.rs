extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day16")?;
    let input = BufReader::new(input);
    let input = input.lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();

    let field_valid_ranges_regex = Regex::new(r"([a-z ]+): ([0-9]+)\-([0-9]+) or ([0-9]+)\-([0-9]+)").unwrap();
    let csv_regex = Regex::new(r"([0-9]+,)+[0-9]+").unwrap();
    let mut valid_tickets : Vec<Vec<usize>> = Vec::new();
    let mut ticket_fields : Vec<TicketField> = Vec::new();
    let mut my_ticket = Vec::new();
    let mut invalid_sum = 0;

    for line in &input {
        if field_valid_ranges_regex.is_match(&line){
            let caps = field_valid_ranges_regex.captures(&line).unwrap();
            let name = caps[1].to_string();
            let intervals : Vec<(usize, usize)> = vec!{(caps[2].parse().unwrap(), caps[3].parse().unwrap()), (caps[4].parse().unwrap(), caps[5].parse().unwrap())};
            ticket_fields.push(TicketField {intervals, name})
        } else if csv_regex.is_match(&line){
            let ticket = line.split(",").map(|n| n.parse().unwrap()).collect::<Vec<usize>>();
            if my_ticket.len() > 0 {
                let invalid_values = ticket.iter()
                    .filter(|&value| !ticket_fields.iter().flat_map(|field| field.intervals.iter()).any(|(min, max)| value >= min && value <= max));
                invalid_sum += invalid_values.clone().sum::<usize>();

                if invalid_values.count() == 0{
                    valid_tickets.push(ticket);
                }
            } else {
                my_ticket = ticket.clone();
            }
        }
    }
    println!("Part 1 {}", invalid_sum);

    let mut solver = Solver {map: HashMap::new(), valid_tickets, ticket_fields};
    let solution = solver.solve(&Vec::new());
    let prod = solution.unwrap().iter().enumerate()
        .filter(|(_, &sol)| solver.ticket_fields[sol].name.starts_with("departure"))
        .map(|(i, _)| my_ticket[i])
        .fold(1, |x, y| x*y);
    println!("Part 2 {:?}", prod);
    Ok(())
}

enum Result{
    Partial,
    Full,
    Fail
}

struct TicketField{
    intervals : Vec<(usize, usize)>,
    name : String
}

struct Solver{
    map : HashMap<usize, bool>,
    valid_tickets : Vec<Vec<usize>>,
    ticket_fields : Vec<TicketField>
}

impl Solver{
    fn solve(&mut self, solution: &Vec<usize>) -> Option<Vec<usize>> {
        let mut left : Vec<usize> = (0..self.ticket_fields.len()).into_iter().filter(|i| !solution.contains(i)).collect();
        left.sort_by_cached_key(|&k| self.count(k));
        left.iter().find_map(|&l| {
            let mut new_solution = solution.clone();
            new_solution.push(l);
            match self.check(&new_solution){
                Result::Full => Some(new_solution),
                Result::Partial => self.solve(&new_solution),
                Result::Fail => None
            }
        })
    }

    fn check(&mut self, solution: &Vec<usize>) -> Result {
        if solution.iter().enumerate().any(|(j, &i)| !self.check_column(i, j)){
            Result::Fail
        } else if solution.len() == self.ticket_fields.len(){
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
        (0..self.valid_tickets[0].len()).into_iter().filter(|&i| self.check_column(ticket_field_index, i)).count()
    }
}