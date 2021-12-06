use std::collections::HashMap;

type FishMap = HashMap<u8, u64>;

fn distribute(fishlist: lanternfish::FishList) -> FishMap {
    let mut fishmap: HashMap<u8, u64> = HashMap::new();
    for f in &fishlist {
        fishmap.insert(*f, fishmap.get(f).unwrap_or(&0) + 1);
    }
    fishmap
}

fn main() {
    let mut fishes = distribute(lanternfish::parse_stdin());

    let mut total: u64 = 0;
    for _ in 0..256 {
        let mut next_fishes: FishMap = FishMap::new();
        total = 0;
        for (age, count) in &fishes {
            if *age == 0 {
                next_fishes.insert(6, *count + next_fishes.get(&6).unwrap_or(&0));
                next_fishes.insert(8, *count);
                total += count;
            } else {
                next_fishes.insert(*age - 1, *count + next_fishes.get(&(*age - 1)).unwrap_or(&0));
            }
            total += count;
        }
        fishes = next_fishes;
    }

    println!("fishes: {}", total);
}
