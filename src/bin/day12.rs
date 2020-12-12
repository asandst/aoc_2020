use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("input_day12")?;
    let input = BufReader::new(input);

    let input = input.lines().into_iter().map(|line|{
        let entry = line.unwrap();
        let mut dir = entry; 
        let value = dir.split_off(1).parse().unwrap();
        (dir, value)
    }).collect::<Vec<(String, i64)>>();

    let mut state = State {x: 0, y: 0, x_dir: 1, y_dir: 0};
    for (dir, value) in input.clone() {
        state = match dir.as_str() {
            "N" => State {x: state.x, y: state.y - value, x_dir: state.x_dir, y_dir: state.y_dir},
            "S" => State {x: state.x, y: state.y + value, x_dir: state.x_dir, y_dir: state.y_dir},
            "W" => State {x: state.x - value, y: state.y, x_dir: state.x_dir, y_dir: state.y_dir},
            "E" => State {x: state.x + value, y: state.y, x_dir: state.x_dir, y_dir: state.y_dir},
            "L" | "R" => {
                let (x_dir, y_dir) = rotate(state.x_dir, state.y_dir, &dir, value);
                State {x: state.x, y: state.y, x_dir, y_dir}
            },
            "F" => State {x: state.x + value * state.x_dir, y: state.y + value * state.y_dir, x_dir: state.x_dir, y_dir: state.y_dir},
            _ => panic!()
        };
    }
    println!("Part1 {}", state.x.abs() + state.y.abs());

    let mut waypoint = Waypoint {x: 10, y: -1};
    let mut state = State {x: 0, y: 0, x_dir: 0, y_dir: 0};
    for (dir, value) in input {
        waypoint = match dir.as_str() {
            "N" => Waypoint {x: waypoint.x, y: waypoint.y - value},
            "S" => Waypoint {x: waypoint.x, y: waypoint.y + value},
            "W" => Waypoint {x: waypoint.x - value, y: waypoint.y},
            "E" => Waypoint {x: waypoint.x + value, y: waypoint.y},
            "L" | "R" => {
                let (x, y) = rotate(waypoint.x, waypoint.y, &dir, value);
                Waypoint {x, y}
            },
            "F" => {
                state = State {x: state.x + value * waypoint.x, y: state.y + value * waypoint.y, x_dir: 0, y_dir: 0};
                waypoint
            },
            _ => panic!()
        };
    }
    println!("Part2 {}", state.x.abs() + state.y.abs());
    
    Ok(())
}

fn rotate(x: i64, y:i64, dir: &String, value: i64) -> (i64, i64){
    match dir.as_str() {
        "L" => match value {
            90 => (y, -x),
            180 => (-x, -y),
            270 => (-y, x),
            _ => panic!()
        },
        "R" => match value {
            90 => (-y, x),
            180 => (-x, -y),
            270 => (y, -x),
            _ => panic!()
        },
        _ => panic!()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct State{
    x : i64,
    y : i64,
    x_dir : i64,
    y_dir : i64
}

#[derive(Clone, Debug, PartialEq)]
struct Waypoint{
    x : i64,
    y : i64,
}
