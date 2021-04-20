//!

use notcurses::sys::*;

fn main() -> NcResult<()> {
    let _nc = Nc::new();

    println!("hello world");

    sleep![1];

    Ok(())
}
