use trench_map::parse_stdin;

fn main() {
    let (algo, mut image) = parse_stdin();
    println!("Initial image: {}x{}, {} pixels lit\n{}", image.rows(), image.cols(), image.count(), image);
    for i in 0..50 {
        image = image.enhance(&algo);
        println!("Enhance {}: {}x{}, {} pixels lit", i, image.rows(), image.cols(), image.count());
    }
}
