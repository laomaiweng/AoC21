use std::io;
use std::error::Error;

const WINDOWS: usize = 4;

fn main() -> Result<(),Box<dyn Error>> {
    let stdin = io::stdin();

    let mut windows: [u32; WINDOWS] = [0; WINDOWS];
    let mut measurements = 0;
    let mut increases = 0;

    loop {
        let mut buffer = String::new();
        let read = stdin.read_line(&mut buffer)?;
        if read == 0 {
            break;
        }

        let depth: u32 = buffer.trim().parse()?;

        // add measurement to windows
        windows[measurements % WINDOWS] += depth;
        if measurements > 0 {
            windows[(measurements - 1) % WINDOWS] += depth;
        }
        if measurements > 1 {
            windows[(measurements - 2) % WINDOWS] += depth;
        }

        //eprintln!("STATE: {:#?}", windows);

        // compare and erase the last measurement
        if measurements == 2 {
            eprintln!("{} (N/A - no previous sum)", windows[0]);
        }
        if measurements > 2 {
            let old_window = windows[(measurements - 3) % WINDOWS];
            let new_window = windows[(measurements - 2) % WINDOWS];
            if old_window < new_window {
                eprintln!("{} (increased)", new_window);
                increases += 1;
            } else if old_window == new_window {
                eprintln!("{} (no change)", new_window);
            } else {
                eprintln!("{} (decreased)", new_window);
            }
            windows[(measurements - 3) % WINDOWS] = 0;
        }

        measurements += 1;
    }
    println!("total: {} increases", increases);
    Ok(())
}
