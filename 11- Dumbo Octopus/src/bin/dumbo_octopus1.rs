use dumbo_octopus::parse_stdin;

fn main() {
    let mut octopuses = parse_stdin();

    let flashes = (0..100).fold(0, |flashes, _| flashes + octopuses.step());

    println!("flashes: {}", flashes);
}
