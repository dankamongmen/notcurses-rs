//! info

use notcurses::*;

fn main() -> Result<()> {
    let nc = Notcurses::new_cli_silent()?;

    println!("{:#?}", nc.capabilities());

    Ok(())
}
