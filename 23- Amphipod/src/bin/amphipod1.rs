use amphipod::parse_stdin;

fn main() {
    let (mut map, pods) = parse_stdin();
    map.reset(&pods);
    println!("Initial map:\n{}", map);
    println!("");
    map.clear();

    let solution = map.solve(pods);

    map.reset(solution.unwrap().path.last().unwrap());
    println!("\nSolved:\n{}", map);
}
