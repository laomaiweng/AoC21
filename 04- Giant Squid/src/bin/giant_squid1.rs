use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    let (numbers, mut grids) = giant_squid::parse_stdin();

    let mut won = false;
    for n in numbers {
        eprintln!("ticking: {}", n);
        for (i, g) in grids.iter_mut().enumerate() {
            if g.tick(n) {
                println!("grid #{} won, score {}", i, g.score() * (n as u32));
                won = true;
            }
        }
        if won {
            break;
        }
    }
    Ok(())
}
