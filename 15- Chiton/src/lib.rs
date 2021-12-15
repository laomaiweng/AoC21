use std::collections::HashSet;
use std::io::{self, BufRead};

use grid;

type Coords = (usize, usize);

struct Cell {
    risk: u32,
    distance: Option<u32>,
    previous: Option<Coords>,
}

pub struct Map(grid::Grid<Cell>);

impl Map {
    pub fn solve(&mut self) {
        let (n,m) = self.0.size();

        // Dijkstra 1: mark all nodes unvisited, create a set of all unvisited nodes
        let mut unvisited: HashSet<Coords> = HashSet::new();
        for i in 0..n {
            for j in 0..m {
                unvisited.insert((i,j));
            }
        }
        // Dijkstra 2: assign to every node a tentative distance value (we assert it), set it to 0
        //             for our initial node
        assert!(self.0.iter().all(|c| c.distance.is_none()));
        self.0[0][0].distance = Some(0);

        // Dijkstra 2: set the initial node as current
        let mut current = (0,0);

        // Dijkstra 5: the algorithm can stop once the destination node could be selected as the
        //             next "current"
        while current != (n-1,m-1) {
            let (i,j) = current;
            // Dijkstra 3: for the current node, consider all its unvisited neighbors and calculate
            //             their tentative distances through the current node
            let distance = self.0[i][j].distance.unwrap();
            if i > 0 {
                self.compute_neighbor(current, distance, (i-1,j));
            }
            if i < n-1 {
                self.compute_neighbor(current, distance, (i+1,j));
            }
            if j > 0 {
                self.compute_neighbor(current, distance, (i,j-1));
            }
            if j < m-1 {
                self.compute_neighbor(current, distance, (i,j+1));
            }

            // Dijkstra 4: mark the current node as visited and remove it from the unvisited set
            unvisited.remove(&current);

            // Dijkstra 6: select the unvisited node that is marked with the smallest tentative distance
            current = *(unvisited.iter().filter_map(|c| {
                if let Some(distance) = self.0[c.0][c.1].distance {
                    Some((c, distance))
                } else {
                    None
                }
            }).min_by_key(|(_, d)| *d).expect("No minimum-distance unvisited node!")).0;
        }
    }

    fn compute_neighbor(&mut self, from: Coords, distance: u32, to: Coords) {
        let cell = &mut self.0[to.0][to.1];
        let distance = distance + cell.risk;
        if distance < cell.distance.unwrap_or(u32::MAX) {
            cell.distance = Some(distance);
            cell.previous = Some(from);
            if to.0 == self.0.rows() - 1 && to.1 == self.0.cols() - 1 {
                println!("Found new minimum path to destination with risk {}.", distance);
            }
        }
    }
}

pub fn parse_stdin() -> Map {
    let mut map = grid::grid![];

    for line in io::stdin().lock().lines() {
        map.push_row(line.unwrap().bytes().map(|c| Cell {
            risk: (c - 0x30) as u32, /* c-'0' */
            distance: None,
            previous: None,
        }).collect());
    }

    Map(map)
}
