use super::p21::*;
use super::util::*;

pub fn run(args: &[String]) {
    let file = get_file(args, 22).unwrap();
    let mut octopodes = parse_octopodes(file);

    let mut did_synchronize = false;
    let mut count = 0;
    while !did_synchronize {
        octopodes.advance();
        did_synchronize = octopodes.did_all_flash();
        count += 1;
    }

    println!("Answer is {}", count);
}
