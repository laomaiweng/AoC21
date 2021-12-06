fn main() {
    let mut fishes = lanternfish::parse_stdin();

    for _ in 0..80 {
        let mut spawn = 0;
        for f in &mut fishes {
            if *f == 0 {
                *f = 6;
                spawn += 1;
            } else {
                *f -= 1;
            }
        }
        fishes.resize(fishes.len() + spawn, 8);
    }

    println!("fishes: {}", fishes.len());
}
