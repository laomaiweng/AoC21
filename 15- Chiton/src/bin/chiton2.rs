use chiton::parse_stdin;

fn main() {
    let map = parse_stdin();
    println!("Loaded {}x{} map.", map.rows(), map.cols());
    let mut map = map.tile(5, 5);
    println!("Tiled to {}x{} map.", map.rows(), map.cols());

    map.solve();
}
