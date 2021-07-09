//! info

use notcurses::*;

fn main() -> NResult<()> {
    let nc = Notcurses::without_altscreen()?;

    let caps = nc.capabilities();
    let geom = nc.geometry();

    println!("{:#?}\n{:#?}", caps, geom);

    Ok(())
}
