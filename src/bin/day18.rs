use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day18")?;
    let input = BufReader::new(input).lines().map(|line| line.unwrap().to_string()).collect::<Vec<String>>();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in input{
        sum1 += calc_part1(&mut line.replace(" ", "").chars().collect::<VecDeque<char>>());
        let stack = shunting_yard(&mut line.replace(" ", "").chars().collect::<VecDeque<char>>());
        sum2 += postfix_calc(stack);
    }
    

    println!("Part 1 {}", sum1);
    println!("Part 2 {}", sum2);

    Ok(())
}

fn calc_part1(input : &mut VecDeque<char>) -> i64{
    let mut sum = 0;
    let mut op = None;

    loop {
        let front = input.pop_front();
        match front{
            Some('(') => {
                let value = calc_part1(input);
                if op.is_none(){
                    sum = value;
                } else {
                    match op{
                        Some('+') => sum += value,
                        Some('*') => sum *= value,
                        _ => panic!()
                    }
                }
            },
            Some(')') => break sum,
            Some('+') => op = Some('+'),
            Some('*') => op = Some('*'),
            Some(c) => {
                let value = c.to_string().parse().unwrap();
                if op.is_none(){
                    sum = value;
                } else {
                    match op{
                        Some('+') => sum += value,
                        Some('*') => sum *= value,
                        _ => panic!()
                    }
                }
            },
            None => break sum
        }
    }
}

fn shunting_yard(input : &mut VecDeque<char>) -> VecDeque<State>{

    let mut stack : VecDeque<State> = VecDeque::new();
    let mut op_stack : VecDeque<State> = VecDeque::new();

    let op_stack_prec = |left: State, right: State| {
        if left == right {
            true
        } else if right == State::LParam {
            false
        } else if left == State::Prod && right == State::Plus{
            true
        } else {
            false
        }
    };

    for token in input{
        match token{
            '*' => {
                while op_stack.len() > 0 && op_stack_prec(State::Prod, *op_stack.back().unwrap()){
                    stack.push_back(op_stack.pop_back().unwrap());
                }
                op_stack.push_back(State::Prod)
            },
            '+' => {
                while op_stack.len() > 0 && op_stack_prec(State::Plus, *op_stack.back().unwrap()){
                    stack.push_back(op_stack.pop_back().unwrap());
                }
                op_stack.push_back(State::Plus)
            },
            '(' => op_stack.push_back(State::LParam),
            ')' => {
                while *op_stack.back().unwrap() != State::LParam {
                    stack.push_back(op_stack.pop_back().unwrap());
                }
                op_stack.pop_back();
            },
            c => {
                let value = c.to_string().parse().unwrap();
                stack.push_back(State::Value(value));
            }
        }
    }

    while op_stack.len() > 0 {
        stack.push_back(op_stack.pop_back().unwrap());
    }

    stack
}

//rewrite to 2 stack calc instead of modding stack?
fn postfix_calc(mut stack : VecDeque<State>) -> i64{
    if stack.len() == 1 {
        if let State::Value(n) = stack.pop_back().unwrap(){
            return n;
        }
    }

    let mut index = 0;
    for (i, &state) in stack.iter().enumerate(){
        if state == State::Plus || state == State::Prod{
            index = i;
            break;
        }
    }
    
    let op = stack.remove(index).unwrap();
    if let State::Value(n2) = stack.remove(index-1).unwrap(){
        if let State::Value(n1) = stack.remove(index-2).unwrap(){
            let res = match op {
                State::Plus => n1 + n2,
                State::Prod => n1 * n2,
                _ => panic!()
            };
            stack.insert(index-2, State::Value(res));
        }
    }

    postfix_calc(stack)
}


#[derive(Clone, Debug, PartialEq, Copy)]
enum State {
    Value(i64),
    Plus,
    Prod,
    LParam
}
