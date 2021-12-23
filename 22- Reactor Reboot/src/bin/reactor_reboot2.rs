use reactor_reboot::{Reactor, parse_stdin};

fn main() {
    let mut reactor = Reactor::new();
    let steps = parse_stdin();

    for step in &steps {
        reactor.do_step(&step);
    }

    println!("{} reactor cubes on.", reactor.count());
}
