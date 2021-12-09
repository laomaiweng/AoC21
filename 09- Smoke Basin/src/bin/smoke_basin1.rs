use smoke_basin::{find_lowest_neighbor, parse_stdin};

fn main() {
    let map = parse_stdin();

    let mut risk: u32 = 0;

    let (n, m) = map.size();
    for i in 0..n {
        for j in 0..m {
            let v = map[i][j];
            if find_lowest_neighbor(&map, i, j) == None {
                println!("basin bottom @ ({}, {})", i, j);
                risk += 1 + v as u32;
            }
        }
    }
    println!("risk level: {}", risk);
}
