use notcurses::{NcAlign, NcBlitter, NcDirect, NcError, NcScale};

fn main() -> Result<(), NcError> {
    let mut ncd = NcDirect::new()?;

    if !ncd.canopen_images() {
        println!("Sorry, your terminal doesn't support images.");
        std::process::exit(1);
    }

    let img = examples_path("direct-image.png");

    // Alignment
    ncd.render_image(&img, NcAlign::Left, NcBlitter::_1x1, NcScale::None)?;
    ncd.render_image(&img, NcAlign::Center, NcBlitter::_1x1, NcScale::None)?;
    ncd.render_image(&img, NcAlign::Right, NcBlitter::_1x1, NcScale::None)?;

    // Scale
    ncd.render_image(&img, NcAlign::Center, NcBlitter::_1x1, NcScale::Stretch)?;
    ncd.render_image(&img, NcAlign::Center, NcBlitter::_1x1, NcScale::Scale)?;

    // Blitter
    //
    // NOTE: NcBlitter::_1x1x4 & NcBlitter::_4x1 are still unimplemented,
    // they both ought be falling back to 1x1 with a top half.
    ncd.render_image(&img, NcAlign::Center, NcBlitter::_1x1, NcScale::None)?;
    ncd.render_image(&img, NcAlign::Center, NcBlitter::_1x1x4, NcScale::None)?;
    ncd.render_image(&img, NcAlign::Center, NcBlitter::_2x1, NcScale::None)?;
    ncd.render_image(&img, NcAlign::Center, NcBlitter::_2x2, NcScale::None)?;
    ncd.render_image(&img, NcAlign::Center, NcBlitter::_4x1, NcScale::None)?;
    ncd.render_image(&img, NcAlign::Center, NcBlitter::_8x1, NcScale::None)?;
    ncd.render_image(&img, NcAlign::Center, NcBlitter::Braille, NcScale::None)?;

    Ok(())
}


/// Return an absolute path to a file, relative to the examples folder
/// 
/// file = the relative path to the file (from the examples folder)
fn examples_path(file: &str) -> String {
    let mut dir = std::env::current_exe().unwrap().to_path_buf();
    for _ in 0..4 {
        dir.pop();
    }
    dir.push("examples");
    dir.push(file);
    format!("{}", dir.display())
}

