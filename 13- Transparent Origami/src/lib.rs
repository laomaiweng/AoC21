use std::cmp;
use std::default;
use std::fmt;
use std::io::{self, BufRead};

use grid;
use regex;

pub enum Cell {
    Blank,
    Dot,
}

impl default::Default for Cell {
    fn default() -> Self {
        Cell::Blank
    }
}

pub struct Fold {
    pub axis: char,
    pub coord: usize,
}

pub struct Grid {
    cells: grid::Grid<Cell>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        Grid {
            cells: grid::Grid::new(rows, cols),
            rows,
            cols,
        }
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        for y in 0..self.rows {
            for x in 0..self.cols {
                if matches!(self.cells[y][x], Cell::Dot) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn fold(&mut self, fold: &Fold) {
        match fold.axis {
            'x' => {
                for x in 1..(self.cols - fold.coord) {
                    for y in 0..self.rows {
                        if matches!(self.cells[y][fold.coord + x], Cell::Dot) {
                            self.cells[y][fold.coord - x] = Cell::Dot;
                        }
                    }
                }
                self.cols = fold.coord;
            },
            'y' => {
                for y in 1..(self.rows - fold.coord) {
                    for x in 0..self.cols {
                        if matches!(self.cells[fold.coord + y][x], Cell::Dot) {
                            self.cells[fold.coord - y][x] = Cell::Dot;
                        }
                    }
                }
                self.rows = fold.coord;
            },
            _ => { eprintln!("invalid fold axis: {}", fold.axis); },
        };
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.cols {
                write!(f, "{}", match self.cells[y][x] {
                    Cell::Dot => "#",
                    Cell::Blank => ".",
                })?;
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}

enum ParseMode {
    Dots,
    Folds,
}

pub fn parse_stdin() -> (Grid, Vec<Fold>) {
    let mut grid: Option<Grid> = None;
    let mut folds: Vec<Fold> = Vec::new();

    let fold_re = regex::Regex::new(r"fold along (?P<axis>[xy])=(?P<coord>[0-9]+)").unwrap();

    let mut rows = 1;
    let mut cols = 1;
    let mut dots: Vec<(usize,usize)> = Vec::new();
    let mut mode: ParseMode = ParseMode::Dots;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            // done parsing dots, create the grid
            println!("grid size: ({}, {})", rows, cols);
            grid = Some(Grid::new(rows, cols));
            for dot in &dots {
                grid.as_mut().map(|g| g.cells[dot.0][dot.1] = Cell::Dot);
            }
            dots.clear();
            mode = ParseMode::Folds;
            continue;
        }
        match mode {
            ParseMode::Dots => {
                let coords: Vec<usize> = line.split(',').map(|n| n.parse::<usize>().unwrap()).collect();
                rows = cmp::max(rows, coords[1] + 1);
                cols = cmp::max(cols, coords[0] + 1);
                dots.push((coords[1], coords[0]));
            },
            ParseMode::Folds => {
                let captures = fold_re.captures(&line).unwrap();
                folds.push(Fold {
                    axis: captures[1].chars().next().unwrap(),
                    coord: captures[2].parse::<usize>().unwrap(),
                });
            },
        }
    }

    (grid.unwrap(), folds)
}
