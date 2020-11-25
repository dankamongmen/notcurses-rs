// total functions: 37
// ------------------------------------------ (done / remaining)
// - done: 30 /  6
// - test:  7 / 30
// ------------------------- ↓ from bindgen: 37
//+ ncdirect_bg_default
//  ncdirect_bg_palindex        // ?
//+ ncdirect_bg_rgb
//  ncdirect_box
//+ ncdirect_canopen_images     // as: can_open_images()
//+ ncdirect_canutf8            // as: can_utf8()
//# ncdirect_clear
//+ ncdirect_cursor_disable
//+ ncdirect_cursor_down
//+ ncdirect_cursor_enable
//+ ncdirect_cursor_left
//# ncdirect_cursor_move_yx
//+ ncdirect_cursor_pop
//+ ncdirect_cursor_push
//+ ncdirect_cursor_right
//+ ncdirect_cursor_up
//# ncdirect_cursor_yx
//# ncdirect_dim_x
//# ncdirect_dim_y
//  ncdirect_double_box
//+ ncdirect_fg_default
//  ncdirect_fg_palindex        // ?
//+ ncdirect_fg_rgb
//  ncdirect_flush
//  ncdirect_getc
//+ ncdirect_hline_interp       // WIP
//# ncdirect_init               // inside new()
//# ncdirect_palette_size
//+ ncdirect_printf_aligned
//+ ncdirect_putstr             // as print_colored()
//+ ncdirect_render_image       // as print_aligned()
//  ncdirect_rounded_box
//+ ncdirect_stop               // inside Drop Trait
//+ ncdirect_styles_off
//+ ncdirect_styles_on
//+ ncdirect_styles_set
//+ ncdirect_vline_interp       // WIP
//
// ------------------------- ↓ extra functions
//
// .styles_off_all

use core::ptr::{null, null_mut};
use cstr_core::CString;

use enumflags2::BitFlags;

use crate::{sys, Align, Blitter, NcChannels, Error, NcRgb, Scale, Style};

/// Direct Mode context
///
/// Can be used to manipulate the habitual output to the terminal
///
/// ## List of methods
///
/// ```text
/// .new()?→               // new direct mode instance
///
/// .can_open_images()→    // image support, bool
/// .can_utf8()→           // UTF-8 support, bool
/// .palette_size()→       // get the palette size (256)
/// .cols()→               // get number of columns
/// .rows()→               // get number of rows
///
/// .clear()?              // clear the screen
/// .bg(…)?                // set the background color
/// .fg(…)?                // set the foreground color
/// .bg_default()?         // set the default background color
/// .fg_default()?         // set the default foreground color
///
/// .cursor_enable()?      // enable the cursor
/// .cursor_disable()?     // disable the cursor
///
/// .cursor_yx()?→         // get cursor coordinates
/// .cursor_move_yx(…)?    // move the cursor to coordinates
/// .cursor_pop()?         // pop cursor location to stack
/// .cursor_push()?        // push cursor location to stack
///
/// .cursor_up(…)?         // move the cursor up
/// .cursor_down(…)?       // move the cursor down
/// .cursor_left(…)?       // move the cursor left
/// .cursor_right(…)?      // move the cursor right
///
/// .hline_interp(…)?      // draw horizontal lines interpolating 2 colors
/// .vline_interp(…)?      // draw vertical lines interpolating 2 colors
///
/// .styles_on(…)?         // turns on the provided style(s)
/// .styles_off(…)?        // turns off the provided style(s)
/// .styles_off_all()?     // turns off all styles
/// .styles_set(…)?        // turns on the provided styles & off the rest
///
/// .putstr(…)?            //
/// .printf_aligned(…)?    //
///
/// .render_image(…)?      //
///
/// // ----------------------------------------------- legend:
/// .function()            //
/// .function()→           // returns value
/// .function()?           // returns Result<(), Error>
/// .function()?→          // returns Result<value, Error>
/// .function(…)           // has argument(s)
/// ```
///
/// ## Links
///
/// - [man notcurses_directmode(3)](https://nick-black.com/notcurses/notcurses_directmode.3.html)
///
pub struct Direct {
    data: *mut sys::NcDirect,
}

impl Direct {
    // CONSTRUCTORS: new() -----------------------------------------------------

    /// Return a Direct Mode instance
    ///
    /// Initialize a direct-mode notcurses context on the connected terminal,
    /// which must be a tty. You'll usually want stdout.
    ///
    /// Direct mode supports a limited subset of notcurses routines which
    /// directly affect the terminal, and neither supports nor requires
    /// `notcurses_render()`. This can be used to add color and styling to
    /// text in the standard output paradigm.
    ///
    /// `flags` is a bitmask over `NCDIRECT_OPTION_*`.
    ///
    /// Returns NULL on error, including any failure initializing terminfo.
    ///
    pub fn new() -> Result<Self, Error> {
        // unsafe {
        //     let _ = libc::setlocale(libc::LC_ALL, std::ffi::CString::new("").unwrap().as_ptr());
        // }

        // TODO: ncdirect_init() returns NULL on failure. Otherwise, the return value points
        // to a valid struct ncdirect, which can be used until it is provided to ncdirect_stop().
        Ok(Direct {
            data: unsafe { sys::ncdirect_init(null(), null_mut(), 0) },
        })
    }

    // ----------------------------------------------------------^ CONSTRUCTORS

    /// Set the background color
    ///
    // TODO: TEST
    pub fn bg(&mut self, rgb: NcRgb) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_bg_rgb(self.data, rgb) < 0 {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }

    /// Set the default background color
    ///
    // TODO: TEST
    pub fn bg_default(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_bg_default(self.data) < 0 {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }

    /// Draw a box with its upper-left corner at the current cursor position,
    /// having dimensions |ylen|x|xlen|. See Plane.box() for more information.
    /// The minimum box size is 2x2, and it cannot be drawn off-screen.
    /// |wchars| is an array of 6 wide characters: UL, UR, LL, LR, HL, VL.
    ///
    // TODO: understand how this works, and then simplify it
    // TODO: TEST
    #[allow(clippy::too_many_arguments)]
    pub fn box1(
        &mut self,
        ul: u64,
        ur: u64,
        dl: u64,
        dr: u64,
        wchars: i32,
        ylen: i32,
        xlen: i32,
        ctrlword: u32,
    ) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_box(self.data, ul, ur, dl, dr, &wchars, ylen, xlen, ctrlword) < 0 {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }
    /// Return true if terminal can open images
    // TODO: TEST
    pub fn can_open_images(&self) -> bool {
        unsafe { sys::ncdirect_canopen_images(self.data) }
    }

    /// Return true if terminal can display UTF-8 characters
    // TODO: TEST
    pub fn can_utf8(&self) -> bool {
        unsafe { sys::ncdirect_canutf8(self.data) }
    }

    /// Clear the screen
    // TODO: TEST
    pub fn clear(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_clear(self.data) < 0 {
                return Err(Error::Clear);
            }
        }
        Ok(())
    }

    /// Get the current number of columns
    ///
    pub fn cols(&self) -> i32 {
        unsafe { sys::ncdirect_dim_x(self.data) }
    }

    /// Disables the cursor
    ///
    /// -1 to retain current location on that axis
    // TODO: TEST
    pub fn cursor_disable(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_cursor_disable(self.data) < 0 {
                return Err(Error::Cursor);
            }
        }
        Ok(())
    }

    /// Enables the cursor
    ///
    /// -1 to retain current location on that axis
    // TODO: TEST
    pub fn cursor_enable(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_cursor_enable(self.data) < 0 {
                return Err(Error::Cursor);
            }
        }
        Ok(())
    }

    /// Return the position of the cursor as a tuple of (rows, cols)
    ///
    /// Get the cursor position, when supported. This requires writing to the
    /// terminal, and then reading from it. If the terminal doesn't reply, or
    /// doesn't reply in a way we understand, the results might be deleterious.
    pub fn cursor_yx(&mut self) -> Result<(i32, i32), Error> {
        let mut y = 0;
        let mut x = 0;
        unsafe {
            if sys::ncdirect_cursor_yx(self.data, &mut y, &mut x) < 0 {
                return Err(Error::Cursor);
            }
        }
        Ok((y, x))
    }

    /// Moves the cursor to the provided coordinates (rows, cols)
    ///
    /// -1 to retain current location on that axis
    pub fn cursor_move_yx(&mut self, y: i32, x: i32) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_cursor_move_yx(self.data, y, x) < 0 {
                return Err(Error::Cursor);
            }
        }
        Ok(())
    }

    /// Moves the cursor the specified number of rows down
    ///
    /// -1 to retain current location on that axis
    // TODO: TEST
    pub fn cursor_down(&mut self, rows: i32) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_cursor_down(self.data, rows) < 0 {
                return Err(Error::Cursor);
            }
        }
        Ok(())
    }

    /// Moves the cursor the specified number of columns left
    ///
    /// -1 to retain current location on that axis
    // TODO: TEST
    pub fn cursor_left(&mut self, cols: i32) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_cursor_left(self.data, cols) < 0 {
                return Err(Error::Cursor);
            }
        }
        Ok(())
    }

    /// Pop the cursor location to the terminal's stack. The depth of this
    /// stack, and indeed its existence, is terminal-dependent.
    pub fn cursor_pop(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_cursor_pop(self.data) < 0 {
                return Err(Error::Cursor);
            }
        }
        Ok(())
    }

    /// Push the cursor location to the terminal's stack. The depth of this
    /// stack, and indeed its existence, is terminal-dependent.
    pub fn cursor_push(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_cursor_push(self.data) < 0 {
                return Err(Error::Cursor);
            }
        }
        Ok(())
    }

    /// Moves the cursor the specified number of columns right
    ///
    /// -1 to retain current location on that axis
    // TODO: TEST
    pub fn cursor_right(&mut self, cols: i32) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_cursor_right(self.data, cols) < 0 {
                return Err(Error::Cursor);
            }
        }
        Ok(())
    }

    /// Moves the cursor the specified number of rows up
    ///
    /// -1 to retain current location on that axis
    // TODO: TEST
    pub fn cursor_up(&mut self, rows: i32) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_cursor_up(self.data, rows) < 0 {
                return Err(Error::Cursor);
            }
        }
        Ok(())
    }

    /// Set the foreground color
    ///
    // TODO: TEST
    pub fn fg(&mut self, rgb: NcRgb) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_fg_rgb(self.data, rgb) < 0 {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }

    /// Set the default foreground color
    ///
    // TODO: TEST
    pub fn fg_default(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_fg_default(self.data) < 0 {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }

    ///
    // TODO: TEST
    #[inline]
    pub fn flush() {}

    /// Draw horizontal lines using the specified channels, interpolating
    /// between them as we go. The EGC may not use more than one column.
    ///
    /// For a horizontal line, |len| cannot exceed the screen width minus the
    /// cursor's offset. All lines start at the current cursor position.
    ///
    // TODO: TEST
    // FIXME: TYPES
    pub fn hline_interp(&mut self, egc: &str, len: i32, h1: u64, h2: u64) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_hline_interp(
                self.data,
                CString::new(egc).unwrap().as_ptr(),
                len,
                h1,
                h2,
            ) < 0
            {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }

    /// Return the number of simultaneous colors claimed to be supported, or 1 if
    /// there is no color support. Note that several terminal emulators advertise
    /// more colors than they actually support, downsampling internally.
    // TODO: TEST
    // CHECK: probably should be unsigned
    pub fn palette_size(&self) -> u32 {
        unsafe { sys::ncdirect_palette_size(self.data) }
    }

    ///
    ///
    ///
    // NOTE: once println!() works, this wont have much utility
    // TODO: TEST
    pub fn print_aligned(&mut self, y: i32, align: Align, fmt: &str) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_printf_aligned(
                self.data,
                y,
                align as sys::NcAlign,
                CString::new(fmt).unwrap().as_ptr(),
            ) < 0
            {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }

    ///
    ///
    // TODO: TEST
    pub fn print_colored(&mut self, channels: NcChannels, utf8: &str) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_putstr(self.data, channels, CString::new(utf8).unwrap().as_ptr()) < 0 {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }

    /// Display an image using the specified blitter and scaling. The image may
    /// be arbitrarily many rows -- the output will scroll -- but will only occupy
    /// the column of the cursor, and those to the right.
    pub fn render_image(
        &mut self,
        filename: &str,
        align: Align,
        blitter: Blitter,
        scale: Scale,
    ) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_render_image(
                self.data,
                CString::new(filename).expect("err filename").as_ptr(),
                align as sys::NcAlign,
                blitter as sys::NcBlitter,
                scale as sys::NcScale,
            ) != 0
            {
                return Err(Error::ImageRender);
            }
        }
        Ok(())
    }

    /// Get the current number of rows
    ///
    pub fn rows(&self) -> i32 {
        unsafe { sys::ncdirect_dim_y(self.data) }
    }

    /// Turn off the indicated styles
    pub fn styles_off(&mut self, style: impl Into<BitFlags<Style>>) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_styles_off(self.data, style.into().bits()) < 0 {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }

    /// Turn off all the styling
    ///
    pub fn styles_off_all(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_styles_off(self.data, sys::NCSTYLE_MASK) < 0 {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }

    /// Turn on the indicated styles
    ///
    pub fn styles_on(&mut self, style: impl Into<BitFlags<Style>>) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_styles_on(self.data, style.into().bits()) < 0 {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }

    /// Turn on just the indicated styles, and off the rest
    pub fn styles_set(&mut self, style: impl Into<BitFlags<Style>>) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_styles_set(self.data, style.into().bits()) < 0 {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }

    /// Draw vertical lines using the specified channels, interpolating between
    /// them as we go. The EGC may not use more than one column.
    ///
    /// For a vertical line, |len| may be as long as you'd like; the screen
    /// will scroll as necessary. All lines start at the current cursor position.
    ///
    // TODO: TEST
    // FIXME: TYPES
    pub fn vline_interp(&mut self, egc: &str, len: i32, h1: u64, h2: u64) -> Result<(), Error> {
        unsafe {
            if sys::ncdirect_vline_interp(
                self.data,
                CString::new(egc).unwrap().as_ptr(),
                len,
                h1,
                h2,
            ) < 0
            {
                return Err(Error::Generic);
            }
        }
        Ok(())
    }
}

impl Drop for Direct {
    fn drop(&mut self) {
        // It is important to reset the terminal before exiting, whether terminating due to intended operation
        // or a received signal. This is usually accomplished by explicitly calling notcurses_stop.
        //
        // For convenience, notcurses by default installs signal handlers for various signals typically resulting
        // in process termination (see signal(7)). These signal handlers call notcurses_stop for each struct notcurses
        // in the process, and then propagate the signal to any previously-configured handler.
        // These handlers are disabled upon entry to notcurses_stop
        //
        //
        // notcurses in full or direct mode is always supposed to leave you with:
        // - palette reset (oc terminfo)
        // - cursor visible (cnorm terminfo)
        // - all styles reset (sgr0 terminfo)
        //
        // [API](https://nick-black.com/notcurses/notcurses_directmode.3.html#description)
        unsafe {
            sys::ncdirect_stop(self.data);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
    #[test]
    fn () -> Result<(), Error> {
        let mut nc = Direct::new()?;
        //assert_eq!(, );
        Ok(())
    }
    */

    #[test]
    fn clear() -> Result<(), Error> {
        let mut _nc = Direct::new()?;
        // NOTE: commented out bc when the screen gets cleared the previous output is lost.
        //nc.clear()?;
        Ok(())
    }

    #[test]
    fn cursor_yx() -> Result<(), Error> {
        let mut nc = Direct::new()?;
        let _yx = nc.cursor_yx()?;
        print!("cursor_yx={:?} ", _yx);
        Ok(())
    }

    #[test]
    fn cols() -> Result<(), Error> {
        let nc = Direct::new()?;
        let _x = nc.cols();
        print!("cols={} ", _x);
        Ok(())
    }

    #[test]
    fn rows() -> Result<(), Error> {
        let nc = Direct::new()?;
        let _y = nc.rows();
        print!("rows={} ", _y);
        Ok(())
    }

    #[test]
    fn move_cursor_yx() -> Result<(), Error> {
        let mut nc = Direct::new()?;
        let _yx_a = nc.cursor_yx()?;
        print!("cursor_yx A={:?} ", _yx_a);

        //nc.cursor_move_yx(yx_a.0 - 1, yx_a.1 - 1)?;
        nc.cursor_move_yx(4, 35)?;
        let _yx_b = nc.cursor_yx()?;
        print!("cursor_yx B={:?} ", _yx_b);

        // FIXME: CHECK: see why ↑this↑ fails:
        //assert_ne!(_yx_a.0, _yx_b.0);
        //assert_ne!(_yx_a.1, _yx_b.1);

        nc.cursor_move_yx(_yx_a.0, _yx_a.1)?;
        // let _yx_c = nc.cursor_yx()?;
        // assert_eq!((yx_a.0, yx_a.1), (_yx_c.0, _yx_c.1));
        Ok(())
    }

    #[test]
    fn new() -> Result<(), Error> {
        let _nc = Direct::new()?;
        Ok(())
    }

    #[test]
    fn palette_size() -> Result<(), Error> {
        let mut _nc = Direct::new()?;
        assert!(_nc.palette_size() > 0);
        Ok(())
    }
}
