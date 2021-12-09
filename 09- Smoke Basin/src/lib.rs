use std::io::{self, BufRead};

use grid;

const SIZE: usize = 100;

type Map = grid::Grid<u8>;

pub fn find_lowest_neighbor(map: &Map, i: usize, j: usize) -> Option<(usize, usize)> {
    let (n, m) = map.size();

    let mut neighbors: Vec<(usize, usize, u8)> = Vec::new();
    if i > 0 {
        neighbors.push((i-1, j, map[i-1][j]));
    }
    if j > 0 {
        neighbors.push((i, j-1, map[i][j-1]));
    }
    if i < n-1 {
        neighbors.push((i+1, j, map[i+1][j]));
    }
    if j < m-1 {
        neighbors.push((i, j+1, map[i][j+1]));
    }

    let mut lowest = None;
    let mut level = map[i][j];
    for neighbor in &neighbors {
        if neighbor.2 <= level {
            lowest = Some((neighbor.0, neighbor.1));
            level = neighbor.2;
        }
    }

    lowest
}

pub fn parse_stdin() -> Map {
    let mut cells: Vec<u8> = Vec::with_capacity(SIZE*SIZE);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        cells.extend(line.unwrap().into_bytes().iter().map(|n| n-0x30).collect::<Vec<u8>>());
    }

    Map::from_vec(cells, SIZE)
}
