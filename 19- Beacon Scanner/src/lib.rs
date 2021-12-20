use std::cmp;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{self, BufRead};

use nalgebra::{matrix, Matrix3, Vector3};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coords(Vector3<i32>);

// Implement our own total order since nalgebra's PartialOrd doesn't fit our purposes.
impl Ord for Coords {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        for (i,j) in self.0.iter().zip(other.0.iter()) {
            let local = i.cmp(j);
            if local != cmp::Ordering::Equal {
                return local;
            }
        }
        cmp::Ordering::Equal
    }
}

impl PartialOrd for Coords {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type Rotation = Matrix3<i32>;
type Translation = Vector3<i32>;

fn rotations() -> Vec<Rotation> {
    let mut rotations = Vec::new();

    /*          Θ =  0° 90° 180° 270° */
    let cos_theta = [1,  0,  -1,   0];
    let sin_theta = [0,  1,   0,  -1];

    // Rotations around +x.
    let x = matrix![1, 0, 0;
                    0, 1, 0;
                    0, 0, 1];
    for (cos, sin) in cos_theta.iter().copied().zip(sin_theta.iter().copied()) {
        rotations.push(x * matrix![1,   0,    0;
                                   0, cos, -sin;
                                   0, sin,  cos]);
    }
    // Rotations around -x.
    let minus_x = matrix![-1, 0,  0;
                           0, 1,  0;
                           0, 0, -1];
    for (cos, sin) in cos_theta.iter().copied().zip(sin_theta.iter().copied()) {
        rotations.push(minus_x * matrix![1,   0,    0;
                                         0, cos, -sin;
                                         0, sin,  cos]);
    }

    // Rotations around +y.
    let y = matrix![0, -1, 0;
                    1,  0, 0;
                    0,  0, 1];
    for (cos, sin) in cos_theta.iter().copied().zip(sin_theta.iter().copied()) {
        rotations.push(y * matrix![ cos, 0, sin;
                                      0, 1,   0;
                                   -sin, 0, cos]);
    }
    // Rotations around -y.
    let minus_y = matrix![ 0, 1, 0;
                          -1, 0, 0;
                           0, 0, 1];
    for (cos, sin) in cos_theta.iter().copied().zip(sin_theta.iter().copied()) {
        rotations.push(minus_y * matrix![ cos, 0, sin;
                                            0, 1,   0;
                                         -sin, 0, cos]);
    }

    // Rotations around +z.
    let z = matrix![0, 0, -1;
                    0, 1,  0;
                    1, 0,  0];
    for (cos, sin) in cos_theta.iter().copied().zip(sin_theta.iter().copied()) {
        rotations.push(z * matrix![cos, -sin, 0;
                                   sin,  cos, 0;
                                     0,    0, 1]);
    }
    // Rotations around -z.
    let minus_z = matrix![ 0, 0, 1;
                           0, 1, 0;
                          -1, 0, 0];
    for (cos, sin) in cos_theta.iter().copied().zip(sin_theta.iter().copied()) {
        rotations.push(minus_z * matrix![cos, -sin, 0;
                                         sin,  cos, 0;
                                           0,    0, 1]);
    }

    rotations
}

pub struct Scanner {
    pub index: u32,
    beacons: BTreeSet<Coords>,
    rotation: Rotation,
    translation: Translation,
}

impl Scanner {
    pub fn new(index: u32) -> Self {
        Scanner {
            index,
            beacons: BTreeSet::new(),
            rotation: Rotation::identity(),
            translation: Translation::zeros(),
        }
    }

    pub fn count(&self) -> usize {
        self.beacons.len()
    }

    pub fn distance(&self, other: &Self) -> Translation {
        other.translation - self.translation
    }

    pub fn collide(&self, other: &Scanner, threshold: usize) -> Option<(Rotation, Translation)> {
        for r in rotations() {
            // Rotate the beacons of the other scanner.
            let rotated_beacons: Vec<Coords> = other.beacons.iter().map(|b| Coords(r * b.0)).collect();

            // Translate each of them onto each of our beacons and check how many other beacons match.
            for b1 in &self.beacons {
                for b2 in &rotated_beacons {
                    let translation = b1.0 - b2.0;
                    let translated_beacons: BTreeSet<Coords> = rotated_beacons.iter().map(|b| Coords(b.0 + translation)).collect();
                    let count = self.beacons.intersection(&translated_beacons).count();
                    if count >= threshold {
                        // Bingo, got enough matches to return the transform.
                        return Some((r, translation));
                    }
                }
            }
        }
        None
    }

    pub fn transform(&mut self, rotation: Rotation, translation: Translation) {
        self.rotation = rotation;
        self.translation = translation;
        self.beacons = self.beacons.iter().map(|b| Coords(rotation * b.0 + translation)).collect();
    }

    pub fn merge(&mut self, other: &Self) {
        self.beacons.extend(&other.beacons);
    }
}

pub fn large_scanner_collider(scanners: &mut[Scanner]) {
    // Create the sets of merged and unmerged scanners (by their indices into the array of scanners
    // passed as argument). The set of merged scanners consists only of scanner 0 initially.
    let mut merged = vec![0];
    let mut unmerged: Vec<usize> = (1..scanners.len()).collect();

    // Remember scanners that didn't match so we don't attempt to match them again.
    let mut all_mismatches = HashMap::new();

    // Iterate over unmerged scanners, trying to match them against any merged scanner.
    // Once we find a match, transform it and move it to the list of merged scanners.
    while !unmerged.is_empty() {
        let mut collided = None;
        'unmerged: for (i, scanner) in unmerged.iter().enumerate() {
            let scanner = &scanners[*scanner];
            let mismatches = all_mismatches.entry(scanner.index).or_insert(HashSet::new());
            for candidate in &merged {
                let candidate = &scanners[*candidate];
                if mismatches.contains(&candidate.index) {
                    continue;
                }
                if let Some((rotation, translation)) = candidate.collide(&scanner, 12) {
                    println!("Collided scanners {} & {}!", candidate.index, scanner.index);
                    // Remember the index into the unmerged set so we can move it outside of the
                    // loops (and their borrows).
                    collided = Some((i, rotation, translation));
                    break 'unmerged;
                } else {
                    mismatches.insert(candidate.index);
                }
            }
        }
        if let Some((i, rotation, translation)) = collided {
            // A scanner matched: transform it and move it.
            let index = unmerged.swap_remove(i);
            let scanner = &mut scanners[index];
            scanner.transform(rotation, translation);
            merged.push(index);
        } else {
            panic!("Failed to collide any remaining unmerged scanners!");
        }
    }
}

pub fn parse_stdin() -> Vec<Scanner> {
    let mut scanners = Vec::new();

    let mut scanner = None;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        } else if line.starts_with("--- scanner") {
            scanners.push(Scanner::new(scanners.len() as u32));
            scanner = scanners.last_mut();
        } else if let Some(ref mut scanner) = scanner {
            scanner.beacons.insert(parse_coords(&line));
        }
    }

    scanners
}

fn parse_coords(string: &str) -> Coords {
    Coords(Vector3::from_vec(string.split(',').map(|n| n.parse().unwrap()).collect()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::vector;

    fn parse_scanner(string: &str) -> Scanner {
        let mut scanner = Scanner::new(0);
        for line in string.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }
            scanner.beacons.insert(parse_coords(line));
        }
        scanner
    }

    #[test]
    fn rotate() {
        let initial = vec![
            vector![-1,-1,1],
            vector![-2,-2,2],
            vector![-3,-3,3],
            vector![-2,-3,1],
            vector![5,6,-4],
            vector![8,0,7],
        ];
        let all_rotations: Vec<Vec<_>> = rotations().iter().map(|r|
            initial.iter().map(|i| *r * *i).collect::<Vec<_>>()
        ).collect();

        let target1 = vec![
            vector![1,-1,1],
            vector![2,-2,2],
            vector![3,-3,3],
            vector![2,-1,3],
            vector![-5,4,-6],
            vector![-8,-7,0],
        ];
        assert!(all_rotations.contains(&target1));

        let target2 = vec![
            vector![-1,-1,-1],
            vector![-2,-2,-2],
            vector![-3,-3,-3],
            vector![-1,-3,-2],
            vector![4,6,5],
            vector![-7,0,8],
        ];
        assert!(all_rotations.contains(&target2));

        let target3 = vec![
            vector![1,1,-1],
            vector![2,2,-2],
            vector![3,3,-3],
            vector![1,3,-2],
            vector![-4,-6,5],
            vector![7,0,8],
        ];
        assert!(all_rotations.contains(&target3));

        let target4 = vec![
            vector![1,1,1],
            vector![2,2,2],
            vector![3,3,3],
            vector![3,1,2],
            vector![-6,-4,-5],
            vector![0,7,-8],
        ];
        assert!(all_rotations.contains(&target4));
    }

    #[test]
    fn full_collision() {
        let scanner0 = parse_scanner(r#"
            -1,-1,1
            -2,-2,2
            -3,-3,3
            -2,-3,1
            5,6,-4
            8,0,7
        "#);
        let mut scanner1 = parse_scanner(r#"
            1,-1,1
            2,-2,2
            3,-3,3
            2,-1,3
            -5,4,-6
            -8,-7,0
        "#);
        let (rotation, translation) = scanner0.collide(&scanner1, scanner0.beacons.len()).expect("No matching transform found!");
        scanner1.transform(rotation, translation);
        assert_eq!(scanner0.beacons, scanner1.beacons);
    }

    #[test]
    fn partial_collision() {
        let scanner0 = parse_scanner(r#"
            404,-588,-901
            528,-643,409
            -838,591,734
            390,-675,-793
            -537,-823,-458
            -485,-357,347
            -345,-311,381
            -661,-816,-575
            -876,649,763
            -618,-824,-621
            553,345,-567
            474,580,667
            -447,-329,318
            -584,868,-557
            544,-627,-890
            564,392,-477
            455,729,728
            -892,524,684
            -689,845,-530
            423,-701,434
            7,-33,-71
            630,319,-379
            443,580,662
            -789,900,-551
            459,-707,401
        "#);

        let mut scanner1 = parse_scanner(r#"
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            95,138,22
            -476,619,847
            -340,-569,-846
            567,-361,727
            -460,603,-452
            669,-402,600
            729,430,532
            -500,-761,534
            -322,571,750
            -466,-666,-811
            -429,-592,574
            -355,545,-477
            703,-491,-529
            -328,-685,520
            413,935,-424
            -391,539,-444
            586,-435,557
            -364,-763,-893
            807,-499,-711
            755,-354,-619
            553,889,-390
        "#);
        let (rotation1, translation1) = scanner0.collide(&scanner1, 12).expect("No matching transform found!");
        assert_eq!(translation1, vector![68, -1246, -43]);
        scanner1.transform(rotation1, translation1);
        let overlap1: BTreeSet<_> = scanner0.beacons.intersection(&scanner1.beacons).copied().collect();
        let expected1 = parse_scanner(r#"
            -618,-824,-621
            -537,-823,-458
            -447,-329,318
            404,-588,-901
            544,-627,-890
            528,-643,409
            -661,-816,-575
            390,-675,-793
            423,-701,434
            -345,-311,381
            459,-707,401
            -485,-357,347
        "#);
        assert_eq!(overlap1, expected1.beacons);

        let mut scanner4 = parse_scanner(r#"
            727,592,562
            -293,-554,779
            441,611,-461
            -714,465,-776
            -743,427,-804
            -660,-479,-426
            832,-632,460
            927,-485,-438
            408,393,-506
            466,436,-512
            110,16,151
            -258,-428,682
            -393,719,612
            -211,-452,876
            808,-476,-593
            -575,615,604
            -485,667,467
            -680,325,-822
            -627,-443,-432
            872,-547,-609
            833,512,582
            807,604,487
            839,-516,451
            891,-625,532
            -652,-548,-490
            30,-46,-14
        "#);
        let (rotation4, translation4) = scanner1.collide(&scanner4, 12).expect("No matching transform found!");
        assert_eq!(translation4, vector![-20, -1133, 1061]);
        scanner4.transform(rotation4, translation4);
        let overlap4: BTreeSet<_> = scanner1.beacons.intersection(&scanner4.beacons).copied().collect();
        let expected4 = parse_scanner(r#"
            459,-707,401
            -739,-1745,668
            -485,-357,347
            432,-2009,850
            528,-643,409
            423,-701,434
            -345,-311,381
            408,-1815,803
            534,-1912,768
            -687,-1600,576
            -447,-329,318
            -635,-1737,486
        "#);
        assert_eq!(overlap4, expected4.beacons);
    }
}
