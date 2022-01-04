use std::borrow::Borrow;
use std::io::{self, BufRead};

use grid;
#[cfg(feature = "curses")]
use pancurses::{self, Window};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    East,
    South,
}

pub struct Map (grid::Grid<Cell>);

impl Map {
    pub fn rows(&self) -> usize { self.0.rows() }
    pub fn cols(&self) -> usize { self.0.cols() }

    /// Move cucumbers in-place.
    // There are two traps:
    // * We must take care not to move a cucumber into a slot if that slot was occupied at the
    //   start of the pass. To this end we iterate over cucumbers in the direction of their
    //   movement (which turns out to be the natural direction for iterating over a 2D grid), and
    //   we special-case the first column/row to gracefully handle cucumbers wrapping around.
    // * We must take care not to cascade-move a cucumber that we just moved, as we move forward
    //   into the map. To this end we remember to skip the next cell if we moved a cucumber.
    // Moving into a new map would perhaps feel a bit less hackish, but it would require allocating
    // 2 additional maps for each call to step(), one for each pass.
    pub fn step(&mut self) -> usize {
        let mut moved = 0;
        let (n,m) = self.0.size();

        // 1st pass: eastbound cucumbers.
        for i in 0..n {
            // Remember whether column 0 was occupied to handle wrap-around correctly.
            let col0_occupied = self.0[i][0] != Cell::Empty;
            let mut skip = false;
            for j in 0..m {
                if skip {
                    skip = false;
                    continue;
                }
                match self.0[i][j] {
                    Cell::Empty | Cell::South => (),
                    Cell::East => {
                        let nj = if j < m-1 { j+1 } else { 0 };
                        if (nj == 0 && !col0_occupied) || (nj > 0 && self.0[i][nj] == Cell::Empty) {
                            // Move!
                            self.0[i][nj] = self.0[i][j];
                            self.0[i][j] = Cell::Empty;
                            moved += 1;
                            skip = true;
                        }
                    },
                }
            }
        }

        // 2nd pass: southbound cucumbers.
        // Remember whether row 0 was occupied to handle wrap-around correctly.
        let row0_occupied: Vec<_> = self.0.iter_row(0).map(|&c| c != Cell::Empty).collect();
        let mut skip = vec![false; m];
        for i in 0..n {
            for j in 0..m {
                if skip[j] {
                    skip[j] = false;
                    continue;
                }
                match self.0[i][j] {
                    Cell::Empty | Cell::East => (),
                    Cell::South => {
                        let ni = if i < n-1 { i+1 } else { 0 };
                        if (ni == 0 && !row0_occupied[j]) || (ni > 0 && self.0[ni][j] == Cell::Empty) {
                            // Move!
                            self.0[ni][j] = self.0[i][j];
                            self.0[i][j] = Cell::Empty;
                            moved += 1;
                            skip[j] = true;
                        }
                    },
                }
            }
        }

        moved
    }

    pub fn lines(&self) -> MapLines<'_> {
        MapLines {
            map: &self,
            row: 0,
        }
    }

    #[cfg(feature = "curses")]
    pub fn render(&self, window: &Window) {
        window.clear();
        for (i, line) in self.lines().enumerate() {
            window.mvaddstr(i as i32, 0, &line);
        }
        window.refresh();
    }
}

pub struct MapLines<'m> {
    map: &'m Map,
    row: usize,
}

impl<'m> Iterator for MapLines<'m> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.row += 1;
        if self.row <= self.map.0.rows() {
            Some(self.map.0.iter_row(self.row - 1).map(|&c| match c {
                Cell::Empty => '.',
                Cell::East => '>',
                Cell::South => 'v',
            }).collect())
        } else {
            None
        }
    }
}

#[cfg(feature = "curses")]
pub fn init_curses(map: &Map) -> Window {
    pancurses::resize_term(map.rows() as i32, map.cols() as i32);
    let window = pancurses::initscr();
    map.render(&window);
    pancurses::napms(3000);
    window
}

#[cfg(feature = "curses")]
pub fn fini_curses(window: Window) {
    // Wait for a keystroke before exiting.
    pancurses::noecho();
    window.getch();
    pancurses::endwin();
}

pub fn parse_stdin() -> Map {
    parse_lines(io::stdin().lock().lines().flatten())
}

pub fn parse_lines<I>(lines: I) -> Map
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    let mut cols: usize = 0;
    let rows: Vec<_> = lines.into_iter().filter_map(|l| {
        let line = l.borrow().trim();
        if line.is_empty() {
            return None;
        }
        let row: Vec<_> = line.chars().map(|c| match c {
            '.' => Cell::Empty,
            '>' => Cell::East,
            'v' => Cell::South,
            _ => panic!("unexpected character: {}", c),
        }).collect();
        cols = row.len();
        Some(row)
    }).flatten().collect();

    Map(grid::Grid::from_vec(rows, cols))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(map: &Map, expected: &str) {
        let expected: Vec<_> = expected.lines().filter_map(|l|
            if l.trim().is_empty() {
                None
            } else {
                Some(l.trim())
            }
        ).collect();
        let actual: Vec<_> = map.lines().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn single_row() {
        let mut map = parse_lines("...>>>>>...".lines());
        assert_eq!(map.step(), 1);
        check(&map, "...>>>>.>..");
        assert_eq!(map.step(), 2);
        check(&map, "...>>>.>.>.");
    }

    #[test]
    fn move_order() {
        let mut map = parse_lines(r#"
            ..........
            .>v....v..
            .......>..
            ..........
        "#.lines());
        assert_eq!(map.step(), 3);
        check(&map, r#"
            ..........
            .>........
            ..v....v>.
            ..........
        "#);
    }

    #[test]
    fn wrap_around() {
        // Initial state:
        let mut map = parse_lines(r#"
            ...>...
            .......
            ......>
            v.....>
            ......>
            .......
            ..vvv..
        "#.lines());

        // After 1 step:
        map.step();
        check(&map, r#"
            ..vv>..
            .......
            >......
            v.....>
            >......
            .......
            ....v..
        "#);

        // After 2 steps:
        map.step();
        check(&map, r#"
            ....v>.
            ..vv...
            .>.....
            ......>
            v>.....
            .......
            .......
        "#);

        // After 3 steps:
        map.step();
        check(&map, r#"
            ......>
            ..v.v..
            ..>v...
            >......
            ..>....
            v......
            .......
        "#);

        // After 4 steps:
        map.step();
        check(&map, r#"
            >......
            ..v....
            ..>.v..
            .>.v...
            ...>...
            .......
            v......
        "#);
    }

    #[test]
    fn fixed_point() {
        // Initial state:
        let mut map = parse_lines(r#"
            v...>>.vv>
            .vv>>.vv..
            >>.>v>...v
            >>v>>.>.v.
            v>v.vv.v..
            >.>>..v...
            .vv..>.>v.
            v.v..>>v.v
            ....v..v.>
        "#.lines());

        // After 1 step:
        map.step();
        check(&map, r#"
            ....>.>v.>
            v.v>.>v.v.
            >v>>..>v..
            >>v>v>.>.v
            .>v.v...v.
            v>>.>vvv..
            ..v...>>..
            vv...>>vv.
            >.v.v..v.v
        "#);

        // After 2 steps:
        map.step();
        check(&map, r#"
            >.v.v>>..v
            v.v.>>vv..
            >v>.>.>.v.
            >>v>v.>v>.
            .>..v....v
            .>v>>.v.v.
            v....v>v>.
            .vv..>>v..
            v>.....vv.
        "#);

        // After 3 steps:
        map.step();
        check(&map, r#"
            v>v.v>.>v.
            v...>>.v.v
            >vv>.>v>..
            >>v>v.>.v>
            ..>....v..
            .>.>v>v..v
            ..v..v>vv>
            v.v..>>v..
            .v>....v..
        "#);

        // After 4 steps:
        map.step();
        check(&map, r#"
            v>..v.>>..
            v.v.>.>.v.
            >vv.>>.v>v
            >>.>..v>.>
            ..v>v...v.
            ..>>.>vv..
            >.v.vv>v.v
            .....>>vv.
            vvv>...v..
        "#);

        // After 5 steps:
        map.step();
        check(&map, r#"
            vv>...>v>.
            v.v.v>.>v.
            >.v.>.>.>v
            >v>.>..v>>
            ..v>v.v...
            ..>.>>vvv.
            .>...v>v..
            ..v.v>>v.v
            v.v.>...v.
        "#);

        // After 10 steps:
        for _ in 5..10 {
            map.step();
        }
        check(&map, r#"
            ..>..>>vv.
            v.....>>.v
            ..v.v>>>v>
            v>.>v.>>>.
            ..v>v.vv.v
            .v.>>>.v..
            v.v..>v>..
            ..v...>v.>
            .vv..v>vv.
        "#);

        // After 20 steps:
        for _ in 10..20 {
            map.step();
        }
        check(&map, r#"
            v>.....>>.
            >vv>.....v
            .>v>v.vv>>
            v>>>v.>v.>
            ....vv>v..
            .v.>>>vvv.
            ..v..>>vv.
            v.v...>>.v
            ..v.....v>
        "#);

        // After 30 steps:
        for _ in 20..30 {
            map.step();
        }
        check(&map, r#"
            .vv.v..>>>
            v>...v...>
            >.v>.>vv.>
            >v>.>.>v.>
            .>..v.vv..
            ..v>..>>v.
            ....v>..>v
            v.v...>vv>
            v.v...>vvv
        "#);

        // After 40 steps:
        for _ in 30..40 {
            map.step();
        }
        check(&map, r#"
            >>v>v..v..
            ..>>v..vv.
            ..>>>v.>.v
            ..>>>>vvv>
            v.....>...
            v.v...>v>>
            >vv.....v>
            .>v...v.>v
            vvv.v..v.>
        "#);

        // After 50 steps:
        for _ in 40..50 {
            map.step();
        }
        check(&map, r#"
            ..>>v>vv.v
            ..v.>>vv..
            v.>>v>>v..
            ..>>>>>vv.
            vvv....>vv
            ..v....>>>
            v>.......>
            .vv>....v>
            .>v.vv.v..
        "#);

        // After 55 steps:
        for _ in 50..55 {
            map.step();
        }
        check(&map, r#"
            ..>>v>vv..
            ..v.>>vv..
            ..>>v>>vv.
            ..>>>>>vv.
            v......>vv
            v>v....>>v
            vvv...>..>
            >vv.....>.
            .>v.vv.v..
        "#);

        // After 56 steps:
        map.step();
        check(&map, r#"
            ..>>v>vv..
            ..v.>>vv..
            ..>>v>>vv.
            ..>>>>>vv.
            v......>vv
            v>v....>>v
            vvv....>.>
            >vv......>
            .>v.vv.v..
        "#);

        // After 57 steps:
        map.step();
        check(&map, r#"
            ..>>v>vv..
            ..v.>>vv..
            ..>>v>>vv.
            ..>>>>>vv.
            v......>vv
            v>v....>>v
            vvv.....>>
            >vv......>
            .>v.vv.v..
        "#);

        // After 58 steps:
        assert_eq!(map.step(), 0);
        check(&map, r#"
            ..>>v>vv..
            ..v.>>vv..
            ..>>v>>vv.
            ..>>>>>vv.
            v......>vv
            v>v....>>v
            vvv.....>>
            >vv......>
            .>v.vv.v..
        "#);
    }
}
