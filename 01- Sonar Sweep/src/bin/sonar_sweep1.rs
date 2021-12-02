use std::io;
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    let stdin = io::stdin();

    let mut depth: Option<u32> = None;
    let mut increases = 0;

    loop {
        let mut buffer = String::new();
        let read = stdin.read_line(&mut buffer)?;
        if read == 0 {
            break;
        }

        let new_depth: u32 = buffer.trim().parse()?;
        if let Some(old_depth) = depth {
            if new_depth > old_depth {
                eprintln!("increased");
                increases += 1;
            } else if new_depth == old_depth {
                eprintln!("unchanged");
            } else {
                eprintln!("decreased");
            }
        }
        depth = Some(new_depth);
    }
    println!("{}", increases);
    Ok(())
}
