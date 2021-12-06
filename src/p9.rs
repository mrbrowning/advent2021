use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// I realize you could model this algebraically, finding the y = mx + b form of each line and then
// solving for mx + b = nx + c and checking if the resulting point is an integer and within the
// line's endpoints, but I was more interested in messing with ranges for my own edification.

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn get_points(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            let start_point = min(self.start.y, self.end.y);
            let end_point = max(self.start.y, self.end.y);

            (start_point..=end_point)
                .map(|y| Point { x: self.start.x, y })
                .collect()
        } else if self.start.y == self.end.y {
            let start_point = min(self.start.x, self.end.x);
            let end_point = max(self.start.x, self.end.x);

            (start_point..=end_point)
                .map(|x| Point { x, y: self.start.y })
                .collect()
        } else {
            let x_range: Vec<u32> = if self.start.x > self.end.x {
                (self.end.x..=self.start.x).rev().collect()
            } else {
                (self.start.x..=self.end.x).collect()
            };
            let y_range: Vec<u32> = if self.start.y > self.end.y {
                (self.end.y..=self.start.y).rev().collect()
            } else {
                (self.start.y..=self.end.y).collect()
            };

            x_range
                .iter()
                .zip(y_range.iter())
                .map(|(x, y)| Point { x: *x, y: *y })
                .collect::<Vec<Point>>()
        }
    }
}

fn parse_point(point: &str) -> Point {
    let mut split = point.split(",");

    Point {
        x: split.next().unwrap().parse::<u32>().unwrap(),
        y: split.next().unwrap().parse::<u32>().unwrap(),
    }
}

pub fn parse_lines(file: BufReader<File>) -> Vec<Line> {
    file.lines()
        .map(|line| {
            line.unwrap()
                .split(" -> ")
                .map(|s| s.to_owned())
                .collect::<Vec<_>>()
        })
        .map(|s| Line {
            start: parse_point(&s[0]),
            end: parse_point(&s[1]),
        })
        .collect()
}

pub fn overlaps(lines: Vec<Line>) -> u32 {
    let mut seen: HashSet<Point> = HashSet::new();
    let mut overlap: HashSet<Point> = HashSet::new();

    for line in lines {
        for point in line.get_points() {
            if seen.contains(&point) {
                overlap.insert(point);
            } else {
                seen.insert(point);
            }
        }
    }

    overlap.len() as u32
}

pub fn run(args: &[String]) {
    let filename = match args.len() {
        0 => "raw_data/input5.txt",
        1 => args[0].as_str(),
        _ => args[0].as_str(),
    };

    let raw_file = File::open(filename);
    if raw_file.is_err() {
        println!("failed to load file: {:?}", raw_file);
        return;
    }

    let file = BufReader::new(raw_file.unwrap());
    let lines = parse_lines(file);

    println!("Answer is {}", overlaps(lines));
}
