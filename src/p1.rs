use std::io::BufRead;

use super::util::*;

pub fn run(args: &[String]) {
    let file = get_file(args, 1).unwrap();

    let numbers: Vec<i32> = file
        .lines()
        .filter_map(|line| match line {
            Ok(l) => l.parse::<i32>().ok(),
            Err(e) => {
                println!("WARNING: lines failed: {:?}", e);
                None
            }
        })
        .collect();

    let increasing_count = numbers[1..]
        .iter()
        .enumerate()
        .map(|(i, x)| x > &numbers[i])
        .fold(0, |acc, p| if p { acc + 1 } else { acc });

    println!("Answer is: {}", increasing_count);
}
