use std::collections::{HashMap, HashSet};

use beacon_scanner::{Scanner, parse_stdin};

fn main() {
    // Parse the input scanners and initialize the global view.
    let mut unmerged = parse_stdin();
    let mut global = Scanner::new(unmerged.len() as u32);

    // Merge scanner 0 into the global view, it will be our reference scanner.
    let scanner0 = unmerged.swap_remove(0);
    global.merge(&scanner0);
    let mut merged = vec![scanner0];

    // Remember scanners that didn't match so we don't attempt to match them again.
    let mut all_mismatches = HashMap::new();

    // Iterate over unmerged scanners, trying to match them against any merged scanner.
    // Once we find a match, merge the scanner into the global view and move it to the list of
    // merged scanners.
    while !unmerged.is_empty() {
        let mut collided = None;
        'unmerged: for (i, scanner) in unmerged.iter().enumerate() {
            let mismatches = all_mismatches.entry(scanner.index).or_insert(HashSet::new());
            for candidate in &merged {
                if mismatches.contains(&candidate.index) {
                    continue;
                }
                if let Some((rotation, translation)) = candidate.collide(&scanner, 12) {
                    println!("Collided scanners {} & {}!", candidate.index, scanner.index);
                    // Remember the index so we can move the scanner from unmerged to merged
                    // outside of the loops.
                    collided = Some((i, rotation, translation));
                    break 'unmerged;
                } else {
                    mismatches.insert(candidate.index);
                }
            }
        }
        if let Some((i, rotation, translation)) = collided {
            // A scanner matched: transform it, merge it and move it.
            let mut scanner = unmerged.swap_remove(i);
            scanner.transform(&rotation, &translation);
            global.merge(&scanner);
            merged.push(scanner);
        }
    }
    println!("Total beacons: {}", global.count());
}
