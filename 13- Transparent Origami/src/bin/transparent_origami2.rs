use transparent_origami::parse_stdin;

fn main() {
    let (mut grid, folds) = parse_stdin();

    println!("grid has {} dots", grid.count());
    for (i, fold) in folds.iter().enumerate() {
        grid.fold(fold);
        println!("grid has {} dots after fold {}", grid.count(), i+1);
    }
    println!("{}", grid);
}
