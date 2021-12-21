use std::cmp;
use std::collections::BTreeMap;
use std::io::{self, BufRead};
use std::iter::Iterator;

#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn advance<T: Iterator<Item = u32>>(&mut self, die: &mut T) -> u32 {
        let rolls: u32 = (0..3).map(|_| die.next().unwrap()).sum();
        self.advance_by(rolls)
    }

    fn advance_by(&mut self, rolls: u32) -> u32 {
        self.pos = (self.pos + rolls) % 10;
        self.score += self.pos();
        self.score
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    pawns: [Pawn; 2],
    next: usize,
}

impl State {
    fn primary(&self) -> &Pawn {
        &self.pawns[self.primary_index()]
    }

    fn primary_mut(&mut self) -> &mut Pawn {
        &mut self.pawns[self.primary_index()]
    }

    #[inline(always)]
    fn primary_index(&self) -> usize { self.next }

    fn secondary(&self) -> &Pawn {
        &self.pawns[self.secondary_index()]
    }

    #[inline(always)]
    fn secondary_index(&self) -> usize { 1 - self.next }

    fn switch(&mut self) {
        self.next = self.secondary_index();
    }

    /// Generate the new state of the game after the provided rolls.
    fn advance(&self, rolls: &(u32, u32, u32)) -> Self {
        let mut state = *self;
        state.primary_mut().advance_by(rolls.0 + rolls.1 + rolls.2);
        state.switch();
        state
    }
}

impl From<&[Pawn]> for State {
    fn from(pawns: &[Pawn]) -> Self {
        assert!(pawns.len() == 2);
        State {
            pawns: pawns.try_into().unwrap(),
            next: 0,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // Compare the scores of the primary pawns (the ones that will get to play next).
        self.primary().score().cmp(&other.primary().score()).then(
        // If equal, compare the scores of the secondary pawns.
            self.secondary().score().cmp(&other.secondary().score())
        ).then(
        // Otherwise, just compare all the other fields to produce a total ordering, though the
        // *actual* ordering based on those fields shouldn't really matter.
            self.primary().pos().cmp(&other.primary().pos())
        ).then(
            self.secondary().pos().cmp(&other.secondary().pos())
        ).then(
            self.next.cmp(&other.next)
        )
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Distrib {
    states: BTreeMap<State, usize>,
    winners: [usize; 2],
}

impl Distrib {
    pub fn new(pawns: &[Pawn]) -> Self {
        let state = State::from(pawns);
        Distrib {
            states: BTreeMap::from([(state, 1)]),
            winners: [0; 2],
        }
    }

    pub fn winners(&self) -> [usize; 2] { self.winners }

    pub fn is_complete(&self) -> bool { self.states.is_empty() }

    pub fn advance(self, dirac_die: &[(u32, u32, u32)], win: u32) -> Self {
        let mut distrib = Distrib::from(&self);

        // Generate the next distribution by iterating over all states and advancing them by all
        // rolls of the die.
        for (state, count) in &self.states {
            for rolls in dirac_die {
                let next = state.advance(&rolls);
                if next.secondary().score() >= win {
                    // The pawn which just got to play has won.
                    distrib.winners[next.secondary_index()] += count;
                } else {
                    // No winner yet, populate the states map.
                    *distrib.states.entry(next).or_default() += count;
                }
            }
        }

        distrib
    }
}

impl From<&Distrib> for Distrib {
    fn from(distrib: &Distrib) -> Self {
        Distrib {
            states: BTreeMap::new(),
            winners: distrib.winners,
        }
    }
}

pub fn parse_stdin() -> Vec<Pawn> {
    let mut pawns = Vec::new();

    for line in io::stdin().lock().lines() {
        pawns.push(Pawn::new(line.unwrap().split(": ").nth(1).unwrap().parse().unwrap()));
    }

    pawns
}
