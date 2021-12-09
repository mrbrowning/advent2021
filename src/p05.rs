use std::io::BufRead;

use super::util::*;

pub fn get_digits(line: String) -> Vec<u32> {
    (0..line.len())
        .map(|i| line.chars().nth(i).unwrap().to_digit(2).unwrap())
        .collect::<Vec<u32>>()
}

pub fn run(args: &[String]) {
    let file = get_file(args, 5).unwrap();

    let counts: Vec<Vec<u32>> = file
        .lines()
        .filter_map(|line| line.ok())
        .map(get_digits)
        .collect();
    let places = counts[0].len();

    let gamma_chars: Vec<char> = counts
        .iter()
        .fold((0..places).map(|_| 0).collect::<Vec<u32>>(), |acc, x| {
            (0..places).map(|i| acc[i] + x[i]).collect::<Vec<u32>>()
        })
        .iter()
        .map(|x| {
            if *x > ((counts.len() as u32) / 2) {
                '1'
            } else {
                '0'
            }
        })
        .collect();

    let gamma: String = gamma_chars.iter().collect();
    let epsilon: String = gamma_chars
        .iter()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => {
                println!("WARNING: failed to invert char {}", c);
                '0'
            }
        })
        .collect();
    let answer =
        u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap();

    println!("Answer is: {}", answer);
}
