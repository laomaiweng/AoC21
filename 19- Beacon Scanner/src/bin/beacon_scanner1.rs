use beacon_scanner::{Scanner, parse_stdin};

fn main() {
    let mut unmerged = parse_stdin();
    let mut global = Scanner::new(0);

    let scanner0 = unmerged.swap_remove(0);
    global.merge(&scanner0);
    let mut merged = vec![scanner0];

    while !unmerged.is_empty() {
        let mut collided = None;
        'unmerged: for (i, scanner) in unmerged.iter().enumerate() {
            for candidate in &merged {
                if let Some((rotation, translation)) = candidate.collide(&scanner, 12) {
                    println!("Collided scanners {} & {}!", candidate.index, scanner.index);
                    collided = Some((i, rotation, translation));
                    break 'unmerged;
                }
            }
        }
        if let Some((i, rotation, translation)) = collided {
            let mut scanner = unmerged.swap_remove(i);
            scanner.transform(&rotation, &translation);
            global.merge(&scanner);
            merged.push(scanner);
        }
    }
    println!("Total beacons: {}", global.count());
}
