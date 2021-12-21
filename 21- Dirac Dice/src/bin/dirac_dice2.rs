use itertools::iproduct;

use dirac_dice::{Distrib, parse_stdin};

const WIN: u32 = 21;

fn main() {
    let pawns = parse_stdin();
    println!("Pawn 1 starts at {}, pawn 2 starts at {}.", pawns[0].pos(), pawns[1].pos());

    let dirac_die: Vec<(u32, u32, u32)> = iproduct!(1..=3, 1..=3, 1..=3).collect();

    let mut iterations = 0;
    let mut distrib = Distrib::new(&pawns);
    while !distrib.is_complete() {
        iterations += 1;
        distrib = distrib.advance(&dirac_die, WIN);
    }

    let winners = distrib.winners();
    println!("The meta-game ended after {} iterations.", iterations);
    let w = if winners[0] > winners[1] { 0 } else { 1 };
    let l = 1-w;
    println!("Pawn {} wins in {} universes, while pawn {} merely wins in {}.", w+1, winners[w], l+1, winners[l]);
}
