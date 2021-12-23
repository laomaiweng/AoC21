use std::cmp;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops;
use std::io::{self, BufRead};

use itertools::iproduct;

#[derive(Debug)]
struct Partition<T> {
    disjoint: Vec<T>,
    intersect: Option<T>,
}

impl<T> Partition<T> {
    fn has_intersection(&self) -> bool {
        self.intersect.is_some()
    }

    fn into_parts(mut self) -> Vec<T> {
        if let Some(intersect) = self.intersect {
            self.disjoint.push(intersect);
        }
        self.disjoint
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Range(ops::RangeInclusive<i32>);

impl Range {
    fn size(&self) -> usize {
        (*self.0.end() - *self.0.start()) as usize + 1
    }

    fn partition(&self, other: &Self) -> Partition<Self> {
        // Easy case: no intersection.
        /* +-----+                  *
         *             +----------+ */
        if self.0.end() < other.0.start() || other.0.end() < self.0.start() {
            return Partition {
                disjoint: vec![self.clone()],
                intersect: None,
            };
        }

        // Some intersection.
        let mut disjoint = Vec::new();
        let range = |start, end| Range(start..=end);

        if self.0.start() < other.0.start() {
            /* +ddddddd------------ *
             *         +----------- */
            disjoint.push(range(*self.0.start(), *other.0.start()-1));
        }
        if other.0.end() < self.0.end() {
            /* -------------dddddd+ *
             * ------------+        */
            disjoint.push(range(*other.0.end()+1, *self.0.end()));
        }
        /* +-------iiii+        *
         *         +----------+ */
        let intersect_start = cmp::max(*self.0.start(), *other.0.start());
        let intersect_end = cmp::min(*self.0.end(), *other.0.end());
        let intersect = Some(range(intersect_start, intersect_end));

        Partition { disjoint, intersect }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Cube {
    x: Range,
    y: Range,
    z: Range,
}

impl Cube {
    fn size(&self) -> usize {
        self.x.size() * self.y.size() * self.z.size()
    }

    fn size_in(&self, bounding_rect: &Cube) -> usize {
        let partition = self.partition(bounding_rect);
        if let Some(intersect) = partition.intersect {
            intersect.size()
        } else {
            0
        }
    }

    fn partition(&self, other: &Self) -> Partition<Self> {
        let x_partition = self.x.partition(&other.x);
        let y_partition = self.y.partition(&other.y);
        let z_partition = self.z.partition(&other.z);

        if !x_partition.has_intersection() || !y_partition.has_intersection() || !z_partition.has_intersection() {
            return Partition {
                disjoint: vec![self.clone()],
                intersect: None,
            }
        }

        // Combine the partitions.
        let x_segments = x_partition.into_parts();
        let y_segments = y_partition.into_parts();
        let z_segments = z_partition.into_parts();
        let mut cubes: Vec<Cube> = iproduct!(x_segments, y_segments, z_segments).map(|(x,y,z)| Cube { x, y, z, }).collect();
        // The intersecting part of each partition is the last segment, so the last cube is the
        // intersecting cube.
        let intersect = cubes.pop();

        Partition {
            disjoint: cubes,
            intersect,
        }
    }
}

impl<T> PartialEq for Partition<T> 
where
    T: Eq + Hash + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        // Ignore order in vec of disjoint parts when comparing.
        let h1: HashSet<T> = HashSet::from_iter(self.disjoint.iter().cloned());
        let h2 = HashSet::from_iter(other.disjoint.iter().cloned());
        h1 == h2 && self.intersect == other.intersect
    }
}

impl<T: Eq + Hash + Clone> Eq for Partition<T> {}

pub fn cube(x: ops::RangeInclusive<i32>, y: ops::RangeInclusive<i32>, z: ops::RangeInclusive<i32>) -> Cube {
    Cube {
        x: Range(x),
        y: Range(y),
        z: Range(z),
    }
}

#[derive(Debug)]
pub struct Step {
    cube: Cube,
    state: bool,
}

#[derive(Debug)]
pub struct Reactor {
    /// ON cubes.
    areas: Vec<Cube>,
}

impl Reactor {
    pub fn new() -> Self {
        Reactor {
            areas: Vec::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.areas.iter().fold(0, |acc, cube| acc + cube.size())
    }

    pub fn count_in(&self, bounding_rect: &Cube) -> usize {
        self.areas.iter().fold(0, |acc, cube| acc + cube.size_in(bounding_rect))
    }

    pub fn do_step(&mut self, step: &Step) {
        let mut new_areas = Vec::new();
        for cube in &self.areas {
            let mut partition = cube.partition(&step.cube);
            new_areas.append(&mut partition.disjoint);
        }
        if step.state {
            new_areas.push(step.cube.clone());
        }
        self.areas = new_areas;
    }
}

pub fn parse_stdin() -> Vec<Step> {
    let mut steps = Vec::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let state = match parts[0] {
            "on" => true,
            "off" => false,
            _ => panic!("Invalid state!"),
        };
        let ranges: Vec<_> = parts[1].split(',').map(|r|
            r.split('=').nth(1).unwrap().split("..").map(|n|
                n.parse::<i32>().unwrap()
            ).collect::<Vec<_>>()
        ).map(|r| r[0]..=r[1]).collect();
        steps.push(Step {
            cube: Cube {
                x: Range(ranges[0].clone()),
                y: Range(ranges[1].clone()),
                z: Range(ranges[2].clone()),
            },
            state,
        });
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_1d_none() {
        let a = Range(0..=2);
        let b = Range(3..=5);
        let expected = Partition {
            disjoint: vec![
                Range(0..=2),
            ],
            intersect: None,
        };
        assert_eq!(a.partition(&b), expected);
    }

    #[test]
    fn partition_1d_left() {
        let a = Range(0..=3);
        let b = Range(2..=5);
        let expected = Partition {
            disjoint: vec![
                Range(0..=1),
            ],
            intersect: Some(Range(2..=3)),
        };
        assert_eq!(a.partition(&b), expected);
    }

    #[test]
    fn partition_1d_right() {
        let a = Range(2..=5);
        let b = Range(0..=3);
        let expected = Partition {
            disjoint: vec![
                Range(4..=5),
            ],
            intersect: Some(Range(2..=3)),
        };
        assert_eq!(a.partition(&b), expected);
    }

    #[test]
    fn partition_1d_inner() {
        let a = Range(2..=3);
        let b = Range(0..=5);
        let expected = Partition {
            disjoint: vec![
            ],
            intersect: Some(Range(2..=3)),
        };
        assert_eq!(a.partition(&b), expected);
    }

    #[test]
    fn partition_1d_outer() {
        let a = Range(0..=5);
        let b = Range(2..=3);
        let expected = Partition {
            disjoint: vec![
                Range(0..=1),
                Range(4..=5),
            ],
            intersect: Some(Range(2..=3)),
        };
        assert_eq!(a.partition(&b), expected);
    }

    #[test]
    fn partition_1d_equal() {
        let a = Range(0..=5);
        let b = Range(0..=5);
        let expected = Partition {
            disjoint: vec![
            ],
            intersect: Some(Range(0..=5)),
        };
        assert_eq!(a.partition(&b), expected);
    }

    #[test]
    fn partition_3d() {
        let a = cube(10..=12, 10..=12, 10..=12);
        let b = cube(11..=13, 11..=13, 11..=13);
        let expected = Partition {
            disjoint: vec![
                cube(10..=10, 10..=10, 10..=10),
                cube(10..=10, 11..=12, 10..=10),
                cube(11..=12, 10..=10, 10..=10),
                cube(11..=12, 11..=12, 10..=10),
                cube(10..=10, 10..=10, 11..=12),
                cube(10..=10, 11..=12, 11..=12),
                cube(11..=12, 10..=10, 11..=12),
            ],
            intersect: Some(cube(11..=12, 11..=12, 11..=12)),
        };
        assert_eq!(a.partition(&b), expected);
    }
}
