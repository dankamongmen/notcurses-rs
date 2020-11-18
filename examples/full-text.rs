use notcurses::{Error, Notcurses};

fn main() -> Result<(), Error> {

    let mut nc = Notcurses::without_altmode()?;

    println!("dim_yx={:?}", nc.stdplane().dim_yx());
    println!("supported_styles: {:#b}", nc.supported_styles());
    println!("supported_styles_str: {:?}", nc.supported_styles_str());

    Ok(())
}
