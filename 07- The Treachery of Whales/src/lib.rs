use std::io::{self, BufRead};

pub type CrabList = Vec<u32>;

pub fn parse_stdin() -> CrabList {
    let mut crabs: Vec<u32> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        crabs.extend(line.unwrap().split(',').map(|n| n.parse::<u32>().unwrap()));
    }

    crabs
}

pub fn sign(a: u32, b: u32) -> &'static str {
    if a < b {
        "<"
    } else if a == b {
        "="
    } else {
        ">"
    }
}
