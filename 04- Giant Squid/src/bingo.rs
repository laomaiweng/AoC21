const SIZE: usize = 5;

#[derive(Copy, Clone)]
pub struct Number {
    value: u8,
    ticked: bool,
}

impl Number {
    pub fn new(value: u8) -> Number {
        Number { value, ticked: false }
    }
}

pub struct Bingo {
    pub grid: [[Number; SIZE]; SIZE],
    completed: bool,
}

impl Bingo {
    pub fn new() -> Bingo {
        Bingo {
            grid: [[Number::new(0); SIZE]; SIZE],
            completed: false,
        }
    }

    fn check_at(&self, row: usize, col: usize) -> bool {
        let row_ticked = self.grid[row].iter().all(|&n| n.ticked);
        let column_ticked = (0..SIZE).map(|j| &self.grid[j][col]).all(|&n| n.ticked);
        row_ticked || column_ticked
    }

    pub fn tick(&mut self, number: u8) -> bool {
        // don't tick already completed grids
        if self.completed {
            return false;
        }

        for i in 0..SIZE {
            for j in 0..SIZE {
                let n = &mut self.grid[i][j];
                if n.value == number {
                    n.ticked = true;
                    if self.check_at(i, j) {
                        self.completed = true;
                        return true;
                    }
                    // keep going, the number might appear multiple times?
                }
            }
        }
        false
    }

    pub fn score(&self) -> u32 {
        let mut score = 0;
        for i in 0..SIZE {
            for j in 0..SIZE {
                let n = &self.grid[i][j];
                if !n.ticked {
                    score += n.value as u32;
                }
            }
        }
        score
    }
}
