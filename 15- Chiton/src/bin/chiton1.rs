use chiton::parse_stdin;

fn main() {
    let mut map = parse_stdin();
    println!("Loaded {}x{} map.", map.rows(), map.cols());
    map.solve();
}
