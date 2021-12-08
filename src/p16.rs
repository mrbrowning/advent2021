use super::p15::*;
use super::util::*;

pub fn run(args: &[String]) {
    let file = get_file(args, 16).unwrap();
    let readouts = parse_readouts(file);

    let answer: u32 = readouts.iter().map(|r| r.deduce_display()).sum();
    println!("Answer is {}", answer);
}