use std::io::BufRead;

use super::p3::{parse_direction, Direction};
use super::util::*;

pub fn run(args: &[String]) {
    let file = get_file(args, 4).unwrap();

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
