use std::env;

mod util;

mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;
mod p11;
mod p13;
mod p14;
mod p15;
mod p16;
mod p17;
mod p18;
mod p19;
mod p20;
mod p21;
mod p22;

fn announce(exercise: u8) {
    println!("Running problem {}", exercise);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let exercise = args[1].as_str();
    let args = &args[2..];

    if exercise == "1" {
        announce(1);
        p01::run(args);
    } else if exercise == "2" {
        announce(2);
        p02::run(args);
    } else if exercise == "3" {
        announce(3);
        p03::run(args);
    } else if exercise == "4" {
        announce(4);
        p04::run(args);
    } else if exercise == "5" {
        announce(5);
        p05::run(args);
    } else if exercise == "6" {
        announce(6);
        p06::run(args);
    } else if exercise == "7" {
        announce(7);
        p07::run(args);
    } else if exercise == "8" {
        announce(8);
        p08::run(args);
    } else if exercise == "9" {
        announce(9);
        p09::run(args);
    } else if exercise == "11" {
        announce(11);
        p11::run(args);
    } else if exercise == "13" {
        announce(13);
        p13::run(args);
    } else if exercise == "14" {
        announce(14);
        p14::run(args);
    } else if exercise == "15" {
        announce(15);
        p15::run(args);
    } else if exercise == "16" {
        announce(16);
        p16::run(args);
    } else if exercise == "17" {
        announce(17);
        p17::run(args);
    } else if exercise == "18" {
        announce(18);
        p18::run(args);
    } else if exercise == "19" {
        announce(19);
        p19::run(args);
    } else if exercise == "20" {
        announce(20);
        p20::run(args);
    } else if exercise == "21" {
        announce(21);
        p21::run(args);
    } else if exercise == "22" {
        announce(22);
        p22::run(args);
    }
}
