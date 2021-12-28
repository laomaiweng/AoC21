use std::io::{self, BufRead};

use amphipod::parse_lines;

static LINES: &str = "  #D#C#B#A#
  #D#B#A#C#"; 

fn main() {
    let mut lines: Vec<String> = io::stdin().lock().lines().flatten().collect();
    for line in LINES.lines() {
        lines.insert(lines.len()-2, line.to_owned());
    }
    let (mut map, pods) = parse_lines(lines);
    map.reset(&pods);
    println!("Initial map:\n{}", map);
    println!("");
    map.clear();

    let solution = map.solve(pods);

    map.reset(solution.unwrap().path.last().unwrap());
    println!("\nSolved:\n{}", map);
}
