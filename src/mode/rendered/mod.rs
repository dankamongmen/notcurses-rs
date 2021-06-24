//! `Notcurses` wrapper struct and traits implementations.

use crate::{ncresult, sys::Nc, Capabilities, NResult, PixelGeometry};

mod builder;
mod loglevel;
pub use builder::NotcursesBuilder;
pub use loglevel::LogLevel;

/// The main **notcurses** *rendered mode* context.
#[derive(Debug)]
pub struct Notcurses<'nc> {
    pub(crate) raw: &'nc mut Nc,
}

impl<'nc> Drop for Notcurses<'nc> {
    /// Destroys the Notcurses context.
    fn drop(&mut self) {
        let _ = self.raw.stop();
    }
}

impl<'nc> Notcurses<'nc> {
    /// New `Notcurses` instance.
    pub fn new() -> NResult<Self> {
        Ok(Self { raw: Nc::new()? })
    }

    /// Returns a [`NotcursesBuilder`] used to customize a new
    /// `Notcurses` instance.
    pub fn build() -> NotcursesBuilder {
        NotcursesBuilder::default()
    }

    /// Returns a reference to the inner [`Nc`].
    pub fn as_nc(&self) -> &Nc {
        self.raw
    }

    /// Returns a mutable reference to the inner [`Nc`].
    pub fn as_nc_mut(&mut self) -> &mut Nc {
        self.raw
    }

    // pub fn align
    // pub fn at_yx

    /// Disables the terminal cursor, if supported.
    pub fn cursor_disable(&mut self) -> NResult<()> {
        ncresult![self.raw.cursor_disable()]
    }

    /// Enables the terminal cursor, if supported, plaxing it at `x`,`y`.
    pub fn cursor_enable(&mut self, x: u32, y: u32) -> NResult<()> {
        ncresult![self.raw.cursor_enable(y, x)]
    }

    // debug
    // debug_caps

    /// Destroys all [`Plane`][crate::Plane]s.
    ///
    /// Any pre-existing `Planes` will be invalid and shouldn't be used again.
    pub fn drop_planes(&mut self) {
        self.raw.drop_planes();
    }

    // TODO:
    // getc
    // getc_nblock
    // getc_blocking
    // inputready_fd

    // lex_blitter
    // lex_margins
    // lex_scalemode

    /// Disables signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z). They are enabled by default.
    pub fn linesigs_disable(&mut self) -> NResult<()> {
        ncresult![self.raw.linesigs_disable()]
    }

    /// Restores signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z), if disabled.
    pub fn linesigs_enable(&mut self) -> NResult<()> {
        ncresult![self.raw.linesigs_enable()]
    }

    /// Disables mouse events.
    ///
    /// Any events in the input queue can still be delivered.
    pub fn mouse_disable(&mut self) -> NResult<()> {
        ncresult![self.raw.mouse_disable()]
    }

    /// Enable the mouse in "button-event tracking" mode with focus detection
    /// and UTF8-style extended coordinates.
    ///
    /// On success, mouse events will be published to
    /// [getc()][Notcurses#method.getc].
    pub fn mouse_enable(&mut self) -> NResult<()> {
        ncresult![self.raw.mouse_enable()]
    }

    /// Refreshes the physical screen to match what was last rendered (i.e.,
    /// without reflecting any changes since the last call to
    /// [render][crate::Notcurses#method.render]).
    ///
    /// Returns the current terminal size (`x`, `y`).
    ///
    /// This is primarily useful if the screen is externally corrupted, or if an
    /// [NCKEY_RESIZE][crate::sys::NCKEY_RESIZE] event has been read and you're not
    /// yet ready to render.
    // TODO: sys::NCKEY_RESIZE reference
    pub fn refresh(&mut self) -> NResult<(u32, u32)> {
        let (y, x) = self.raw.refresh()?;
        Ok((x, y))
    }

    // stats
    // stats_aloc
    // stats_reset

    // str_blitter
    // str_scalemode

    // TODO:
    // supported_styles

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
            cursor: true,
        }
    }

    /// Returns the size of the terminal in columns and rows (x, y).
    pub fn term_size(&self) -> (u32, u32) {
        let (h, w) = self.raw.term_dim_yx();
        (w, h)
    }

    /// Returns the `PixelGeometry` of the terminal.
    pub fn term_pixelgeometry(&self) -> PixelGeometry {
        self.raw.stdplane_const().pixelgeom()
    }

    /// Returns the name of the detected terminal.
    pub fn term_name(&self) -> String {
        self.raw.detected_terminal()
    }

    /// Returns a human-readable string describing the running notcurses version.
    pub fn version() -> String {
        Nc::version()
    }

    /// Returns the running Notcurses version components
    /// (major, minor, patch, tweak).
    pub fn version_components() -> (u32, u32, u32, u32) {
        Nc::version_components()
    }
}
