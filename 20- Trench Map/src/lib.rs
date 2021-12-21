use std::fmt;
use std::io::{self, BufRead};

use grid::Grid;

pub type Algorithm = Vec<bool>;
pub struct Image(Grid<bool>);

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (n,m) = self.0.size();
        for i in 2..n-2 {
            let row = self.0[i][2..m-2].iter().map(|b| if *b { '#' } else { '.' }).collect::<String>();
            write!(f, "{}\n", row)?;
        }
        fmt::Result::Ok(())
    }
}

impl Image {
    pub fn count(&self) -> usize {
        self.0.iter().filter(|b| **b).count()
    }

    pub fn rows(&self) -> usize { self.0.rows()-4 }
    pub fn cols(&self) -> usize { self.0.cols()-4 }

    pub fn enhance(&self, algo: &Algorithm) -> Image {
        let (n,m) = self.0.size();
        let default = if self.0[0][0] { *algo.last().unwrap() } else { algo[0] };
        let mut image = Image(Grid::init(n + 2, m + 2, default));
        for i in 1..n-1 {
            for j in 1..m-1 {
                // Collect surrounding bits.
                let mut bits: Vec<bool> = self.0[i-1][j-1..=j+1].iter().copied().collect();
                bits.extend(self.0[i][j-1..=j+1].iter().copied().collect::<Vec<_>>());
                bits.extend(self.0[i+1][j-1..=j+1].iter().copied().collect::<Vec<_>>());
                // Turn them into an index.
                let mut index = 0usize;
                for (i, b) in bits.iter().enumerate() {
                    let b = if *b { 1 } else { 0 };
                    index += b << (bits.len() - 1 - i);
                }
                // Set the corresponding bit in the new image.
                image.0[i+1][j+1] = algo[index];
            }
        }
        image
    }
}

pub fn parse_stdin() -> (Algorithm, Image) {
    let mut algo = None;
    let mut image = None;

    enum ParseMode {
        Algorithm,
        Image,
    }
    let mut mode = ParseMode::Algorithm;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            mode = ParseMode::Image;
            continue;
        }
        match mode {
            ParseMode::Algorithm => {
                algo = Some(line.chars().map(parse_char).collect());
            },
            ParseMode::Image => {
                // Extend the input with 2 dark pixels on each size.
                let mut row = Vec::with_capacity(line.len() + 4);
                row.push(false);
                row.push(false);
                row.extend(line.chars().map(parse_char).collect::<Vec<_>>());
                row.push(false);
                row.push(false);
                if image.is_none() {
                    // Create the image: add 2 dark rows at the top.
                    let mut new_image = Image(Grid::from_vec(dark_row(row.len()), row.len()));
                    new_image.0.push_row(dark_row(row.len()));
                    image = Some(new_image);
                }
                if let Some(ref mut image) = image {
                    // Add the new row.
                    image.0.push_row(row);
                }
            },
        }
    }

    // Finalize the image: add 2 dark rows at the bottom.
    let mut image = image.unwrap();
    image.0.push_row(dark_row(image.0.cols()));
    image.0.push_row(dark_row(image.0.cols()));

    (algo.unwrap(), image)
}

fn dark_row(n: usize) -> Vec<bool> {
    (0..n).map(|_| false).collect()
}

fn parse_char(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!("Invalid character: {}", c),
    }
}
