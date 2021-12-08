use seven_segment_search::parse_stdin;

fn main() {
    let displays = parse_stdin();

    // lengths for digits:     1, 4, 7, 8
    let lengths: [usize; 4] = [2, 4, 3, 7];
    let count: usize = displays.iter().map(
        |l| l.output.iter().filter(
            |o| lengths.contains(&o.len())
        ).count()
    ).sum();
    println!("1/4/7/8 count: {}", count);
}
