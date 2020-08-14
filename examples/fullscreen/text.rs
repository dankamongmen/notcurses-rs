use notcurses::{Error, Options, NotCurses};

fn main() -> Result<(), Error> {

    // let mut nc = NotCurses::new(Options::new())?;
    let mut nc = NotCurses::new(Options::without_altmode())?;

    println!("dim_yx={:?}", nc.stdplane().dim_yx());
    println!("supported_styles: {:#b}", nc.supported_styles());
    println!("supported_styles_str: {:?}", nc.supported_styles_str());

    Ok(())
}
