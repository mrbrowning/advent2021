use std::env;

mod p1;
mod p2;

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
    }
}