use treachery_of_whales::{CrabList, parse_stdin, sign};

fn align_cost(crabs: &CrabList, position: u32) -> u32 {
    let mut cost: u32 = 0;
    for c in crabs {
        let n = if *c > position {
            *c - position
        } else {
            position - *c
        };
        cost += n * (n+1) / 2;
    }
    cost
}

fn main() {
    let mut crabs = parse_stdin();
    crabs.sort();

    let median = crabs[crabs.len()/2];
    let mean = crabs.iter().sum::<u32>() / (crabs.len() as u32);
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    println!("crabs: {}", crabs.len());
    println!("median: {}", median);
    println!("mean: {}", mean);
    println!("min: {}", min);
    println!("max: {}", max);
    println!("");

    let median_cost: Vec<u32> = (median-1..=median+1).map(|n| align_cost(&crabs, n)).collect();
    let s1 = sign(median_cost[0], median_cost[1]);
    let s2 = sign(median_cost[1], median_cost[2]);
    println!("align(median): {} {} {} {} {}", median_cost[0], s1, median_cost[1], s2, median_cost[2]);
    println!("");

    let mean_cost: Vec<u32> = (mean-1..=mean+1).map(|n| align_cost(&crabs, n)).collect();
    let s1 = sign(mean_cost[0], mean_cost[1]);
    let s2 = sign(mean_cost[1], mean_cost[2]);
    println!("align(mean): {} {} {} {} {}", mean_cost[0], s1, mean_cost[1], s2, mean_cost[2]);
    println!("");

    println!("align(min): {}", align_cost(&crabs, min));
    println!("align(max): {}", align_cost(&crabs, max));
}
