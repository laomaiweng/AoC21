use hydrothermal_venture::Grid;

fn main() {
    let lines = hydrothermal_venture::parse_stdin();
    let mut grid = Grid::new();

    for line in &lines {
        //eprintln!("line: {}", line);
        if (line.a.x == line.b.x) || (line.a.y == line.b.y) {
            grid.draw(line);
            //eprintln!("{}", grid);
        }
    }

    println!("overlapping: {}", grid.count_overlapping());
}
