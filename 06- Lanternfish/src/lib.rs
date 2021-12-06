use std::io::{self, BufRead};

pub type FishList = Vec<u8>;

pub fn parse_stdin() -> FishList {
    let mut fishes: Vec<u8> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        fishes.extend(line.unwrap().split(',').map(|n| n.parse::<u8>().unwrap()));
    }

    fishes
}
