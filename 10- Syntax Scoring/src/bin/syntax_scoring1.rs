use std::collections::HashMap;

use syntax_scoring::{Parse, parse_line, parse_stdin};

fn main() {
    let lines = parse_stdin();

    let token_score = HashMap::from([
        (')', 3u64),
        (']', 57u64),
        ('}', 1197u64),
        ('>', 25137u64),
    ]);

    let mut score = 0u64;

    for (i, line) in lines.iter().enumerate() {
        let parse = parse_line(&line, i);
        if let Parse::SyntaxError(c, expected) = parse {
            if let Some(expected) = expected {
                println!("Syntax error on line {}: found '{}', expected '{}'.", i, c, expected);
            } else {
                println!("Syntax error on line {}: found '{}' with no open chunk.", i, c);
            }
            score += token_score.get(&c).unwrap();
        }
    }

    println!("score: {}", score);
}
