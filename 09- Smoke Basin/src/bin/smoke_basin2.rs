use std::collections::HashMap;

use smoke_basin::{find_lowest_neighbor, parse_stdin};

fn main() {
    let map = parse_stdin();

    let mut basins: HashMap<(usize, usize), u32> = HashMap::new();

    let (n, m) = map.size();
    for i in 0..n {
        for j in 0..m {
            eprint!("({},{}):{}", i, j, map[i][j]);
            let mut bottom = if map[i][j] != 9 {
                Some((i, j))
            } else { None };
            while bottom != None {
                let (x, y) = bottom.unwrap();
                bottom = find_lowest_neighbor(&map, x, y);
                match bottom {
                    None => { basins.insert((x, y), basins.get(&(x, y)).unwrap_or(&0) + 1); },
                    Some((a,b)) => { eprint!(" -> ({},{}):{}", a, b, map[a][b]); },
                }
            }
            eprintln!("");
        }
    }
    let mut basins: Vec<((usize, usize), u32)> = basins.into_iter().collect();
    basins.sort_unstable_by_key(|(_,s)| *s);
    for ((i,j), size) in basins.iter() {
        println!("basin @ ({}, {}): size {}", i, j, size);
    }
}
