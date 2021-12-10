use std::io::{self, BufRead};

use phf::phf_map;

pub enum Parse {
    Ok,
    SyntaxError(char, Option<char>),
    Incomplete(Vec<char>),
    Invalid(char),
}

pub static TOKEN_INVERSE: phf::Map<char, char> = phf_map! {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
    ')' => '(',
    ']' => '[',
    '}' => '{',
    '>' => '<',
};

pub fn parse_line(line: &str, i: usize) -> Parse {
    let mut chunks: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '('|'['|'{'|'<' => chunks.push(c),
            ')'|']'|'}'|'>' => {
                let last = chunks.pop();
                if let Some(last) = last {
                    let expected = *TOKEN_INVERSE.get(&last).unwrap();
                    if c != expected {
                        return Parse::SyntaxError(c, Some(expected));
                    }
                } else {
                    return Parse::SyntaxError(c, None);
                }
            },
            _ => {
                eprintln!("Invalid character on line {}: '{}'.", i, c);
                return Parse::Invalid(c);
            },
        }
    }
    if chunks.is_empty() {
        Parse::Ok
    } else {
        Parse::Incomplete(chunks)
    }
}

pub fn parse_stdin() -> Vec<String> {
    io::stdin().lock().lines().flatten().collect()
}
