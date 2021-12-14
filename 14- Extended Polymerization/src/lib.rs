use std::collections::HashMap;
use std::io::{self, BufRead};

use itertools::Itertools;

pub type Rules = HashMap<(char, char), char>;
pub type Distrib = HashMap<(char, char), usize>;

pub fn grow_polymer(template: String, rules: &Rules) -> String {
    let mut polymer = String::new();
    let iter_left = template.chars();
    let mut iter_right = template.chars();
    polymer.push(iter_right.next().unwrap());
    for (l,r) in iter_left.zip(iter_right) {
        if let Some(insert) = rules.get(&(l,r)) {
            polymer.push(*insert);
        }
        polymer.push(r);
    }
    polymer
}

pub fn compute_distrib(template: &str) -> Distrib {
    let iter_left = template.chars();
    let mut iter_right = template.chars();
    iter_right.next();
    iter_left.zip(iter_right).counts()
}

pub fn grow_polymer_distrib(template: Distrib, rules: &Rules) -> Distrib {
    let mut distrib = Distrib::new();
    for ((l, r), count) in template.iter() {
        if let Some(m) = rules.get(&(*l, *r)) {
            *distrib.entry((*l,*m)).or_insert(0) += count;
            *distrib.entry((*m,*r)).or_insert(0) += count;
        } else {
            *distrib.entry((*l,*r)).or_insert(0) += count;
        }
    }
    distrib
}

enum ParseMode {
    Template,
    Rules,
}

pub fn parse_stdin() -> (String, Rules) {
    let mut template = String::new();
    let mut rules = Rules::new();

    let mut mode: ParseMode = ParseMode::Template;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            mode = ParseMode::Rules;
            continue;
        }
        match mode {
            ParseMode::Template => {
                template = line;
            },
            ParseMode::Rules => {
                let rule: Vec<&str> = line.split(" -> ").collect();
                rules.insert(rule[0].chars().collect_tuple().unwrap(), rule[1].chars().next().unwrap());
            },
        }
    }

    (template, rules)
}
