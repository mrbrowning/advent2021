use std::collections::HashMap;
use std::io::BufRead;

use super::p19::*;
use super::util::*;

fn get_score(chars: &Vec<char>, score_table: &HashMap<char, u64>) -> u64 {
    chars
        .iter()
        .fold(0, |acc, c| acc * 5 + score_table.get(c).unwrap())
}

pub fn run(args: &[String]) {
    let file = get_file(args, 20).unwrap();

    let score_table: HashMap<char, u64> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let omitted_delims: Vec<Vec<char>> = file
        .lines()
        .map(|line| DelimMatcher::new().parse_line(&line.unwrap()))
        .filter_map(|parse| {
            parse.err().and_then(|pe| match pe {
                ChallengeError::Omission(o) => Some(o),
                _ => None,
            })
        })
        .collect();

    let mut scores: Vec<u64> = omitted_delims
        .iter()
        .map(|xs| get_score(xs, &score_table))
        .collect();
    scores.sort();

    println!("Answer is {}", scores[scores.len() / 2]);
}
