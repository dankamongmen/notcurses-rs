//! Shows how you can use the libnotcurses-sys library directly.

use notcurses::sys::*;

fn main() -> NcResult<()> {
    let nc = Nc::new()?;

    println!("hello world");

    sleep![1];

    nc.stop()?;
    Ok(())
}
