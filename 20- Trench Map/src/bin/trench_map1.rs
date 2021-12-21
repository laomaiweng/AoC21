use trench_map::parse_stdin;

fn main() {
    let (algo, image) = parse_stdin();
    println!("Initial image: {}x{}, {} pixels lit\n{}", image.rows(), image.cols(), image.count(), image);
    let image1 = image.enhance(&algo);
    println!("Enhance 1: {}x{}, {} pixels lit\n{}", image1.rows(), image1.cols(), image1.count(), image1);
    let image2 = image1.enhance(&algo);
    println!("Enhance 2: {}x{}, {} pixels lit\n{}", image2.rows(), image2.cols(), image2.count(), image2);
}
