use beacon_scanner::{Scanner, large_scanner_collider, parse_stdin};

fn main() {
    // Parse the input scanners and collide them all.
    let mut scanners = parse_stdin();
    large_scanner_collider(&mut scanners);

    // Merge all scanners into a global view.
    let mut global = Scanner::new(scanners.len() as u32);
    for scanner in &scanners {
        global.merge(scanner);
    }
    println!("Total beacons: {}", global.count());
}
