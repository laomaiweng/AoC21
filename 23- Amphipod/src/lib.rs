use std::borrow::Borrow;
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::io::{self, BufRead};

use grid::Grid;

type Position = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum State {
    Initial,
    Hallway,
    Final,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Amphipod {
    kind: char,
    pos: Position,
    state: State,
}

impl Amphipod {
    fn new(kind: char, pos: Position) -> Self {
        Amphipod {
            kind,
            pos,
            // Assume the pod isn't in the right burrow to begin with, or can't stay in it.
            state: State::Initial,
        }
    }

    pub fn is_settled(&self) -> bool {
        if self.state == State::Final { assert!(self.pos.0 != 1); }
        self.state == State::Final
    }

    pub fn cost(&self) -> u32 {
        10u32.pow((self.kind as u32) - 0x41 /*A*/)
    }

    fn room_index(&self) -> usize {
        ((self.kind as u32) - 0x41 /*A*/) as usize
    }

    fn moved(&self, pos: Position) -> Self {
        let mut p = self.clone();
        p.pos = pos;
        p.state = match self.state {
            State::Initial => State::Hallway,
            State::Hallway => State::Final,
            State::Final => panic!("Illegal move!"),
        };
        if p.state == State::Final { assert!(p.pos.0 != 1); }
        p
    }
}

#[derive(Debug)]
pub struct Move<'a> {
    pod: &'a Amphipod,
    steps: Vec<Position>,
}

impl<'a> Move<'a> {
    pub fn len(&self) -> usize { self.steps.len() }

    pub fn cost(&self) -> u32 {
        self.steps.len() as u32 * self.pod.cost()
    }

    pub fn last(&self) -> Amphipod {
        self.nth(self.steps.len() - 1)
    }

    pub fn nth(&self, n: usize) -> Amphipod {
        self.pod.moved(self.steps[n])
     }
}

pub type Pods = Vec<Amphipod>;

#[derive(Clone)]
pub struct Path {
    pub path: Vec<Pods>,
    pub moves: u32,
    pub cost: u32,
}

impl Path {
    pub fn new(initial: Pods) -> Self {
        Path {
            path: vec![initial],
            moves: 0,
            cost: 0,
        }
    }

    pub fn add_move(&mut self, movement: Move, index: usize) -> &Pods {
        let mut new_pods = self.path.last().unwrap().clone();
        new_pods[index] = movement.last();
        self.path.push(new_pods);
        self.cost += movement.cost();
        self.moves += movement.len() as u32;
        &self.path.last().unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Invalid,
    Wall,
    Empty,
    Pod(char),
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Invalid
    }
}

#[derive(Debug)]
pub struct Map {
    map: Grid<Cell>,
    rooms: Vec<usize>,
}

impl Map {
    pub fn depth(&self) -> usize {
        self.map.rows() - 1
    }

    pub fn clear(&mut self) {
        // Clear all pods from the map.
        for c in self.map.iter_mut() {
            if let Cell::Pod(_) = *c {
                *c = Cell::Empty;
            }
        }
    }

    pub fn reset(&mut self, pods: &Pods) {
        self.clear();

        // Place the new pods.
        for p in pods {
            assert_eq!(self.map[p.pos.0][p.pos.1], Cell::Empty);
            self.map[p.pos.0][p.pos.1] = Cell::Pod(p.kind);
        }
    }

    pub fn get_moves<'a>(&self, pod: &'a Amphipod) -> Vec<Move<'a>> {
        assert_eq!(self.map[pod.pos.0][pod.pos.1], Cell::Pod(pod.kind));

        return match pod.state {
            State::Initial => {
                self.get_moves_init(pod)
            },

            State::Hallway => {
                self.get_moves_hallway(pod)
            },

            State::Final => {
                // This pod has reached its destination, it can't move.
                Vec::new()
            },
        }
    }

    fn get_moves_init<'a>(&self, pod: &'a Amphipod) -> Vec<Move<'a>> {
        // Up in the room towards the hallway.
        let mut steps = Vec::new();
        if (1..pod.pos.0).rev().any(|i| {
            steps.push((i, pod.pos.1));

            matches!(self.map[i][pod.pos.1], Cell::Pod(_))
        }) {
            // At least one cell above the pod is occupied, can't move.
            return Vec::new();
        }

        // Left/right into the hallway.
        let mut moves = Vec::new();

        let left = (0..pod.pos.1).rev();
        let right = (pod.pos.1+1)..self.map.cols();
        let mut sides: [Box<dyn Iterator<Item = _>>; 2] = [Box::new(left), Box::new(right)];
        for side in &mut sides {
            let mut steps = steps.clone();
            for i in &mut *side {
                // Stop this side as soon as the way is blocked.
                if self.map[1][i] != Cell::Empty {
                    break;
                }
                steps.push((1, i));

                // Rule 1: Amphipods will never stop on the space immediately outside any
                // room.
                if !self.rooms.contains(&i) {
                    moves.push(Move {
                        pod,
                        steps: steps.clone(),
                    });
                }
            }
        }

        // eprintln!("Init moves for pod {:?}: {:?}", pod, moves);

        moves
    }

    fn get_moves_hallway<'a>(&self, pod: &'a Amphipod) -> Vec<Move<'a>> {
        let room = self.rooms[pod.room_index()];

        let mut steps = Vec::new();

        // Rule 3: Once an amphipod stops moving in the hallway, it will stay in that spot until it
        // can move into a room.
        let mut hallway: Box<dyn Iterator<Item = _>> = if room < pod.pos.1 {
            Box::new(((room+1)..pod.pos.1).rev())
        } else {
            Box::new((pod.pos.1+1)..room)
        };
        if hallway.any(|i| {
            steps.push((1, i));

            self.map[1][i] != Cell::Empty
        }) {
            // Can't move at all if we can't reach our final room.
            return Vec::new();
        }

        // Last steps: get into the room.
        let mut i = 1;
        while self.map[i][room] == Cell::Empty {
            steps.push((i, room));
            i += 1;
        }
        // Rule 2: Amphipods will never move from the hallway into a room unless that room is their
        // destination room and that room contains no amphipods which do not also have that room as
        // their own destination.
        if (i..self.depth()).any(|i| self.map[i][room] != Cell::Pod(pod.kind)) {
            return Vec::new();
        }

        vec![Move { pod, steps, }]
    }

    pub fn solve(&mut self, pods: Pods) -> Option<Path> {
        self.clear();

        let mut explored = 0;
        let mut solution = Path {
            path: Vec::new(),
            moves: 0,
            cost: u32::MAX,
        };
        let mut states = HashMap::new();
        let mut paths = vec![Path::new(pods)];
        while !paths.is_empty() {
            if (explored % 1000) == 0 {
                eprint!("{}[2K\rPending / explored (distinct): {} / {} ({})", 27 as char, paths.len(), explored, states.len());
            }
            explored += 1;
            let path = paths.pop().unwrap();
            let pods = path.path.last().unwrap();
            self.reset(pods);

            // Is this path finished?
            if pods.iter().all(Amphipod::is_settled) {
                println!("{}[2K\rFound new solution at cost {} with {} moves.", 27 as char, path.cost, path.moves);
                solution = path;
                continue;
            }

            // Queue possible next moves.
            for (i,p) in pods.iter().enumerate() {
                for m in self.get_moves(p) {
                    let mut new_path = path.clone();
                    let new_pods = new_path.add_move(m, i);

                    let state = states.entry(new_pods.clone()).or_insert(u32::MAX);
                    // Discard moves that are too costly compared to the current best known solution or
                    // path to this state.
                    if new_path.cost >= solution.cost || *state <= new_path.cost {
                        continue;
                    }
                    *state = new_path.cost;
                    paths.push(new_path);
                }
            }
        }
        eprintln!("{}[2K\rPending / explored (distinct): {} / {} ({})", 27 as char, paths.len(), explored, states.len());

        if solution.moves != 0 {
            Some(solution)
        } else {
            None
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let map: Vec<String> = (0..self.map.rows()).map(|i| {
            self.map.iter_row(i).map(|c|
                match c {
                    Cell::Invalid => ' ',
                    Cell::Wall => '#',
                    Cell::Empty => '.',
                    Cell::Pod(c) => *c,
                }
            ).collect()
        }).collect();
        write!(f, "{}", map.join("\n"))
    }
}

pub fn parse_stdin() -> (Map, Pods) {
    parse_lines(io::stdin().lock().lines().flatten())
}

pub fn parse_lines<I>(lines: I) -> (Map, Pods) 
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    let mut rows = 0;
    let mut cols: Option<usize> = None;
    let mut map = Vec::new();
    let mut pods = Vec::new();
    let mut rooms = BTreeSet::new();

    for line in lines {
        let (mut row, mut row_pods) = parse_line(&line.borrow(), rows, cols);
        for pod in &row_pods {
            rooms.insert(pod.pos.1);
        }
        if cols.is_none() {
            cols = Some(row.len());
        }
        map.append(&mut row);
        pods.append(&mut row_pods);
        rows += 1;
    }

    let map = Map {
        map: Grid::from_vec(map, cols.unwrap()),
        rooms: rooms.iter().copied().collect(),
    };

    // Set the state of all already-in-place pods to final.
    for p in &mut pods {
        if p.pos.0 == map.depth() - 1 && map.rooms[p.room_index()] == p.pos.1 {
            p.state = State::Final;
        }
    }
    assert!(pods.iter().all(|p| p.pos.0 < map.depth()-1 || map.rooms[p.room_index()] != p.pos.1 || p.state == State::Final));

    (map, pods)
}

fn parse_line(line: &str, rows: usize, cols: Option<usize>) -> (Vec<Cell>, Pods) {
    let mut pods = Vec::new();
    let mut row = Vec::new();

    for c in line.chars() {
        let pos = (rows, row.len());
        row.push(match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            'A' | 'B' | 'C' | 'D' => {
                pods.push(Amphipod::new(c, pos));
                Cell::Empty
            },
            _ => Cell::Invalid,
        });
    }

    if let Some(cols) = cols {
        if row.len() < cols {
            row.extend((row.len()..cols).map(|_| Cell::Invalid));
        }
    }

    (row, pods)
}
