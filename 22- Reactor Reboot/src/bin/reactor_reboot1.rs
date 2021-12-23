use reactor_reboot::{Reactor, cube, parse_stdin};

fn main() {
    let mut reactor = Reactor::new();
    let steps = parse_stdin();

    for step in &steps {
        reactor.do_step(&step);
    }

    let bounding_rect = cube(-50..=50, -50..=50, -50..=50);
    println!("{} reactor cubes on in the -50..50 area.", reactor.count_in(&bounding_rect));
}
