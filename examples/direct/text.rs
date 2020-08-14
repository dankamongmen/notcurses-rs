#![allow(unused_imports)]

use notcurses::{NcDirect, NcError, NcStyle};
use libnotcurses_sys as nc;

fn main() -> Result<(), NcError> {
    let mut ncd = NcDirect::new()?;


    // INFO

    println!("Terminal rows={0}, cols={1}", ncd.dim_y(), ncd.dim_x());

    println!("Can open images: {0}\nCan UTF-8: {1}",
        ncd.can_open_images(), ncd.can_utf8());

    println!("palette_size: {}", ncd.palette_size());


    // TEXT & STYLE

    let stylesv = vec![
        ("[DIM]", NcStyle::Dim),
        ("[UNDERLINE]", NcStyle::Underline),
        ("[ITALIC]", NcStyle::Italic),
        ("[BOLD]", NcStyle::Bold),
        ("[REVERSE]", NcStyle::Reverse),
        ("[BLINK]", NcStyle::Blink),
        ("[INVIS]", NcStyle::Invis),
        ("[PROTECT]", NcStyle::Protect),
        ("[STANDOUT]", NcStyle::Standout),
    ];

    ncd.putstr(0, "\nSingle styles:\n")?;

    ncd.putstr(0, "[DEFAULT]")?;
    for (label, style) in stylesv.iter() {
        ncd.styles_on(*style)?;
        ncd.putstr(0, label)?;
        ncd.styles_off(*style)?;
    }

    ncd.putstr(0, "\nJoint styles:\n")?;

    ncd.putstr(0, "[DEFAULT ")?;
    for (label, style) in stylesv.iter() {
        ncd.styles_on(*style)?;
        ncd.putstr(0, &label.chars().map(
            |c| match c { '[' | ']' => ' ', _ => c }).collect::<String>())?;
        if let NcStyle::Blink = style { break ; }
    }
    ncd.styles_off_all()?;
    ncd.putstr(0, "]")?;


    // TEXT mixing Rust's print!() & println!() and notcurses' putstr() & printf()
    //
    ncd.putstr(0, "\n\n1")?;
    println!("2 < instead of printing this concatenated AFTER, it appears BEFORE 1");

    ncd.putstr(0, "\n\n1 \n")?;
    println!("2 < it does work (better) with a `\\n` after 1");


    // TODO: more tests with styles_set & bold+italic
    //
    //ncd.styles_off(NcStyle::Bold)?;
    //ncd.styles_on(NcStyle::Italic)?;

    // COLORS & TEXT (WIP)

    ncd.bg(0x00FF00 as u32)?; // FIXME: colors don't seem to work
    ncd.fg(0xFF0000 as u32)?;
    println!("\nhello colors? (investigate)");
    ncd.putstr(nc::channels_combine(0xFF008800, 0xFFBB0099), "hello colors 2")?;
    ncd.putstr(0, "...")?;


    // WIP----------------------- ↓

    // CURSOR & TEXT

    // println!("Cursor position: {:?}", ncd.cursor_yx()?);
    // ncd.cursor_move_yx(200,100)?;
    // ncd.cursor_move_yx(yx.0, yx.1)?;
    // ncd.cursor_disable()?;
    // ncd.cursor_enable()?;

    // ncd.clear()?;

    Ok(())
}

