use passage_pathing::{Map, Path, parse_stdin};

fn advance(map: &Map, path: Path) -> Vec<Path> {
    let mut new_paths = Vec::new();
    let last_cave = path.caves.last().unwrap();
    // generate new paths through all the node edges (hopefully there aren't any loops)
    for next_cave in &map.get(last_cave).unwrap().edges {
        let next_cave = map.get(next_cave).unwrap();
        // new path is only valid if the next cave isn't an already-visited small cave
        if !next_cave.small || !path.small_caves.contains(&next_cave.name) {
            let mut new_path = path.clone();
            new_path.caves.push(next_cave.name.clone());
            if next_cave.small {
                new_path.small_caves.insert(next_cave.name.clone());
            }
            new_paths.push(new_path);
        }
    }
    new_paths
}

fn main() {
    let map = parse_stdin();
    let mut paths = vec![Path::new(map.get("start").unwrap())];
    let mut completed_paths: Vec<Path> = Vec::new();

    while !paths.is_empty() {
        let path = paths.pop().unwrap();
        let mut new_paths = advance(&map, path);
        while !new_paths.is_empty() {
            let new_path = new_paths.pop().unwrap();
            if new_path.caves.last().unwrap() == "end" {
                completed_paths.push(new_path);
            } else {
                paths.push(new_path);
            }
        }
    }

    println!("total paths: {}", completed_paths.len());
}
