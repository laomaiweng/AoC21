use alu::brute_force_monad;

fn main() {
    let models = brute_force_monad();
    println!("Max model number: {}", models.iter().max().unwrap());
}
