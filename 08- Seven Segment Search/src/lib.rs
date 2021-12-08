use std::io::{self, BufRead};

pub struct Display {
    pub digits: Vec<String>,
    pub output: Vec<String>,
}

pub fn parse_stdin() -> Vec<Display> {
    let mut displays: Vec<Display> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut line: Vec<Vec<String>> = line.unwrap().split(" | ").map(
            |s| s.split_whitespace().map(str::to_owned).collect()
        ).collect();
        let output = line.swap_remove(1);
        let digits = line.swap_remove(0);
        displays.push(Display {
            digits,
            output,
        });
    }

    displays
}
