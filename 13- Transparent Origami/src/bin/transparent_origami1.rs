use transparent_origami::parse_stdin;

fn main() {
    let (mut grid, folds) = parse_stdin();

    println!("grid has {} dots", grid.count());
    grid.fold(&folds[0]);
    println!("grid has {} dots after 1st fold", grid.count());
}
