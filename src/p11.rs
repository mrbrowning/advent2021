use std::convert::TryInto;
use std::io::BufRead;

use super::util::*;

struct LanternfishEcosystem {
    fish: [u64; 9],
}

impl LanternfishEcosystem {
    fn new(seed: Vec<u8>) -> LanternfishEcosystem {
        let mut fish: Vec<u64> = (0..9).map(|_| 0).collect();
        for s in seed {
            fish[s as usize] += 1;
        }

        LanternfishEcosystem {
            fish: fish.try_into().unwrap(),
        }
    }

    fn advance(&mut self) {
        let finishing = self.fish[0];
        for i in 1..9 {
            self.fish[i - 1] = self.fish[i];
        }
        self.fish[8] = finishing;
        self.fish[6] += finishing;
    }

    fn count(&self) -> u64 {
        self.fish.iter().sum()
    }
}

fn parse_seed(line: &str) -> Vec<u8> {
    line.split(",").map(|s| s.parse::<u8>().unwrap()).collect()
}

pub fn run(args: &[String]) {
    let file = get_file(args, 11).unwrap();
    let mut fish = LanternfishEcosystem::new(parse_seed(&file.lines().next().unwrap().unwrap()));

    for _ in 0..256 {
        fish.advance();
    }

    println!("Answer is {}", fish.count());
}
