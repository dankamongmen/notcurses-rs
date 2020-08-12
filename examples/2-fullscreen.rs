use notcurses::{NcError, NcLogLevel, NcOptionFlag as flag, NcOptions, NotCurses};
use enumflags2::RawBitFlags; // necessary for now

fn main() -> Result<(), NcError> {
    println!("example 2: fullscreen");

    // Different ways of creating NcOptions
    let _opts1 = NcOptions::new_default();
    let _opts2 = NcOptions::new(NcLogLevel::Silent, flag::empty()); // needs Trait RawBitFlags for now
    let _opts3 = NcOptions::new(NcLogLevel::Silent, flag::NoAlternateScreen);
    let _opts4 = NcOptions::new(
        NcLogLevel::Silent,
        flag::SuppressBanners | flag::NoAlternateScreen,
    );

    let mut nc1 = NotCurses::new(_opts4)?;

    let stdplane1 = nc1.stdplane();

    println!("dim_yx={:?}", stdplane1.dim_yx());
    println!("supported_styles: {:#b}", nc1.supported_styles());
    println!("supported_styles_str: {:?}", nc1.supported_styles_str());

    Ok(())
}
