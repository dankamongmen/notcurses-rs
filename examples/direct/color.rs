#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

use notcurses::{Direct, Error};
use libnotcurses_sys as nc; // TEMP

fn main() -> Result<(), Error> {

    let mut rng = rand::thread_rng();

    let mut ncd = Direct::new()?;

    fill_screen(&mut ncd, " ░▒▓█")?;
    std::thread::sleep(std::time::Duration::new(2, 0));

    let (cols, rows) = (ncd.cols(), ncd.rows());
    for n in 0..(cols * rows * 3) {
        ncd.cursor_move_yx(rng.gen_range(0, rows), rng.gen_range(0, cols))?;
        ncd.print_colored(rngcolors(), &"▗▐ ▖▀▟▌▙██▇▆▅▄▃▂▁".chars().choose(&mut rng).unwrap().to_string())?;
    }
    ncd.cursor_move_yx(rows + 1, 1)?;

    Ok(())
}

/// Fill the screen with random characters from the ones provided
///
fn fill_screen(ncd: &mut Direct, s: &str) -> Result<(), Error> {

    let mut rng = rand::thread_rng();

    let (cols, rows) = (ncd.cols(), ncd.rows());
    let (mut col_count, mut row_count) = (0, 0);
    let (mut fcolor, mut bcolor, mut color) = (0, 0, 0);
    let (mut fr, mut fg, mut fb, mut br, mut bg, mut bb) = (0, 0, 0, 0, 0, 0);

    ncd.clear()?;

    for row in 0..=rows {

        // foreground -blue -green
        fb = modcolor(row_count, rows, false, 1.2);
        fg = modcolor(row_count, rows, false, 0.8);
        // background +blue
        bb = modcolor(row_count, rows, true, 0.3);

        col_count = 0;
        for col in 0..=cols {

            // foreground +red
            fr = modcolor(col_count, cols, false, 1.2);
            // background +red +green
            br = modcolor(col_count, cols, true, 1.);
            bg = modcolor(col_count, cols, true, 1.);

            nc::channel_set_rgb(&mut fcolor, fr, fg, fb);
            nc::channel_set_rgb(&mut bcolor, br, bg, bb);
            color = nc::channels_combine(fcolor, bcolor);

            ncd.cursor_move_yx(row, col)?;
            ncd.print_colored(color, &s.chars().choose(&mut rng).unwrap().to_string())?;

            col_count += 1;
        }
        row_count += 1;
    }
    Ok(())
}

/// Increases or decreases the value of color by a single step in a scale
///
#[inline]
fn modcolor(count: u16, total: i32, increase: bool, fraction: f32) -> u8 {
    let colorstep = 255. / total as f32;

    ((count as f32 * colorstep ) / fraction * increase as u8 as f32) as u8
    + ((255. - (count as f32 * colorstep)) / fraction * !increase as u8 as f32) as u8
}

/// Get a random color pair
#[inline]
fn rngcolors() -> u64 {
    let mut rng = rand::thread_rng();

    let (mut fcolor, mut bcolor) = (0, 0);
    nc::channel_set_rgb(&mut fcolor, rng.gen_range(0,255), rng.gen_range(0,255), rng.gen_range(0,255));
    nc::channel_set_rgb(&mut bcolor, rng.gen_range(0,255), rng.gen_range(0,255), rng.gen_range(0,255));
    nc::channels_combine(fcolor, bcolor)
}
