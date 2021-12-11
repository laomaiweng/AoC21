use std::io::{self, BufRead};

use grid;

const SIZE: usize = 10;

pub struct Octopuses {
    grid: grid::Grid<u8>,
}

impl Octopuses {
    pub fn step(&mut self) -> u32 {
        let mut flashes = 0u32;
        let (n, m) = self.grid.size();

        // 1: increase
        for i in 0..n {
            for j in 0..m {
                self.grid[i][j] += 1;
            }
        }

        // 2: flash
        for i in 0..n {
            for j in 0..m {
                flashes += self.flash(i, j);
            }
        }

        flashes
    }

    #[cfg(feature = "norecurse")]
    fn flash(&mut self, row: usize, col: usize) -> u32 {
        let (n, m) = self.grid.size();

        let mut flashes = 0u32;
        let mut flashers: Vec<(usize, usize)> = vec![(row,col)];
        while !flashers.is_empty() {
            let (row,col) = flashers.pop().unwrap();
            if self.grid[row][col] <= 9 {
                continue;
            }
            self.grid[row][col] = 0;
            flashes += 1;

            let rmin = if row == 0 { 0 } else { row-1 };
            let rmax = if row < n-1 { row+1 } else { row };
            let cmin = if col == 0 { 0 } else { col-1 };
            let cmax = if col < m-1 { col+1 } else { col };
            for i in rmin..=rmax {
                for j in cmin..=cmax {
                    // only flash once
                    if self.grid[i][j] != 0 {
                        self.grid[i][j] += 1;
                        if self.grid[i][j] > 9 {
                            flashers.push((i,j));
                        }
                    }
                }
            }
        }

        flashes
    }

    #[cfg(not(feature = "norecurse"))]
    fn flash(&mut self, row: usize, col: usize) -> u32 {
        if self.grid[row][col] <= 9 {
            return 0;
        }

        // 3: set energy to 0
        let mut flashes = 1u32;
        self.grid[row][col] = 0;

        let (n, m) = self.grid.size();
        let rmin = if row == 0 { 0 } else { row-1 };
        let rmax = if row < n-1 { row+1 } else { row };
        let cmin = if col == 0 { 0 } else { col-1 };
        let cmax = if col < m-1 { col+1 } else { col };
        for i in rmin..=rmax {
            for j in cmin..=cmax {
                // only flash once
                if self.grid[i][j] != 0 {
                    self.grid[i][j] += 1;
                    flashes += self.flash(i, j);
                }
            }
        }

        flashes
    }

    pub fn energy(&self) -> u32 {
        self.grid.iter().map(|o| *o as u32).sum()
    }
}

pub fn parse_stdin() -> Octopuses {
    let mut cells: Vec<u8> = Vec::with_capacity(SIZE*SIZE);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        cells.extend(line.unwrap().into_bytes().iter().map(|n| n-0x30).collect::<Vec<u8>>());
    }

    Octopuses {
        grid: grid::Grid::from_vec(cells, SIZE)
    }
}
