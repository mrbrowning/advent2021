use std::io::BufRead;

use super::util::*;

pub enum Question {
    First,
    Second,
}

fn fuel_cost(steps: i64, question: &Question) -> i64 {
    match question {
        Question::First => steps,
        Question::Second => steps * (steps + 1) / 2,
    }
}

pub fn minimum_fuel(crabs: &Vec<i64>, question: &Question) -> i64 {
    let min = *(crabs.iter().min().unwrap());
    let max = *(crabs.iter().max().unwrap());

    // theta(n^2), but I don't think there's a closed form solution for this, since we need to keep
    // track of the sign of each distance
    (min..=max)
        .map(|x| {
            crabs
                .iter()
                .map(|y| fuel_cost((*y - x).abs(), question))
                .sum()
        })
        .min()
        .unwrap()
}

pub fn parse_crabs(line: &str) -> Vec<i64> {
    line.split(",")
        .map(|s| s.to_owned().parse::<i64>().unwrap())
        .collect()
}

pub fn run(args: &[String]) {
    let file = get_file(args, 13).unwrap();
    let crabs = parse_crabs(&file.lines().next().unwrap().unwrap());

    println!("Answer is {}", minimum_fuel(&crabs, &Question::First));
}
