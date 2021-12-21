use dirac_dice::parse_stdin;

fn main() {
    let mut pawns = parse_stdin();
    println!("Pawn 1 starts at {}, pawn 2 starts at {}.", pawns[0].pos(), pawns[1].pos());

    let mut rolls = 0;
    let mut die = (1..=100).cycle();
    'infinite: loop {
        for (i, p) in pawns.iter_mut().enumerate() {
            rolls += 3;
            p.advance(&mut die);
            println!("Pawn {} moves to {}. Score: {}", i, p.pos(), p.score());
            if p.score() >= 1000 { break 'infinite; }
        }
    }

    println!("The game ended after {} rolls.", rolls);
}
