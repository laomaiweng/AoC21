use std::collections::BTreeMap;

use itertools::Itertools;

use extended_polymerization::{grow_polymer, parse_stdin};

fn main() {
    let (mut polymer, rules) = parse_stdin();
    println!("Template: {}", polymer);
    for i in 1..=10 {
        polymer = grow_polymer(polymer, &rules);
        if polymer.len() < 100 {
            println!("After step {}: {}", i, polymer);
        } else {
            println!("After step {}: (polymer too long: {} characters)", i, polymer.len());
        }
    }

    // invert counts and store into a btreemap for sorted iteration
    let counts = BTreeMap::from_iter(polymer.chars().counts().iter().map(|(k,v)| (*v,*k)));
    println!("Distribution:");
    for (k, v) in counts.iter() {
        println!("  {}: {}", v, k);
    }
}
