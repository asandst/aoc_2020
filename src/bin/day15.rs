use std::collections::HashMap;
use std::io;
use std::fs;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input_day15").unwrap();
    let input = input.split(",").map(|p| p.parse().unwrap()).collect::<Vec<usize>>();

    let mut turn = 1;
    let mut numbers : HashMap<usize, State> = HashMap::new();
    let mut last = (0, &State {last: 0, old: 0});

    for &num in input.iter(){
        last = (num, numbers.entry(num).or_insert(State {last: turn, old: 0}));
        turn += 1;
    }
    
    loop {
        let key = last.1.get_key();
        last = (key, numbers.entry(key).and_modify(|e| e.update(turn)).or_insert(State {last: turn, old: 0}));

        if turn == 2020{
            println!("Part 1 {}", last.0);
        } else if turn == 30000000{
            println!("Part 2 {}", last.0);
            break;
        }
        turn += 1;
    }

    Ok(())
}

struct State {
    last: usize,
    old : usize,
}

impl State {
    fn update(&mut self, turn: usize) {
        self.old = self.last;
        self.last = turn;
    }

    fn get_key(&self) -> usize {
        if self.old == 0 {
            0
        } else {
            (self.last - self.old) as usize
        }
    }
}