use regex::Regex;
use std::io::BufRead;

use super::util::*;

pub enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn parse_direction(direction: &str) -> Direction {
    let parser = Regex::new("(forward|up|down) ([0-9]+)").unwrap();
    let direction = match parser.captures(direction) {
        Some(c) => (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str()),
        None => {
            println!("WARNING: failed to parse line {}", direction);
            ("", "")
        }
    };

    let try_amount = direction.1.parse::<i32>();
    let amount = match try_amount {
        Ok(x) => x,
        Err(_) => {
            println!("WARNING: failed to parse direction amount {}", direction.1);
            0
        }
    };

    match direction.0 {
        "forward" => Direction::Forward(amount),
        "down" => Direction::Down(amount),
        "up" => Direction::Up(amount),
        _ => {
            println!("WARNING: Failed to match direction {}", direction.0);
            Direction::Down(0)
        }
    }
}

pub fn run(args: &[String]) {
    let file = get_file(args, 3).unwrap();

    let distances: (i32, i32) = file
        .lines()
        .map(|line| match line {
            Ok(l) => parse_direction(&l),
            Err(_) => {
                println!("WARNING: failed to get line");
                Direction::Down(0)
            }
        })
        .fold((0, 0), |acc, x| match x {
            Direction::Forward(x) => (acc.0 + x, acc.1),
            Direction::Down(x) => (acc.0, acc.1 + x),
            Direction::Up(x) => (acc.0, acc.1 - x),
        });

    println!("Answer is: {}", distances.0 * distances.1);
}
