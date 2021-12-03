use std::io::{self, BufRead};
use std::error::Error;

const NBITS: usize = 12;

fn main() -> Result<(),Box<dyn Error>> {
    let stdin = io::stdin();

    let mut lines: u32 = 0;
    let mut counts: [u32; NBITS] = [0; NBITS];

    for buffer in stdin.lock().lines() {
        for (i, c) in buffer.unwrap().chars().enumerate() {
            match c {
                '0' => (),
                '1' => counts[i] += 1,
                _ => eprintln!("unexpected bit \"{}\" in position {} on line {}", c, i, lines),
            }
        }

        lines += 1;
    }

    let mut gamma: u32 = 0;

    for (i, c) in counts.iter().enumerate() {
        if *c > lines / 2 {
            gamma += 1 << (NBITS - i - 1);
        }
    }

    let epsilon: u32 = (!gamma) & ((1 << NBITS) - 1);

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);
    Ok(())
}
