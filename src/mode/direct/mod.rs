//! `NotcursesD` wrapper struct and traits implementations.

mod builder;
pub use builder::NotcursesDBuilder;

use crate::{
    ncresult, sys::NcDirect, Align, Blitter, Capabilities, Channels, NResult, Plane, Rgb, Scale,
    Style,
};

/// A minimal notcurses *direct mode* context for styling text.
#[derive(Debug)]
pub struct NotcursesD<'ncdirect> {
    pub(crate) ncdirect: &'ncdirect mut NcDirect,
}

impl<'ncdirect> Drop for NotcursesD<'ncdirect> {
    /// Destroys the NotcursesD context.
    fn drop(&mut self) {
        let _ = self.ncdirect.stop();
    }
}

impl<'ncdirect> NotcursesD<'ncdirect> {
    /// New `NotcursesD` instance.
    pub fn new() -> NResult<Self> {
        Ok(Self {
            ncdirect: NcDirect::new()?,
        })
    }

    /// Returns a [`NotcursesDBuilder`] used to customize a new
    /// `NotcursesD` instance.
    pub fn build() -> NotcursesDBuilder {
        NotcursesDBuilder::default()
    }

    /// Clears the screen.
    pub fn clear(&mut self) -> NResult<()> {
        ncresult![self.ncdirect.clear()]
    }

    /// Forces a flush.
    pub fn flush(&mut self) -> NResult<()> {
        ncresult![self.ncdirect.flush()]
    }

    /// Takes the result of [`render_frame`][NotcursesD#method.render_frame]
    /// and writes it to the output.
    pub fn raster_frame(&mut self, plane: &mut Plane, align: Align) -> NResult<()> {
        ncresult![self.ncdirect.raster_frame(plane.ncplane, align.into())]
    }

    /// Renders an image into a [`Plane`] using the specified [`Blitter`] and
    /// [`Scale`], but doesn't write the result.
    ///
    /// The image may be arbitrarily many rows -- the output will scroll --
    /// but will only occupy the column of the cursor, and those to the right.
    ///
    /// To actually write (and free) this, invoke
    /// [`raster_frame`][NotcursesD#method.raster_frame].
    ///
    /// `max_x' and 'max_y` (cell geometry, *not* pixel), if greater than 0,
    /// are used for scaling; the terminal's geometry is otherwise used.
    ///
    pub fn render_frame(
        &mut self,
        filename: &str,
        blitter: Blitter,
        scale: Scale,
        max_x: u32,
        max_y: u32,
    ) -> NResult<Plane> {
        let ncplane =
            self.ncdirect
                .render_frame(filename, blitter.into(), scale.into(), max_y, max_x)?;
        Ok(ncplane.into())
    }

    /// Renders an image using the specified [`Blitter`] and [`Scale`].
    ///
    /// The image may be arbitrarily many rows -- the output will scroll --
    /// but will only occupy the column of the cursor, and those to the right.
    ///
    /// The render/raster process can be split by using
    /// [`render_frame`][#method.render_frame] and
    /// [`raster_frame`][#method.raster_frame].
    ///
    pub fn render_image(
        &mut self,
        filename: &str,
        align: Align,
        blitter: Blitter,
        scale: Scale,
    ) -> NResult<()> {
        ncresult![self
            .ncdirect
            .render_image(filename, align.into(), blitter.into(), scale.into())]
    }

    /// Disables the terminal cursor, if supported.
    pub fn cursor_disable(&mut self) -> NResult<()> {
        ncresult![self.ncdirect.cursor_disable()]
    }

    /// Enables the terminal cursor, if supported.
    pub fn cursor_enable(&mut self) -> NResult<()> {
        ncresult![self.ncdirect.cursor_enable()]
    }

    /// Moves the cursor down any number of `rows`.
    pub fn cursor_down(&mut self, num: i32) -> NResult<()> {
        ncresult![self.ncdirect.cursor_down(num)]
    }

    /// Moves the cursor left any number of `rows`.
    pub fn cursor_left(&mut self, num: i32) -> NResult<()> {
        ncresult![self.ncdirect.cursor_left(num)]
    }

    /// Moves the cursor right any number of `rows`.
    pub fn cursor_right(&mut self, num: i32) -> NResult<()> {
        ncresult![self.ncdirect.cursor_right(num)]
    }

    /// Moves the cursor up any number of `rows`.
    pub fn cursor_up(&mut self, num: i32) -> NResult<()> {
        ncresult![self.ncdirect.cursor_up(num)]
    }

    /// Moves the cursor to the specified column `x`.
    pub fn cursor_set_x(&mut self, x: u32) -> NResult<()> {
        ncresult![self.ncdirect.cursor_set_x(x)]
    }

    /// Moves the cursor to the specified row `y`.
    pub fn cursor_set_y(&mut self, y: u32) -> NResult<()> {
        ncresult![self.ncdirect.cursor_set_y(y)]
    }

    /// Moves the cursor to the specified column `x`, row `y`.
    pub fn cursor_set_xy(&mut self, x: u32, y: u32) -> NResult<()> {
        ncresult![self.ncdirect.cursor_set_yx(y, x)]
    }

    /// Returns the cursor (x, y) position, when supported.
    pub fn cursor_xy(&mut self) -> NResult<(u32, u32)> {
        let (y, x) = self.ncdirect.cursor_yx()?;
        Ok((x, y))
    }

    /// Pushes the cursor location to the terminal's stack.
    ///
    /// The depth of this stack, and indeed its existence, is terminal-dependent.
    pub fn cursor_push(&mut self) -> NResult<()> {
        ncresult![self.ncdirect.cursor_push()]
    }

    /// Pops the cursor location from the terminal's stack.
    ///
    /// The depth of this stack, and indeed its existence, is terminal-dependent.
    pub fn cursor_pop(&mut self) -> NResult<()> {
        ncresult![self.ncdirect.cursor_pop()]
    }

    /// Returns the capabilities of the terminal.
    pub fn capabilities(&self) -> Capabilities {
        Capabilities {
            halfblock: self.ncdirect.canhalfblock(),
            quadrant: self.ncdirect.canquadrant(),
            sextant: self.ncdirect.cansextant(),
            braille: self.ncdirect.canbraille(),
            utf8: self.ncdirect.canutf8(),
            images: self.ncdirect.canopen_images(),
            videos: self.ncdirect.canopen_videos(),
            pixel: self.ncdirect.check_pixel_support().unwrap_or(false),
            truecolor: self.ncdirect.cantruecolor(),
            fade: self.ncdirect.canfade(),
            palette_change: self.ncdirect.canchangecolor(),
            palette_size: self.ncdirect.palette_size().unwrap_or(0),
            cursor: self.ncdirect.canget_cursor(),
        }
    }

    /// Returns the size of the terminal in columns and rows (x, y).
    pub fn term_size(&mut self) -> (u32, u32) {
        let (y, x) = self.ncdirect.dim_yx();
        (x, y)
    }

    /// Returns the name of the detected terminal.
    pub fn term_name(&self) -> String {
        self.ncdirect.detected_terminal()
    }

    /// Sets the background [`Rgb`].
    pub fn set_bg<RGB: Into<Rgb>>(&mut self, rgb: RGB) -> NResult<()> {
        ncresult![self.ncdirect.set_bg_rgb(rgb.into().into())]
    }

    /// Sets the foreground [`Rgb`].
    pub fn set_fg<RGB: Into<Rgb>>(&mut self, rgb: RGB) -> NResult<()> {
        ncresult![self.ncdirect.set_fg_rgb(rgb.into().into())]
    }

    /// Indicates to use the "default color" for the background .
    pub fn set_bg_default(&mut self) -> NResult<()> {
        ncresult![self.ncdirect.set_bg_default()]
    }

    /// Indicates to use the "default color" for the foreground .
    pub fn set_fg_default(&mut self) -> NResult<()> {
        ncresult![self.ncdirect.set_fg_default()]
    }

    // TODO: set_bg_palindex, set_fg_palindex

    // /// Sets the background [`PaletteIndex`].
    // pub fn set_bg_palindex(&mut self, index: PaletteIndex) -> NResult<()> {
    //     ncresult![self.ncdirect.set_bg_palindex(index)]
    // }
    //
    // /// Sets the foreground [`PaletteIndex`].
    // pub fn set_fg_palindex(&mut self, index: PaletteIndex) -> NResult<()> {
    //     ncresult![self.ncdirect.set_fg_palindex(index)]
    // }

    // MAYBE: palette_size

    /// Adds the specified [`Style`]s.
    pub fn add_styles(&mut self, styles: Style) -> NResult<()> {
        ncresult![self.ncdirect.styles_on(styles.bits())]
    }

    /// Deletes the specified [`Style`]s.
    pub fn del_styles(&mut self, styles: Style) -> NResult<()> {
        ncresult![self.ncdirect.styles_off(styles.bits())]
    }

    /// Sets just the specified [`Style`]s.
    pub fn set_styles(&mut self, styles: Style) -> NResult<()> {
        ncresult![self.ncdirect.styles_set(styles.bits())]
    }

    // TODO: getc, getc_nblock, getc_blocking, inputread_fd

    // /// Returns a [char] representing a single unicode point.
    // ///
    // /// If an event is processed, the return value is the `id` field from that
    // /// event.
    // ///
    // /// Provide a None `time` to block at length, a `time` of 0 for non-blocking
    // /// operation, and otherwise a timespec to bound blocking.
    // ///
    // /// Signals in sigmask (less several we handle internally) will be atomically
    // /// masked and unmasked per [ppoll(2)](https://linux.die.net/man/2/ppoll).
    // ///
    // /// `*sigmask` should generally contain all signals.
    // ///
    // /// *C style function: [ncdirect_getc()][crate::ncdirect_getc].*
    // //
    // // CHECK returns 0 on a timeout.
    // pub fn getc(
    //     &mut self,
    //     time: Option<NcTime>,
    //     sigmask: Option<&mut sigset_t>,
    //     input: Option<&mut NcInput>,
    // ) -> NcNResult<char> {
    //
    // }

    /// Outputs the `string` according to the `channels`, and
    /// returns the total number of characters written on success.
    ///
    /// Note that it does not explicitly flush output buffers, so it will not
    /// necessarily be immediately visible.
    ///
    /// It will fail if the NcDirect context and the foreground channel
    /// are both marked as using the default color.
    pub fn putstr(&mut self, channels: Channels, string: &str) -> NResult<()> {
        ncresult![self.ncdirect.putstr(channels.into(), string)]
    }

    /// Reads a (heap-allocated) line of text using the Readline library.
    ///
    /// Initializes Readline the first time it's called.
    ///
    // TODO
    // For input to be echoed to the terminal, it is necessary that the flag
    // [NCDIRECT_OPTION_INHIBIT_CBREAK][crate::NCDIRECT_OPTION_INHIBIT_CBREAK]
    // be provided to the constructor.
    pub fn readline(&mut self, prompt: &str) -> NResult<&str> {
        ncresult![self.ncdirect.readline(prompt)]
    }

    // TODO: r#box, double_box, rounded_box, hline_interp, vline_interp
}
