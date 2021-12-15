use std::cmp::Reverse;
use std::io::{self, BufRead};

use grid;
use itertools;
#[cfg(feature = "bitmap")]
use rustbitmap::{BitMap, Rgba};

type Coords = (usize, usize);

#[derive(Default, Clone, Copy)]
struct Cell {
    risk: u32,
    distance: Option<u32>,
    previous: Option<Coords>,
}

pub struct Map(grid::Grid<Cell>);

impl Map {

    pub fn tile(self, rows: usize, cols: usize) -> Self {
        let mut grid = grid::grid![];
        for i in 0..rows {
            for ii in 0..self.0.rows() {
                let row = (0..cols).map(|j| {
                    self.0.iter_row(ii).map(move |c| {
                        Cell {
                            risk: (c.risk - 1 + (i+j) as u32) % 9 + 1,
                            distance: None,
                            previous: None,
                        }
                    })
                }).flatten().collect();
                grid.push_row(row);
            }
        }
        Map(grid)
    }

    pub fn rows(&self) -> usize { self.0.rows() }
    pub fn cols(&self) -> usize { self.0.cols() }

    pub fn solve(&mut self) {
        let (n,m) = self.0.size();

        // Dijkstra 1: mark all nodes unvisited, create a set of all unvisited nodes
        let mut unvisited = Vec::with_capacity(self.0.rows() * self.0.cols());
        unvisited.extend(itertools::iproduct!((0..n).rev(), (0..m).rev()));

        // Dijkstra 2: assign to every node a tentative distance value (we assert it)
        //             set it to 0 for our initial node
        //             set the initial node as current
        //             remove the initial node from the unvisited set (it is the last one by construction)
        assert!(self.0.iter().all(|c| c.distance.is_none()));
        self.0[0][0].distance = Some(0);
        let mut current = (0,0);
        unvisited.pop();

        // Dijkstra 5: the algorithm can stop once the destination node could be selected as the
        //             next "current"
        let mut count: usize = 0;
        while current != (n-1,m-1) {
            let (i,j) = current;
            let distance = self.0[i][j].distance.unwrap();
            if count % 100 == 0 {
                #[cfg(feature = "verbose")]
                println!("Reached node ({},{}).", i, j);

                #[cfg(feature = "bitmap")]
                self.save_as_bitmap(&format!("state/{}.bmp", count/100), distance);
            }
            count += 1;

            // Dijkstra 3: for the current node, consider all its unvisited neighbors and calculate
            //             their tentative distances through the current node
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

            // Dijkstra 6: select the unvisited node that is marked with the smallest tentative distance
            unvisited.sort_unstable_by_key(|c| Reverse(self.0[c.0][c.1].distance.unwrap_or(u32::MAX)));

            // Dijkstra 4: mark the current node as visited and remove it from the unvisited set
            current = unvisited.pop().unwrap();
        }

        #[cfg(feature = "bitmap")]
        self.save_as_bitmap("state/final.bmp", self.0[n-1][m-1].distance.unwrap());
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

    #[cfg(feature = "bitmap")]
    fn save_as_bitmap(&self, name: &str, max_distance: u32) {
        let pixels = self.0.iter().map(|c| {
            if let Some(distance) = c.distance {
                if distance == max_distance {
                    Rgba::white()
                } else {
                    Rgba::rgb(((1. - (distance as f32 / max_distance as f32)) * (u8::MAX as f32)) as u8, 0, 0)
                }
            } else {
                Rgba::black()
            }
        }).collect();
        let bitmap = BitMap::create(self.0.rows() as u32, self.0.cols() as u32, pixels).unwrap();
        bitmap.save_as(name).expect("Failed to save bitmap!");
    }
}

pub fn parse_stdin() -> Map {
    let mut grid = grid::grid![];

    for line in io::stdin().lock().lines() {
        grid.push_row(line.unwrap().bytes().map(|c| Cell {
            risk: (c - 0x30) as u32, /* c-'0' */
            distance: None,
            previous: None,
        }).collect());
    }

    Map(grid)
}
