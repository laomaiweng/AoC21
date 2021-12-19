use std::cmp;
use std::io::{self, BufRead};

use snailfish::{add, reduce, magnitude, parse_string};

pub fn parse_stdin_raw() -> Vec<String> {
    io::stdin().lock().lines().flatten().collect::<Vec<String>>()
}

fn main() {
    let mut mag = 0;
    // Add/reduce modify the numbers but we can't clone them easily (due to RefCell mostly) to
    // reuse the original numbers in other operations, so start with the raw strings (that we can
    // clone easily) and re-parse for each operation.
    let numbers = parse_stdin_raw();
    for (i, left) in numbers.iter().enumerate() {
        for (j, right) in numbers.iter().enumerate() {
            if i == j { continue; }
            let (mut local_arena, local_numbers) = parse_string(&[left.clone(), right.clone()].join("\n"));
            let (left, right) = (local_numbers[0], local_numbers[1]);
            let result = add(left, right, &mut local_arena);
            reduce(result, &mut local_arena);
            mag = cmp::max(mag, magnitude(result, &local_arena));
        }
    }
    println!("Max magnitude: {}", mag);
}
