use std::cmp::min;
use std::collections::HashSet;
use std::convert::identity;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::util::*;

pub type Coordinate = (usize, usize);

pub struct Landscape {
    heights: Vec<Vec<u16>>,
}

impl Landscape {
    pub fn get_height(&self, coord: Coordinate) -> u16 {
        self.heights[coord.0][coord.1]
    }

    pub fn get_adjacent_indexes(&self, coord: Coordinate) -> HashSet<Coordinate> {
        // Look, I know, this is over the top. Super slow, too! Just messing around.
        let dec = |x: usize| match x.checked_sub(1) {
            Some(y) => y,
            None => 0,
        };
        let inc = |len: usize| move |x: usize| min(x + 1, len - 1);

        let transforms_i: [Box<dyn Fn(usize) -> usize>; 2] =
            [Box::new(dec), Box::new(inc(self.heights.len()))];
        let transforms_j: [Box<dyn Fn(usize) -> usize>; 2] =
            [Box::new(dec), Box::new(inc(self.heights[0].len()))];

        let adjacent_i = transforms_i.map(|f| (f(coord.0), coord.1));
        let adjacent_j = transforms_j.map(|f| (coord.0, f(coord.1)));

        adjacent_i
            .into_iter()
            .chain(adjacent_j)
            .filter(|c| *c != coord)
            .collect()
    }
}

fn is_minimum(landscape: &Landscape, coord: Coordinate) -> bool {
    let height = landscape.get_height(coord);

    landscape
        .get_adjacent_indexes(coord)
        .iter()
        .map(|c| landscape.get_height(*c) > height)
        .all(identity)
}

pub fn get_minima(landscape: &Landscape) -> Vec<Coordinate> {
    (0..landscape.heights.len())
        .flat_map(|i| {
            // You forget with numeric primitives that, even though they implement Copy, closures
            // still borrow by default, so using a move closure here is necessary to prevent the
            // iterator from outliving i. I get it, but it's still annoying.
            (0..landscape.heights[0].len()).filter_map(move |j| {
                let coord: Coordinate = (i, j);
                match is_minimum(&landscape, coord) {
                    true => Some(coord),
                    false => None,
                }
            })
        })
        .collect()
}

fn parse_row(row: &str) -> Vec<u16> {
    row.chars()
        .map(|c| char::to_digit(c, 10).unwrap() as u16)
        .collect()
}

pub fn parse_landscape(file: BufReader<File>) -> Landscape {
    let mut heights: Vec<Vec<u16>> = vec![];

    for line in file.lines() {
        heights.push(parse_row(&line.unwrap()));
    }

    Landscape { heights }
}

pub fn run(args: &[String]) {
    let file = get_file(args, 17).unwrap();
    let landscape = parse_landscape(file);

    let minima = get_minima(&landscape);
    println!(
        "Answer is {}",
        minima
            .iter()
            .map(|m| landscape.get_height(*m) + 1)
            .sum::<u16>()
    );
}
