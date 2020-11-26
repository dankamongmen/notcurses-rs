//! Example 'direct-cursor'
//!
//! Explore cursor functions in direct mode
//!

use notcurses::{DirectMode, Error};

// utility macro: sleep for $ms milliseconds
macro_rules! sleep {
    ($ms:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($ms));
    };
}

fn main() -> Result<(), Error> {
    let mut ncd = DirectMode::new()?;

    println!("terminal size (rows, cols): {}, {}", ncd.rows(), ncd.cols());

    ncd.print_colored(0, "The current coordinates are")?;
    ncd.flush()?;

    for _ in 0..20 {
        ncd.print_colored(0, ".")?;
        ncd.flush()?;
        sleep![50];
    }

    let (cy, cx) = ncd.cursor_yx()?;
    ncd.print_colored(0, &format![" ({},{})\n", cy, cx])?;
    sleep![1000];

    let sentence = vec![
        "And", "now", "I", "will", "clear", "the", "screen", ".", ".", ".",
    ];
    for word in sentence {
        ncd.print_colored(0, &format!["{} ", word])?;
        ncd.flush()?;
        sleep![200];
    }
    sleep![300];
    ncd.print_colored(0, "\nbye!\n\n")?;
    ncd.flush()?;
    sleep![600];

    ncd.clear()?;
    sleep![1000];

    Ok(())
}
