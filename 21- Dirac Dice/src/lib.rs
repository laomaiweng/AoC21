use std::io::{self, BufRead};

pub struct Pawn {
    pos: u32,
    score: u32,
}

impl Pawn {
    pub fn new(pos: u32) -> Self {
        Pawn {
            pos: pos-1,
            score: 0,
        }
    }

    pub fn pos(&self) -> u32 { self.pos + 1 }
    pub fn score(&self) -> u32 { self.score }

    pub fn advance<T: std::iter::Iterator<Item = u32>>(&mut self, die: &mut T) -> u32 {
        let adv: u32 = (0..3).map(|_| die.next().unwrap()).sum();
        self.pos = (self.pos + adv) % 10;
        self.score += self.pos();
        self.score
    }
}

pub fn parse_stdin() -> Vec<Pawn> {
    let mut pawns = Vec::new();

    for line in io::stdin().lock().lines() {
        pawns.push(Pawn::new(line.unwrap().split(": ").nth(1).unwrap().parse().unwrap()));
    }

    pawns
}
