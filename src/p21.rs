use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

use super::util::*;

type Coordinate = (usize, usize);

pub struct OctopusPlane {
    octopodes: Vec<Vec<u8>>,
    flashes: u32,
}

impl OctopusPlane {
    fn adjacent_indexes(&self, coord: Coordinate) -> Vec<Coordinate> {
        // Have to cast to a signed type, because -1 isn't a usize.
        // Again, I get it, but it'd be nice to have adding a negative to an unsigned type be
        // interpreted as a legal subtraction.
        let x = coord.0 as i64;
        let y = coord.1 as i64;

        let increments: [i64; 3] = [-1, 0, 1];
        let mut increment_pairs: Vec<(i64, i64)> = increments
            .iter()
            .permutations(2)
            .map(|p| (*p[0], *p[1]))
            .collect();
        increment_pairs.extend([(-1, -1), (1, 1)]);
        
        increment_pairs.into_iter().map(|(i, j)| (x + i, y + j))
            .filter(|p| {
                let i = p.0;
                let j = p.1;
                i >= 0 && j >= 0 && i < self.octopodes.len() as i64 && j < self.octopodes[0].len() as i64
            })
            .map(|(i, j)| (i as usize, j as usize))
            .filter(|p| *p != coord)
            .collect()
    }

    fn get_charged(&self) -> Vec<Coordinate> {
        (0..self.octopodes.len())
            .flat_map(|i| {
                let octopodes = &self.octopodes;
                (0..self.octopodes[0].len()).filter_map(move |j| {
                    if octopodes[i][j] >= 10 {
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn advance(&mut self) {
        for i in 0..self.octopodes.len() {
            for j in 0..self.octopodes[i].len() {
                self.octopodes[i][j] = self.octopodes[i][j] + 1;
            }
        }

        let mut charged: Vec<Coordinate> = self.get_charged();
        let mut flashed: HashSet<Coordinate> = HashSet::new();

        while charged.len() > 0 {
            let adjacent: Vec<Coordinate> = charged
                .iter()
                .flat_map(|coord| self.adjacent_indexes(*coord))
                .collect();

            for coord in charged.iter() {
                let (i, j) = *coord;

                self.octopodes[i][j] = 0;
                flashed.insert(*coord);
                self.flashes += 1;
            }

            for coord in adjacent.iter().filter(|c| !flashed.contains(c)) {
                let (i, j) = *coord;
                self.octopodes[i][j] = self.octopodes[i][j] + 1;
            }

            charged = self.get_charged();
        }
    }

    pub fn did_all_flash(&self) -> bool {
        self.octopodes.iter().all(|row| row.iter().all(|x| *x == 0))
    }
}

pub fn parse_octopodes(file: BufReader<File>) -> OctopusPlane {
    let octopodes: Vec<Vec<u8>> = file
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    OctopusPlane {
        octopodes,
        flashes: 0,
    }
}

pub fn run(args: &[String]) {
    let file = get_file(args, 21).unwrap();
    let mut octopodes = parse_octopodes(file);

    for _ in 0..100 {
        octopodes.advance();
    }

    println!("Answer is {}", octopodes.flashes);
}
