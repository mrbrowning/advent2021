use std::io::BufRead;

use super::p13::{minimum_fuel, parse_crabs, Question};
use super::util::*;

pub fn run(args: &[String]) {
    let file = get_file(args, 14).unwrap();
    let crabs = parse_crabs(&file.lines().next().unwrap().unwrap());

    println!("Answer is {}", minimum_fuel(&crabs, &Question::Second));
}
