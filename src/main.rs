use std::env;

mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;

fn announce(exercise: u8) {
    println!("Running problem {}", exercise);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let exercise = args[1].as_str();
    let args = &args[2..];

    if exercise == "1" {
        announce(1);
        p1::run(args);
    } else if exercise == "2" {
        announce(2);
        p2::run(args);
    } else if exercise == "3" {
        announce(3);
        p3::run(args);
    } else if exercise == "4" {
        announce(4);
        p4::run(args);
    } else if exercise == "5" {
        announce(5);
        p5::run(args);
    } else if exercise == "6" {
        announce(6);
        p6::run(args);
    } else if exercise == "7" {
        announce(7);
        p7::run(args);
    } else if exercise == "8" {
        announce(8);
        p8::run(args);
    }
}
