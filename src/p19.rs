use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::BufRead;

use super::util::*;

const OPENING_DELIMS: [char; 4] = ['(', '<', '{', '['];
const CLOSING_DELIMS: [char; 4] = [')', '>', '}', ']'];

#[derive(Debug)]
pub enum ChallengeError {
    Parse(Option<char>),
    Omission(Vec<char>),
}

impl fmt::Display for ChallengeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ChallengeError::Parse(p) => {
                let repr = match p {
                    Some(c) => c.to_string(),
                    None => "<EOF>".to_owned(),
                };
                format!("Got unexpected {}", repr)
            }
            ChallengeError::Omission(o) => {
                format!("Unfinished line {}", o.iter().collect::<String>())
            }
        };

        write!(f, "{}", message)
    }
}

impl Error for ChallengeError {}

pub struct DelimMatcher {
    delim_stack: Vec<char>,
    pairs: HashMap<char, char>,
}

impl DelimMatcher {
    pub fn new() -> Self {
        let mut pairs: HashMap<char, char> = HashMap::new();
        for i in 0..4 {
            pairs.insert(OPENING_DELIMS[i], CLOSING_DELIMS[i]);
        }
        DelimMatcher {
            delim_stack: vec![],
            pairs,
        }
    }

    pub fn parse_line(mut self, line: &str) -> Result<(), ChallengeError> {
        for c in line.chars() {
            if self.pairs.contains_key(&c) {
                self.delim_stack.push(*self.pairs.get(&c).unwrap());
            } else {
                let closing_delim = self.delim_stack.pop().map(|d| d == c);

                match closing_delim {
                    Some(true) => {
                        continue;
                    }
                    Some(false) => {
                        return Err(ChallengeError::Parse(Some(c)));
                    }
                    None => {
                        return Err(ChallengeError::Parse(None));
                    }
                };
            }
        }

        if self.delim_stack.len() > 0 {
            return Err(ChallengeError::Omission(self.format_stack()));
        }

        Ok(())
    }

    fn format_stack(self) -> Vec<char> {
        self.delim_stack.into_iter().rev().collect()
    }
}

pub fn run(args: &[String]) {
    let file = get_file(args, 19).unwrap();

    let score_table: HashMap<char, u64> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let erroneous_delims: Vec<char> = file
        .lines()
        .map(|line| DelimMatcher::new().parse_line(&line.unwrap()))
        .filter_map(|parse| {
            parse.err().and_then(|pe| match pe {
                ChallengeError::Parse(p) => p,
                _ => None,
            })
        })
        .collect();

    println!(
        "Answer is {}",
        erroneous_delims
            .iter()
            .map(|c| *score_table.get(c).unwrap())
            .sum::<u64>()
    )
}
