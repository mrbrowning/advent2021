use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(args: &[String]) {
    let filename = match args.len() {
        0 => "raw_data/input1.txt",
        1 => args[0].as_str(),
        _ => args[0].as_str(),
    };

    let raw_file = File::open(filename);
    if raw_file.is_err() {
        println!("failed to load file: {:?}", raw_file);
        return;
    }

    let file = BufReader::new(raw_file.unwrap());

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
