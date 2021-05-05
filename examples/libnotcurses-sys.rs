//!

use notcurses::sys::*;

fn main() -> NcResult<()> {
    let nc = Notcurses::new()?;

    println!("hello world");

    sleep![1];

    nc.stop()?;
    Ok(())
}
