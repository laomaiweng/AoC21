use alu::brute_force_monad;

fn main() {
    let models = brute_force_monad();
    println!("Min model number: {}", models.iter().min().unwrap());
}
