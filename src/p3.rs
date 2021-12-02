use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let filename = match args.len() {
        0 => "raw_data/input2.txt",
        1 => args[0].as_str(),
        _ => args[0].as_str(),
    };

    let raw_file = File::open(filename);
    if raw_file.is_err() {
        println!("failed to load file: {:?}", raw_file);
        return;
    }

    let file = BufReader::new(raw_file.unwrap());

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
