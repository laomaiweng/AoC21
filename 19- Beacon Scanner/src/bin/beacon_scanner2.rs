use std::collections::BTreeMap;

use itertools::Itertools;

use beacon_scanner::{large_scanner_collider, parse_stdin};

fn main() {
    // Parse the input scanners and collide them all.
    let mut scanners = parse_stdin();
    large_scanner_collider(&mut scanners);

    // Compute the Manhattan distances between all pairs of scanners.
    let mut manhattan = BTreeMap::new();
    for combo in scanners.iter().combinations(2) {
        let a = combo[0];
        let b = combo[1];
        let dist = a.distance(b);
        manhattan.insert(dist.abs().sum(), (a.index, b.index));
    }
    let (max, (a, b)) = manhattan.iter().rev().next().unwrap();
    println!("Maximum Manhattan distance: {} between {} and {}.", max, a, b);
}
