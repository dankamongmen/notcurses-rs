//! `NotcursesDirect` wrapper struct and traits implementations.

mod builder;
pub use builder::NotcursesDirectBuilder;

use crate::{
    ncresult, sys::NcDirect, Align, Blitter, Capabilities, Channels, Dimension, Offset, Plane,
    NotcursesResult as Result, Rgb, Scale, Style,
};

/// A minimal notcurses direct mode context for styling text.
#[derive(Debug)]
pub struct NotcursesDirect<'a> {
    pub(crate) raw: &'a mut NcDirect,
}

impl<'a> Drop for NotcursesDirect<'a> {
    /// Destroys the NotcursesDirect context.
    fn drop(&mut self) {
        let _ = self.raw.stop();
    }
}

impl<'a> NotcursesDirect<'a> {
    /// New `NotcursesDirect` instance.
    pub fn new() -> Result<Self> {
        Ok(Self {
            raw: NcDirect::new()?,
        })
    }

    /// Returns a [`NotcursesDirectBuilder`] used to customize a new
    /// `NotcursesDirect` instance.
    pub fn build() -> NotcursesDirectBuilder {
        NotcursesDirectBuilder::default()
    }

    /// Clears the screen.
    pub fn clear(&mut self) -> Result<()> {
        ncresult![self.raw.clear()]
    }

    /// Forces a flush.
    pub fn flush(&mut self) -> Result<()> {
        ncresult![self.raw.flush()]
    }

    /// Takes the result of [`render_frame`][NotcursesDirect#method.render_frame]
    /// and writes it to the output.
    pub fn raster_frame(&mut self, plane: &mut Plane, align: Align) -> Result<()> {
        ncresult![self.raw.raster_frame(plane.raw, align.into())]
    }

    /// Renders an image into a [`Plane`] using the specified [`Blitter`] and
    /// [`Scale`], but doesn't write the result.
    ///
    /// The image may be arbitrarily many rows -- the output will scroll --
    /// but will only occupy the column of the cursor, and those to the right.
    ///
    /// To actually write (and free) this, invoke
    /// [`raster_frame`][NotcursesDirect#method.raster_frame].
    ///
    /// `max_x' and 'max_y` (cell geometry, *not* pixel), if greater than 0,
    /// are used for scaling; the terminal's geometry is otherwise used.
    ///
    pub fn render_frame(
        &mut self,
        filename: &str,
        blitter: Blitter,
        scale: Scale,
        max_x: Dimension,
        max_y: Dimension,
    ) -> Result<Plane> {
        let p = self
            .raw
            .render_frame(filename, blitter.into(), scale.into(), max_y, max_x)?;
        Ok(Plane::from_ncplane(p))
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
    ) -> Result<()> {
        ncresult![self
            .raw
            .render_image(filename, align.into(), blitter.into(), scale.into())]
    }

    /// Disables the terminal cursor, if supported.
    pub fn cursor_disable(&mut self) -> Result<()> {
        ncresult![self.raw.cursor_disable()]
    }

    /// Enables the terminal cursor, if supported.
    pub fn cursor_enable(&mut self) -> Result<()> {
        ncresult![self.raw.cursor_enable()]
    }

    /// Moves the cursor down any number of `rows`.
    pub fn cursor_down(&mut self, num: Offset) -> Result<()> {
        ncresult![self.raw.cursor_down(num)]
    }

    /// Moves the cursor left any number of `rows`.
    pub fn cursor_left(&mut self, num: Offset) -> Result<()> {
        ncresult![self.raw.cursor_left(num)]
    }

    /// Moves the cursor right any number of `rows`.
    pub fn cursor_right(&mut self, num: Offset) -> Result<()> {
        ncresult![self.raw.cursor_right(num)]
    }

    /// Moves the cursor up any number of `rows`.
    pub fn cursor_up(&mut self, num: Offset) -> Result<()> {
        ncresult![self.raw.cursor_up(num)]
    }

    /// Moves the cursor to the specified column `x`.
    pub fn cursor_set_x(&mut self, x: Dimension) -> Result<()> {
        ncresult![self.raw.cursor_set_x(x)]
    }

    /// Moves the cursor to the specified row `y`.
    pub fn cursor_set_y(&mut self, y: Dimension) -> Result<()> {
        ncresult![self.raw.cursor_set_y(y)]
    }

    /// Moves the cursor to the specified column `x`, row `y`.
    pub fn cursor_set_xy(&mut self, x: Dimension, y: Dimension) -> Result<()> {
        ncresult![self.raw.cursor_set_yx(y, x)]
    }

    /// Returns the cursor (x, y) position, when supported.
    pub fn cursor_xy(&mut self) -> Result<(Dimension, Dimension)> {
        let (y, x) = self.raw.cursor_yx()?;
        Ok((x, y))
    }

    /// Pushes the cursor location to the terminal's stack.
    ///
    /// The depth of this stack, and indeed its existence, is terminal-dependent.
    pub fn cursor_push(&mut self) -> Result<()> {
        ncresult![self.raw.cursor_push()]
    }

    /// Pops the cursor location from the terminal's stack.
    ///
    /// The depth of this stack, and indeed its existence, is terminal-dependent.
    pub fn cursor_pop(&mut self) -> Result<()> {
        ncresult![self.raw.cursor_pop()]
    }

    /// Returns the capabilities of the terminal.
    pub fn capabilities(&self) -> Capabilities {
        Capabilities {
            halfblock: self.raw.canhalfblock(),
            quadrant: self.raw.canquadrant(),
            sextant: self.raw.cansextant(),
            braille: self.raw.canbraille(),
            utf8: self.raw.canutf8(),
            images: self.raw.canopen_images(),
            videos: self.raw.canopen_videos(),
            pixel: self.raw.check_pixel_support().unwrap_or(false),
            truecolor: self.raw.cantruecolor(),
            fade: self.raw.canfade(),
            palette_change: self.raw.canchangecolor(),
            palette_size: self.raw.palette_size().unwrap_or(0),
            cursor: self.raw.canget_cursor(),
        }
    }

    /// Returns the size of the terminal in columns and rows (x, y).
    pub fn term_size(&mut self) -> (Dimension, Dimension) {
        let (y, x) = self.raw.dim_yx();
        (x, y)
    }

    /// Returns the name of the detected terminal.
    pub fn term_name(&self) -> String {
        self.raw.detected_terminal()
    }

    /// Sets the background [`Rgb`].
    pub fn set_bg<RGB: Into<Rgb>>(&mut self, rgb: RGB) -> Result<()> {
        ncresult![self.raw.set_bg_rgb(rgb.into().into())]
    }

    /// Sets the foreground [`Rgb`].
    pub fn set_fg<RGB: Into<Rgb>>(&mut self, rgb: RGB) -> Result<()> {
        ncresult![self.raw.set_fg_rgb(rgb.into().into())]
    }

    /// Indicates to use the "default color" for the background .
    pub fn set_bg_default(&mut self) -> Result<()> {
        ncresult![self.raw.set_bg_default()]
    }

    /// Indicates to use the "default color" for the foreground .
    pub fn set_fg_default(&mut self) -> Result<()> {
        ncresult![self.raw.set_fg_default()]
    }

    // TODO: set_bg_palindex, set_fg_palindex

    // /// Sets the background [`PaletteIndex`].
    // pub fn set_bg_palindex(&mut self, index: PaletteIndex) -> Result<()> {
    //     ncresult![self.raw.set_bg_palindex(index)]
    // }
    //
    // /// Sets the foreground [`PaletteIndex`].
    // pub fn set_fg_palindex(&mut self, index: PaletteIndex) -> Result<()> {
    //     ncresult![self.raw.set_fg_palindex(index)]
    // }

    // MAYBE: palette_size

    /// Adds the specified [`Style`]s.
    pub fn add_styles(&mut self, styles: Style) -> Result<()> {
        ncresult![self.raw.styles_on(styles.bits())]
    }

    /// Deletes the specified [`Style`]s.
    pub fn del_styles(&mut self, styles: Style) -> Result<()> {
        ncresult![self.raw.styles_off(styles.bits())]
    }

    /// Sets just the specified [`Style`]s.
    pub fn set_styles(&mut self, styles: Style) -> Result<()> {
        ncresult![self.raw.styles_set(styles.bits())]
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
    // ) -> NcResult<char> {
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
    pub fn putstr(&mut self, channels: Channels, string: &str) -> Result<()> {
        ncresult![self.raw.putstr(channels.into(), string)]
    }

    /// Reads a (heap-allocated) line of text using the Readline library.
    ///
    /// Initializes Readline the first time it's called.
    ///
    // TODO: DirectBuilder
    // For input to be echoed to the terminal, it is necessary that the flag
    // [NCDIRECT_OPTION_INHIBIT_CBREAK][crate::NCDIRECT_OPTION_INHIBIT_CBREAK]
    // be provided to the constructor.
    pub fn readline(&mut self, prompt: &str) -> Result<&str> {
        ncresult![self.raw.readline(prompt)]
    }

    // TODO: r#box, double_box, rounded_box, hline_interp, vline_interp
}
