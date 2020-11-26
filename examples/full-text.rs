use notcurses::{Error, FullMode};

fn main() -> Result<(), Error> {
    let mut nc = FullMode::without_altmode()?;

    println!("dim_yx={:?}", nc.stdplane().dim_yx());
    println!("supported_styles: {:#b}", nc.supported_styles());
    println!("supported_styles_str: {:?}", nc.supported_styles_str());

    let (trows, tcols) = nc.dim_yx();
    println!("terminal rows:{} cols:{}", trows, tcols);

    Ok(())
}