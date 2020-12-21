extern crate itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use itertools::Itertools;
use itertools::iproduct;

fn main() -> io::Result<()> {
    let input = File::open("input_day21")?;
    let input = BufReader::new(input).lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();

    let mut foods : Vec<(HashSet<String>, HashSet<String>)> = Vec::new();
    let mut all_allergens : HashSet<String> = HashSet::new();
    let mut all_ingredients : HashSet<String> = HashSet::new();

    for line in input.iter(){
        let mut in_allergen_part = false;

        let mut allergens = HashSet::new();
        let mut ingredients = HashSet::new();
        for word in line.split_whitespace(){
            if word == "(contains"{
                in_allergen_part = true;
            } else if in_allergen_part{
                let word = word.replace(")", "").replace(",", "");
                allergens.insert(word);
            } else {
                ingredients.insert(word.to_string());
            }
        }
        foods.push((ingredients.clone(), allergens.clone()));
        all_allergens.extend(allergens);
        all_ingredients.extend(ingredients);
    }
    
    let solver = Solver {allergens: &all_allergens, ingredients: &all_ingredients, foods: &foods};
    let solution = solver.solve(&HashMap::new());
    println!("{:?}", solution);

    let solution = solution.unwrap();
    let ingredients_used = solution.values().cloned().collect();
    let ingredients_left = all_ingredients.difference(&ingredients_used).cloned().collect::<Vec<String>>();

    let mut count = 0;
    for (ingredients, _) in foods{
        for i in ingredients {
            if ingredients_left.contains(&i){
                count += 1;
            }
        }
    }

    println!("Part1 {}", count);

    let mut ingredients_list = solution.values().cloned().collect::<Vec<String>>();
    let inverse_map = solution.iter().map(|(a, b)| (b.clone(), a.clone())).collect::<HashMap<String, String>>();
    ingredients_list.sort_by_cached_key(|k| inverse_map[k].clone());
    let string = ingredients_list.iter().join(",");
    println!("Part2 {}", string);

    Ok(())
}

enum Result{
    Partial,
    Full,
    Fail
}

struct Solver <'a>{
    allergens: &'a HashSet<String>,
    ingredients: &'a HashSet<String>,
    foods: &'a Vec<(HashSet<String>, HashSet<String>)>
}

impl<'a> Solver<'a>{
    fn solve(&self, solution: &HashMap<String, String>) -> Option<HashMap<String, String>> {
        let allergenes_used = solution.keys().cloned().collect();
        let allergenes_left = self.allergens.difference(&allergenes_used).cloned().collect::<Vec<String>>();

        let ingredients_used = solution.values().cloned().collect();
        let ingredients_left = self.ingredients.difference(&ingredients_used).cloned().collect::<Vec<String>>();

        let mut left = iproduct!(allergenes_left.iter(), ingredients_left.iter());
        left.find_map(|(a, i)| {
            let mut new_solution = solution.clone();
            new_solution.insert(a.clone(), i.clone());
            
            match self.check(&new_solution){
                Result::Full => Some(new_solution),
                Result::Partial => self.solve(&new_solution),
                Result::Fail => None
            }
        })
    }

    fn check(&self, solution: &HashMap<String, String>) -> Result {
        for (i, allergenes) in self.foods {
            for allergene in allergenes{
                if solution.contains_key(allergene) {
                    if !i.contains(&solution[allergene]) {
                        return Result::Fail;
                    }
                }
            }
            
        }

        if solution.len() == self.allergens.len(){
            return Result::Full;
        } else {
            return Result::Partial;
        }
    }
}