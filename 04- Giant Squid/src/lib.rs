use std::io::{self, BufRead};

pub mod bingo;

use crate::bingo::{Bingo, Number};

pub fn parse_stdin() -> (Vec<u8>, Vec<Bingo>) {
    let mut numbers: Vec<u8> = Vec::new();
    let mut grids: Vec<Bingo> = Vec::new();

    let mut gridlines: usize = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if numbers.is_empty() {
            numbers = line.trim().split(',').map(|n| n.parse::<u8>().unwrap()).collect();
        } else if !line.trim().is_empty() {
            if gridlines % 5 == 0 {
                // new grid
                grids.push(Bingo::new());
            }
            let bingo = grids.last_mut().unwrap();
            for (i, n) in line.split_whitespace().map(|n| n.parse::<u8>().unwrap()).enumerate() {
                bingo.grid[gridlines % 5][i] = Number::new(n);
            }
            gridlines += 1;
        }
    }

    println!("numbers: {}", numbers.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","));
    println!("# grids: {}", grids.len());

    (numbers, grids)
}
