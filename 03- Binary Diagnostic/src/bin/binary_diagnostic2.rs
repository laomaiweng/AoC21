use std::io::{self, BufRead};
use std::error::Error;

const NBITS: usize = 12;

fn partition_values<'a, 'b>(values: &'b [&'a str], index: usize) -> (Vec<&'a str>, Vec<&'a str>) {
    values.iter().partition(|&v| v.chars().nth(index).unwrap() == '1')
}

fn filter_most_common<'a, 'b>(values: &'b [&'a str], index: usize) -> Vec<&'a str> {
    let (ones, zeroes) = partition_values(values, index);
    if zeroes.len() <= ones.len() {
        // keep values with a 1
        ones
    } else {
        // keep values with a 0
        zeroes
    }
}

fn filter_least_common<'a, 'b>(values: &'b [&'a str], index: usize) -> Vec<&'a str> {
    let (ones, zeroes) = partition_values(values, index);
    if zeroes.len() <= ones.len() {
        // keep values with a 0
        zeroes
    } else {
        // keep values with a 1
        ones
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    let stdin = io::stdin();

    let report: Vec<String> = stdin.lock().lines().flatten().collect();
    let mut oxygen_values: Vec<_> = report.iter().map(String::as_str).collect();
    let mut co2_values: Vec<_> = report.iter().map(String::as_str).collect();
    for i in 0..NBITS {
        if oxygen_values.len() > 1 {
            oxygen_values = filter_most_common(&oxygen_values, i);
        }
        if co2_values.len() > 1 {
            co2_values = filter_least_common(&co2_values, i);
        }
    }
    assert_eq!(oxygen_values.len(), 1);
    assert_eq!(co2_values.len(), 1);

    let oxygen = u32::from_str_radix(oxygen_values[0], 2)?;
    let co2 = u32::from_str_radix(co2_values[0], 2)?;

    println!("oxygen: {}", oxygen);
    println!("co2: {}", co2);
    Ok(())
}
