use std::io::BufRead;

use super::p05::get_digits;
use super::util::*;

pub fn run(args: &[String]) {
    let file = get_file(args, 6).unwrap();

    let lines: Vec<Vec<u32>> = file
        .lines()
        .filter_map(|line| line.ok())
        .map(get_digits)
        .collect();
    let places = lines[0].len();

    let mut remaining: Vec<&Vec<u32>> = lines.iter().collect();
    for i in 0..places {
        let total = remaining.len() as u32;
        let counts_1 = remaining
            .iter()
            .fold((0..places).map(|_| 0).collect::<Vec<u32>>(), |acc, x| {
                (0..places).map(|i| acc[i] + x[i]).collect::<Vec<u32>>()
            });
        let counts_0: Vec<u32> = counts_1.iter().map(|x| total - x).collect();

        let most_common = if counts_1[i] >= counts_0[i] { 1 } else { 0 };

        remaining = remaining
            .iter()
            .filter(|xs| xs[i] == most_common)
            .map(|xs| *xs)
            .collect();
        if remaining.len() == 1 {
            break;
        }
    }
    let generator_rating: u32 = u32::from_str_radix(
        &(remaining[0]
            .iter()
            .map(|x| char::from_digit(*x, 2).unwrap())
            .collect::<String>()),
        2,
    )
    .unwrap();

    remaining = lines.iter().collect();
    for i in 0..places {
        let total = remaining.len() as u32;
        let counts_1 = remaining
            .iter()
            .fold((0..places).map(|_| 0).collect::<Vec<u32>>(), |acc, x| {
                (0..places).map(|i| acc[i] + x[i]).collect::<Vec<u32>>()
            });
        let counts_0: Vec<u32> = counts_1.iter().map(|x| total - x).collect();

        let least_common = if counts_0[i] <= counts_1[i] { 0 } else { 1 };

        remaining = remaining
            .iter()
            .filter(|xs| xs[i] == least_common)
            .map(|xs| *xs)
            .collect();
        if remaining.len() == 1 {
            break;
        }
    }
    let scrubber_rating: u32 = u32::from_str_radix(
        &(remaining[0]
            .iter()
            .map(|x| char::from_digit(*x, 2).unwrap())
            .collect::<String>()),
        2,
    )
    .unwrap();

    println!("Answer is {}", generator_rating * scrubber_rating);
}
