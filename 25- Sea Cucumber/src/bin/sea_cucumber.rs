#[cfg(feature = "curses")]
use pancurses;

use sea_cucumber::parse_stdin;
#[cfg(feature = "curses")]
use sea_cucumber::{init_curses, fini_curses};

fn main() {
    let mut map = parse_stdin();
    #[cfg(feature = "curses")]
    let window = init_curses(&map);

    let mut count = 1;
    while map.step() != 0 {
        count += 1;

        #[cfg(feature = "curses")]
        {
            map.render(&window);
            pancurses::napms(100);
        }
    }

    println!("Reached a fixed point after {} moves.", count);

    #[cfg(feature = "curses")]
    fini_curses(window);
}
