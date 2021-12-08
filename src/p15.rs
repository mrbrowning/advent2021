use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::util::*;

#[derive(Debug)]
pub struct Readout {
    digits: [String; 10],
    display_value: [String; 4],
}

impl Readout {
    pub fn from(digits: Vec<String>, display_value: Vec<String>) -> Readout {
        Readout {
            digits: digits.try_into().unwrap(),
            display_value: display_value.try_into().unwrap(),
        }
    }

    pub fn deduce_digits(&self) -> Vec<u8> {
        self.display_value
            .iter()
            .filter_map(|d| match d.len() {
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                7 => Some(8),
                _ => None,
            })
            .collect()
    }

    fn get_tags() -> HashMap<u8, char> {
        // The keys represent the sum of the number of times the segments of each digit appear in
        // all 10 digits. Luckily, they end up being unique, and make for an easy tagging system.
        HashMap::from([
            (42, '0'),
            (17, '1'),
            (34, '2'),
            (39, '3'),
            (30, '4'),
            (37, '5'),
            (41, '6'),
            (25, '7'),
            (49, '8'),
            (45, '9'),
        ])
    }

    fn get_counts(&self) -> HashMap<char, u8> {
        let mut counts: HashMap<char, u8> = HashMap::new();

        for s in self.digits.iter() {
            for c in s.chars() {
                counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);
            }
        }

        counts
    }

    fn get_digit_tag(digit: &str, counts: &HashMap<char, u8>) -> u8 {
        digit.chars().map(|c| counts.get(&c).unwrap()).sum()
    }

    pub fn deduce_display(&self) -> u32 {
        let counts = self.get_counts();
        let tags = Readout::get_tags();
        let mut mapping: HashMap<&str, char> = HashMap::new();

        for d in self.digits.iter() {
            let digit = tags.get(&Readout::get_digit_tag(d, &counts)).unwrap();
            mapping.insert(d, *digit);
        }

        self.display_value
            .iter()
            .map(|s| mapping.get(&s as &str).unwrap())
            .collect::<String>()
            .parse::<u32>()
            .unwrap()
    }
}

fn get_strings(line: &str) -> (Vec<String>, Vec<String>) {
    let mut fields = line
        .split(" | ")
        .map(|f| f.split(" ").map(|s| s.to_owned()).collect());

    (fields.next().unwrap(), fields.next().unwrap())
}

pub fn parse_readouts(file: BufReader<File>) -> Vec<Readout> {
    file.lines()
        .map(|line| {
            let (digits, display_value) = get_strings(&line.unwrap());

            Readout::from(digits, display_value)
        })
        .collect()
}

pub fn run(args: &[String]) {
    let file = get_file(args, 15).unwrap();

    let readouts = parse_readouts(file);
    let mut counts: HashMap<u8, u32> = HashMap::new();
    for r in readouts.iter() {
        for d in r.deduce_digits() {
            counts.insert(d, counts.get(&d).unwrap_or(&0) + 1);
        }
    }

    let known_digits: [u8; 4] = [1, 4, 7, 8];

    println!(
        "Answer is {}",
        known_digits
            .iter()
            .map(|d| counts.get(d).unwrap_or(&0))
            .sum::<u32>()
    );
}
