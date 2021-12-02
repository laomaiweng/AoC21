use std::io;
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    let stdin = io::stdin();

    let mut position: i32 = 0;
    let mut depth: i32 = 0;

    loop {
        let mut buffer = String::new();
        let read = stdin.read_line(&mut buffer)?;
        if read == 0 {
            break;
        }

        let mut tokens = buffer.split_whitespace();
        let movement = tokens.next().unwrap();
        let value: i32 = tokens.next().unwrap().parse()?;

        match movement {
            "forward" => position += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => eprintln!("unknown command: {}", movement),
        }
    }

    println!("position: {}", position);
    println!("depth: {}", depth);
    Ok(())
}
