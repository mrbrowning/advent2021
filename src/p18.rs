use super::p17::*;
use super::util::*;
use std::collections::HashSet;
use std::collections::VecDeque;

fn get_basin(landscape: &Landscape, coord: Coordinate) -> HashSet<Coordinate> {
    let mut queue: VecDeque<Coordinate> = VecDeque::from([coord]);
    let mut seen: HashSet<Coordinate> = HashSet::new();

    while queue.len() > 0 {
        let current_coord = queue.pop_front().unwrap();
        seen.insert(current_coord);

        for n in landscape.get_adjacent_indexes(current_coord) {
            if seen.contains(&n) || landscape.get_height(n) >= 9 {
                continue;
            }
            queue.push_back(n);
        }
    }

    seen
}

pub fn run(args: &[String]) {
    let file = get_file(args, 18).unwrap();
    let landscape = parse_landscape(file);

    let basins: Vec<HashSet<Coordinate>> = get_minima(&landscape)
        .iter()
        .map(|m| get_basin(&landscape, *m))
        .collect();

    let mut basin_lengths: Vec<usize> = basins.iter().map(|b| b.len()).collect();
    basin_lengths.sort_by(|a, b| b.cmp(a));

    let answer: u32 = basin_lengths[..3]
        .iter()
        .map(|x| *x as u32)
        .fold(1, |acc, x| acc * x);
    println!("Answer is {}", answer);
}
