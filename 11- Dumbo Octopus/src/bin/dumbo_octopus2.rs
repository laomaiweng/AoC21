use dumbo_octopus::parse_stdin;

fn main() {
    let mut octopuses = parse_stdin();

    for i in 1..usize::MAX {
        octopuses.step();
        if octopuses.energy() == 0 {
            println!("synchronized flash @ {}", i);
            break;
        }
    }
}
