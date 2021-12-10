use std::collections::HashMap;

use syntax_scoring::{TOKEN_INVERSE, Parse, parse_line, parse_stdin};

fn main() {
    let lines = parse_stdin();

    let token_score = HashMap::from([
        (')', 1u64),
        (']', 2u64),
        ('}', 3u64),
        ('>', 4u64),
    ]);

    let mut scores: Vec<u64> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let parse = parse_line(&line, i);
        if let Parse::Incomplete(chunks) = parse {
            let completion: Vec<_> = chunks.iter().rev().map(|c| *TOKEN_INVERSE.get(c).unwrap()).collect();
            let score = completion.iter().fold(0, |score, c| score * 5 + *token_score.get(c).unwrap());
            println!("Completed line {} with \"{}\" (score: {}).", i, completion.iter().collect::<String>(), score);
            scores.push(score);
        }
    }

    scores.sort_unstable();
    println!("middle score: {}", scores[scores.len() / 2]);
}
