use std::collections::{BTreeMap, HashMap};

use extended_polymerization::{compute_distrib, grow_polymer_distrib, parse_stdin};

fn main() {
    let (template, rules) = parse_stdin();
    println!("Template: {}", template);
    let mut polymer = compute_distrib(&template);
    for i in 1..=40 {
        polymer = grow_polymer_distrib(polymer, &rules);
        println!("Polymer length after step {}: {}", i, polymer.values().sum::<usize>()+1);
    }

    // count characters: take the last character in each pair (don't count the first character in
    // each pair, since each character is present in 2 pairs), and add the first character of the
    // template (which is unchanged throughout the polymer growth)
    let mut counts: HashMap<char, usize> = HashMap::new();
    for ((_,r), count) in polymer.iter() {
        *counts.entry(*r).or_insert(0) += count;
    }
    *counts.entry(template.chars().next().unwrap()).or_insert(0) += 1;

    let counts = BTreeMap::from_iter(counts.iter().map(|(k,v)| (*v,*k)));
    println!("Distribution:");
    for (k, v) in counts.iter() {
        println!("  {}: {}", v, k);
    }
}
