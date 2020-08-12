use notcurses::{NcError, NcDirect, NcScale, NcAlign, NcBlitter};

fn main() -> Result<(), NcError> {
    println!("example 1: direct mode");
    let mut ncd = NcDirect::new()?;
    // ncd.clear()?;
    // println!("example 1: direct mode (after clearing)");

    println!("y={}, x={}", ncd.dim_y(), ncd.dim_x());

    // capabilities
    println!("Can open images?: {}\nCan UTF-8?: {}",
        ncd.canopen_images(), ncd.canutf8());

    // cursor

    let yx = ncd.cursor_yx()?;
    println!("Cursor position: {:?}", yx);
    ncd.cursor_move_yx(200,100)?;
    ncd.cursor_move_yx(yx.0, yx.1)?;


    ncd.cursor_disable()?;
    ncd.cursor_enable()?;


    // image
    let ps = ncd.palette_size();
    println!("palette_size: {}", ps);

    //ncd.render_image("1-direct-image-16.png", NcAlign::Right, NcBlitter::_2x1, NcScale::None)?;
    ncd.render_image("1-direct-image-8.png", NcAlign::Right, NcBlitter::_1x1, NcScale::None)?;
    // ncd.render_image("1-direct-image.png", NcAlign::Left, NcBlitter::_1x1, NcScale::Stretch)?;
    // ncd.render_image("1-direct-image.png", NcAlign::Left, NcBlitter::_1x1, NcScale::Scale)?;
    // NOTE: changing blitter has no effect: https://github.com/dankamongmen/notcurses/issues/866


    // FIXME: restore the text format after rendering an image

    println!("palette_size: {}\n\n\n\n\n\n", ps);
    println!("Cursor position: {:?}", ncd.cursor_yx()?);

    Ok(())
}
