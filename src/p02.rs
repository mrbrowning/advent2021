use std::io::BufRead;

use super::util::*;

pub fn run(args: &[String]) {
    let file = get_file(args, 2).unwrap();

    let numbers: Vec<f64> = file
        .lines()
        .filter_map(|line| match line {
            Ok(l) => l.parse::<f64>().ok(),
            Err(e) => {
                println!("WARNING: lines failed: {:?}", e);
                None
            }
        })
        .collect();

    let running_averages = numbers[2..]
        .iter()
        .enumerate()
        .map(|(i, x)| (numbers[i] + numbers[i + 1] + x) / 3.0)
        .collect::<Vec<f64>>();

    let increasing_count = running_averages[1..]
        .iter()
        .enumerate()
        .map(|(i, x)| x > &running_averages[i])
        .fold(0, |acc, p| if p { acc + 1 } else { acc });

    println!("Answer is: {}", increasing_count);
}
