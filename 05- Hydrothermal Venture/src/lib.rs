use std::io::{self, BufRead};
use std::fmt;

use grid;

const SIZE: usize = 1000;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn new(coords: Vec<usize>) -> Point {
        assert_eq!(coords.len(), 2);
        Point {
            x: coords[1],  /* check the example, the coords are formatted as "y,x" */
            y: coords[0],
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.y, self.x)
    }
}

#[derive(Copy, Clone)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    fn new(coords: Vec<Point>) -> Line {
        assert_eq!(coords.len(), 2);
        Line {
            a: coords[0],
            b: coords[1],
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.a, self.b)
    }
}

pub struct Grid {
    pub cells: grid::Grid<usize>
}

impl Grid {
    pub fn new() -> Grid {
        Grid { cells: grid::Grid::new(SIZE, SIZE) }
    }

    pub fn draw(&mut self, line: &Line) {
        // must Box the iterators to circumvent error about incompatible iterator types
        let xrange: Box<dyn Iterator<Item = usize>> = if line.a.x <= line.b.x {
            Box::new(line.a.x..=line.b.x)
        } else {
            Box::new((line.b.x..=line.a.x).rev())
        };
        let yrange: Box<dyn Iterator<Item = usize>> = if line.a.y <= line.b.y {
            Box::new(line.a.y..=line.b.y)
        } else {
            Box::new((line.b.y..=line.a.y).rev())
        };

        if line.a.x == line.b.x {
            // horizontal line
            for y in yrange {
                self.cells[line.a.x][y] += 1;
            }
        } else if line.a.y == line.b.y {
            // vertical line
            for x in xrange {
                self.cells[x][line.a.y] += 1;
            }
        } else {
            // diagonal line
            for (x, y) in xrange.zip(yrange) {
                self.cells[x][y] += 1;
            }
        }
    }

    pub fn count_overlapping(&self) -> usize {
        let mut count: usize = 0;
        for x in 0..SIZE {
            for y in 0..SIZE {
                if self.cells[x][y] > 1 {
                    count += 1;
                }
            }
        }
        count
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows: Vec<String> = Vec::with_capacity(SIZE);
        for x in 0..SIZE {
            rows.push(
                self.cells[x].iter().map(|c| {
                    if *c == 0 {
                        String::from(".")
                    } else {
                        (*c).to_string()
                    }
                }).collect::<Vec<String>>().join("")
            );
        }
        write!(f, "{}", rows.join("\n"))
    }
}

pub fn parse_stdin() -> Vec<Line> {
    let mut vent_lines: Vec<Line> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        vent_lines.push(Line::new(
            line.trim().split(" -> ").map(
                |c| Point::new(
                    c.split(',').map(
                        |n| n.parse::<usize>().unwrap()
                    ).collect()
                )
            ).collect()
        ));
    }
    vent_lines
}
