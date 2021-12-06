use std::collections::HashSet;

use super::p7::*;
use super::util::*;

pub fn run(args: &[String]) {
    let file = get_file(args, 8).unwrap();

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
