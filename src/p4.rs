use std::fs::File;
use std::io::{BufRead, BufReader};

use super::p3::{parse_direction, Direction};

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

    let distances: (i32, i32, i32) = file
        .lines()
        .map(|line| match line {
            Ok(l) => parse_direction(&l),
            Err(_) => {
                println!("WARNING: failed to get line");
                Direction::Down(0)
            }
        })
        .fold((0, 0, 0), |acc, x| match x {
            Direction::Forward(x) => (acc.0 + x, acc.1 + acc.2 * x, acc.2),
            Direction::Down(x) => (acc.0, acc.1, acc.2 + x),
            Direction::Up(x) => (acc.0, acc.1, acc.2 - x),
        });

    println!("Answer is: {}", distances.0 * distances.1);
}
