use std::collections::{HashMap, HashSet};

use seven_segment_search::{Display, parse_stdin};

type Digits = HashMap<String, u32>;

fn solve_display(display: &Display) -> Digits {
    let digit_1 = display.digits.iter().find(|s| s.len() == 2).unwrap();
    let digit_4 = display.digits.iter().find(|s| s.len() == 4).unwrap();
    let digit_7 = display.digits.iter().find(|s| s.len() == 3).unwrap();
    let digit_8 = display.digits.iter().find(|s| s.len() == 7).unwrap();

    let segments_cf: HashSet<_> = digit_1.chars().collect();
    let segments_bdcf: HashSet<_> = digit_4.chars().collect();
    let segments_acf: HashSet<_> = digit_7.chars().collect();
    let segments_abcdefg: HashSet<_> = digit_8.chars().collect();
    let segments_a = &segments_acf - &segments_cf;
    assert_eq!(segments_a.len(), 1);
    let segments_bd: HashSet<_> = &segments_bdcf - &segments_cf;
    let segments_eg: HashSet<_> = &(&segments_abcdefg - &segments_bdcf) - &segments_a;

    let digit_9 = display.digits.iter().find(
        |s| {
            let segs: HashSet<_> = s.chars().collect();
            segments_eg.is_superset(&(&segments_abcdefg - &segs)) && *s != digit_8
        }
    ).unwrap();
    let segments_e: HashSet<_> = &segments_abcdefg - &digit_9.chars().collect();
    assert_eq!(segments_e.len(), 1);
    let segments_g: HashSet<_> = &segments_eg - &segments_e;
    assert_eq!(segments_g.len(), 1);

    let digit_6 = display.digits.iter().find(
        |s| {
            let segs: HashSet<_> = s.chars().collect();
            segments_cf.is_superset(&(&segments_abcdefg - &segs)) && *s != digit_8
        }
    ).unwrap();
    let segments_c: HashSet<_> = &segments_abcdefg - &digit_6.chars().collect();
    assert_eq!(segments_c.len(), 1);
    let segments_f: HashSet<_> = &segments_cf - &segments_c;
    assert_eq!(segments_f.len(), 1);

    let digit_0 = display.digits.iter().find(
        |s| {
            let segs: HashSet<_> = s.chars().collect();
            segments_bd.is_superset(&(&segments_abcdefg - &segs)) && *s != digit_8
        }
    ).unwrap();
    let segments_d: HashSet<_> = &segments_abcdefg - &digit_0.chars().collect();
    assert_eq!(segments_d.len(), 1);
    let segments_b: HashSet<_> = &segments_bd - &segments_d;
    assert_eq!(segments_b.len(), 1);

    let a = *segments_a.iter().next().unwrap();
    let b = *segments_b.iter().next().unwrap();
    let c = *segments_c.iter().next().unwrap();
    let d = *segments_d.iter().next().unwrap();
    let e = *segments_e.iter().next().unwrap();
    let f = *segments_f.iter().next().unwrap();
    let g = *segments_g.iter().next().unwrap();

    let mut segments_0: Vec<_> = [a,b,c,e,f,g].iter().copied().collect();
    segments_0.sort_unstable();
    let mut segments_1: Vec<_> = [c,f].iter().copied().collect();
    segments_1.sort_unstable();
    let mut segments_2: Vec<_> = [a,c,d,e,g].iter().copied().collect();
    segments_2.sort_unstable();
    let mut segments_3: Vec<_> = [a,c,d,f,g].iter().copied().collect();
    segments_3.sort_unstable();
    let mut segments_4: Vec<_> = [b,c,d,f].iter().copied().collect();
    segments_4.sort_unstable();
    let mut segments_5: Vec<_> = [a,b,d,f,g].iter().copied().collect();
    segments_5.sort_unstable();
    let mut segments_6: Vec<_> = [a,b,d,e,f,g].iter().copied().collect();
    segments_6.sort_unstable();
    let mut segments_7: Vec<_> = [a,c,f].iter().copied().collect();
    segments_7.sort_unstable();
    let mut segments_8: Vec<_> = [a,b,c,d,e,f,g].iter().copied().collect();
    segments_8.sort_unstable();
    let mut segments_9: Vec<_> = [a,b,c,d,f,g].iter().copied().collect();
    segments_9.sort_unstable();

    let mut digits = Digits::new();
    digits.insert(segments_0.iter().collect(), 0u32);
    digits.insert(segments_1.iter().collect(), 1u32);
    digits.insert(segments_2.iter().collect(), 2u32);
    digits.insert(segments_3.iter().collect(), 3u32);
    digits.insert(segments_4.iter().collect(), 4u32);
    digits.insert(segments_5.iter().collect(), 5u32);
    digits.insert(segments_6.iter().collect(), 6u32);
    digits.insert(segments_7.iter().collect(), 7u32);
    digits.insert(segments_8.iter().collect(), 8u32);
    digits.insert(segments_9.iter().collect(), 9u32);

    digits
}

fn compute_output(display: &Display, digits: &Digits) -> u32 {
    let mut output = 0u32;
    let mut mult = 1000u32;
    for d in &display.output {
        let mut segs: Vec<_> = d.chars().collect();
        segs.sort_unstable();
        output += mult * digits.get(&segs.iter().collect::<String>()).unwrap();
        mult /= 10u32;
    }
    output
}

fn main() {
    let displays = parse_stdin();

    let mut sum = 0u32;
    for d in &displays {
        let digits = solve_display(d);
        let output = compute_output(&d, &digits);
        println!("{}", output);
        sum += output;
    }

    println!("total: {}", sum);
}
