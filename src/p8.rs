use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

use super::p7::*;

pub fn run(args: &[String]) {
    let filename = match args.len() {
        0 => "raw_data/input3.txt",
        1 => args[0].as_str(),
        _ => args[0].as_str(),
    };

    let raw_file = File::open(filename);
    if raw_file.is_err() {
        println!("failed to load file: {:?}", raw_file);
        return;
    }

    let file = BufReader::new(raw_file.unwrap());
    let (mut boards, picks) = parse_game(file);

    let mut losing_boards: HashSet<usize> = (0..boards.len()).collect();
    for call in picks.iter() {
        let mut found = false;

        for (i, board) in boards.iter_mut().enumerate() {
            if !losing_boards.contains(&i) {
                continue;
            }

            board.play(*call);
            if board.is_winning() && losing_boards.len() > 1 {
                losing_boards.remove(&i);
            } else if board.is_winning() {
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    println!(
        "Answer is {}",
        boards[*(losing_boards.iter().next().unwrap())]
            .sum()
            .unwrap()
    );
}
