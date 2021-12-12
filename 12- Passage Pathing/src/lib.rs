use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

pub struct Cave {
    pub name: String,
    pub edges: HashSet<String>,
    pub small: bool,
}

impl Cave {
    fn new(name: &str) -> Cave {
        Cave {
            name: name.to_owned(),
            edges: HashSet::new(),
            small: name.chars().all(char::is_lowercase),
        }
    }
}

#[derive(Clone)]
pub struct Path {
    pub caves: Vec<String>,
    pub small_caves: HashSet<String>,
    pub small_cave2: Option<String>,
}

impl Path {
    pub fn new(start: &Cave) -> Path {
        Path {
            caves: vec![start.name.clone()],
            small_caves: if start.small { HashSet::from([start.name.clone()]) }
                else { HashSet::new() },
            small_cave2: None,
        }
    }
}

pub type Map = HashMap<String, Cave>;

pub fn parse_stdin() -> Map {
    let mut caves = Map::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let names: Vec<&str> = line.split('-').collect();

        let cave0 = caves.entry(names[0].to_owned()).or_insert_with(|| Cave::new(names[0]));
        cave0.edges.insert(names[1].to_owned());

        let cave1 = caves.entry(names[1].to_owned()).or_insert_with(|| Cave::new(names[1]));
        cave1.edges.insert(names[0].to_owned());
    }

    caves
}
