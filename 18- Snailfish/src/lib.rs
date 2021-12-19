use std::cell::RefCell;
use std::io::{self, BufRead};

use generational_arena::Index;

pub enum Node {
    Leaf(u32),
    Node(Index, Index),
}

type Arena = generational_arena::Arena<RefCell<Node>>;

impl Node {
    fn from_parse_node(parse_node: ParseNode) -> Self {
        match parse_node {
            ParseNode::Leaf(num) => Node::Leaf(num),
            ParseNode::Node(Some(left), Some(right)) => Node::Node(left, right),
            _ => panic!("Invalid parse node state!"),
        }
    }

    pub fn is_leaf(&self) -> bool { matches!(self, Node::Leaf(_)) }
    pub fn is_node(&self) -> bool { matches!(self, Node::Node(_, _)) }

    pub fn to_string(&self, arena: &Arena) -> String {
        match self {
            Node::Leaf(num) => format!("{}", num),
            Node::Node(left, right) => {
                let left = arena.get(*left).unwrap().borrow();
                let right = arena.get(*right).unwrap().borrow();
                format!("[{},{}]", left.to_string(arena), right.to_string(arena))
            },
        }
    }
}

pub fn to_string(root: Index, arena: &Arena) -> String {
    let node = arena.get(root).unwrap().borrow();
    node.to_string(&arena)
}

pub fn add(left: Index, right: Index, arena: &mut Arena) -> Index {
    arena.insert(RefCell::new(Node::Node(left, right)))
}

pub fn explode(root: Index, arena: &mut Arena) -> bool {
    let mut exploded = None;
    let mut left_number_index = None;
    let mut right_number_value = None;

    let mut path = vec![(root, 0)];
    while !path.is_empty() {
        let (current, depth) = path.pop().unwrap();
        let mut node = arena.get(current).unwrap().borrow_mut();
        match *node {
            Node::Leaf(ref mut num) => {
                // If we have a value to propagate right, do it.
                if let Some(value) = right_number_value {
                    *num += value;
                    break; // We're done for this traversal.
                } else { // Otherwise remember the current index in case we later need to propagate left.
                    left_number_index = Some(current);
                }
            },
            Node::Node(ileft, iright) => {
                // Check for explosions.
                let left = arena.get(ileft).unwrap().borrow();
                let right = arena.get(iright).unwrap().borrow();
                if exploded.is_none() && depth >= 4 && left.is_leaf() && right.is_leaf() {
                    let lvalue = if let Node::Leaf(num) = *left { num } else { panic!("¿¿¿") };
                    let rvalue = if let Node::Leaf(num) = *right { num } else { panic!("???") };
                    // Replace the current node.
                    exploded = Some((ileft, iright));
                    *node = Node::Leaf(0);
                    // Propagate left value.
                    if let Some(previous) = left_number_index {
                        let mut previous = arena.get(previous).unwrap().borrow_mut();
                        match *previous {
                            Node::Leaf(ref mut num) => *num += lvalue,
                            _ => panic!("!!!"),
                        }
                    }
                    // Propagate right value.
                    right_number_value = Some(rvalue);
                } else { // Otherwise queue the child nodes (left last since we're in a LIFO queue).
                    path.push((iright, depth+1));
                    path.push((ileft, depth+1));
                }
            },
        }
    }

    if let Some((left, right)) = exploded {
        // Drop the former children of the exploded node.
        arena.remove(left);
        arena.remove(right);
        true
    } else {
        false
    }
}

pub fn split(root: Index, arena: &mut Arena) -> bool {
    let mut split: Option<(Index, u32)> = None;

    let mut path = vec![root];
    while !path.is_empty() {
        let current = path.pop().unwrap();
        let mut node = arena.get(current).unwrap().borrow_mut();
        match *node {
            Node::Leaf(ref mut num) => {
                // Check for splits.
                if *num >= 10 {
                    split = Some((current, *num));
                    break; // Peform the split out of the loop, where we won't be holding a borrow on the arena.
                }
            },
            Node::Node(ileft, iright) => {
                // Queue the child nodes (left last since we're in a LIFO queue).
                path.push(iright);
                path.push(ileft);
            },
        }
    }

    // Perform the split now that we no longer hold any borrow on the arena.
    if let Some((index, num)) = split {
        let left = Node::Leaf(num / 2);
        let ileft = arena.insert(RefCell::new(left));
        let right = Node::Leaf((num + 1) / 2);
        let iright = arena.insert(RefCell::new(right));
        let mut node = arena.get(index).unwrap().borrow_mut();
        *node = Node::Node(ileft, iright);
        true
    } else {
        false
    }
}

pub fn reduce(root: Index, arena: &mut Arena) {
    while explode(root, arena) || split(root, arena) {}
}

pub fn magnitude(index: Index, arena: &Arena) -> u32 {
    let node = arena.get(index).unwrap().borrow();
    match *node {
        Node::Leaf(num) => num,
        Node::Node(left, right) => 3 * magnitude(left, arena) + 2 * magnitude(right, arena),
    }
}

enum ParseNode {
    Leaf(u32),
    Node(Option<Index>, Option<Index>),
}

pub fn parse_stdin() -> (Arena, Vec<Index>) {
    let mut arena = Arena::new();
    let mut numbers = Vec::new();

    for line in io::stdin().lock().lines() {
        numbers.push(parse_line(&line.unwrap(), &mut arena));
    }

    (arena, numbers)
}

pub fn parse_string(string: &str) -> (Arena, Vec<Index>) {
    let mut arena = Arena::new();
    let mut numbers = Vec::new();

    for line in string.lines() {
        if !line.trim().is_empty() {
            numbers.push(parse_line(&line.trim(), &mut arena));
        }
    }

    (arena, numbers)
}

fn parse_line(line: &str, arena: &mut Arena) -> Index {
    let mut parse_nodes: Vec<ParseNode> = Vec::new();
    for c in line.chars() {
        match c {
            '[' => { parse_nodes.push(ParseNode::Node(None, None)); },
            '0'..='9' => { parse_nodes.push(ParseNode::Leaf(c as u32 - 0x30 /*'0'*/)); },
            ',' => {
                let child = parse_nodes.pop().unwrap();
                let index = arena.insert(RefCell::new(Node::from_parse_node(child)));
                if let ParseNode::Node(left, _) = parse_nodes.last_mut().unwrap() {
                    *left = Some(index);
                } else { panic!("Invalid parse node type!"); }
            },
            ']' => {
                let child = parse_nodes.pop().unwrap();
                let index = arena.insert(RefCell::new(Node::from_parse_node(child)));
                if let ParseNode::Node(_, right) = parse_nodes.last_mut().unwrap() {
                    *right = Some(index);
                } else { panic!("Invalid parse node type!"); }
            },
            _ => panic!("Invalid character: {}", c),
        }
    }
    let root = parse_nodes.pop().unwrap();
    assert!(parse_nodes.is_empty());
    arena.insert(RefCell::new(Node::from_parse_node(root)))
}
