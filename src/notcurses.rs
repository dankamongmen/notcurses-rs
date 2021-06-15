//! `Notcurses` wrapper struct and traits implementations.

use crate::{ncresult, sys::Nc, Capabilities, Dimension, Result};

/// The main **notcurses** context.
///
/// *A  wrapper around `sys::`[`Nc`].*
#[derive(Debug)]
pub struct Notcurses<'a> {
    pub(crate) raw: &'a mut Nc,
}

impl<'a> Drop for Notcurses<'a> {
    /// Destroys the Notcurses context.
    fn drop(&mut self) {
        let _ = self.raw.stop();
    }
}

impl<'a> Notcurses<'a> {
    /// New Notcurses instance.
    pub fn new() -> Result<Self> {
        Ok(Self { raw: Nc::new()? })
    }

    /// New Notcurses instance, without an alternate screen.
    pub fn without_altscreen() -> Result<Self> {
        Ok(Self {
            raw: Nc::without_altscreen()?,
        })
    }

    // pub fn align
    // pub fn at_yx

    /// Disables the terminal cursor, if supported.
    pub fn cursor_disable(&mut self) -> Result<()> {
        ncresult![self.raw.cursor_disable()]
    }

    /// Enables the terminal cursor, if supported, plaxing it at `x`,`y`.
    pub fn cursor_enable(&mut self, x: Dimension, y: Dimension) -> Result<()> {
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
    pub fn linesigs_disable(&mut self) -> Result<()> {
        ncresult![self.raw.linesigs_disable()]
    }

    /// Restores signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z), if disabled.
    pub fn linesigs_enable(&mut self) -> Result<()> {
        ncresult![self.raw.linesigs_enable()]
    }

    /// Disables mouse events.
    ///
    /// Any events in the input queue can still be delivered.
    pub fn mouse_disable(&mut self) -> Result<()> {
        ncresult![self.raw.mouse_disable()]
    }

    /// Enable the mouse in "button-event tracking" mode with focus detection
    /// and UTF8-style extended coordinates.
    ///
    /// On success, mouse events will be published to
    /// [getc()][Notcurses#method.getc].
    pub fn mouse_enable(&mut self) -> Result<()> {
        ncresult![self.raw.mouse_enable()]
    }

    /// Refreshes the physical screen to match what was last rendered (i.e.,
    /// without reflecting any changes since the last call to
    /// [render][crate::Notcurses#method.render]).
    ///
    /// Returns the current screen geometry (`x`, `y`).
    ///
    /// This is primarily useful if the screen is externally corrupted, or if an
    /// [NCKEY_RESIZE][crate::sys::NCKEY_RESIZE] event has been read and you're not
    /// yet ready to render.
    // WIP: sys::NCKEY_RESIZE reference
    pub fn refresh(&mut self) -> Result<(Dimension, Dimension)> {
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
    pub fn term_capabilities(&self) -> Capabilities {
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
        }
    }

    /// Returns the size of the terminal in columns and rows (x, y).
    pub fn term_size(&self) -> (Dimension, Dimension) {
        self.raw.term_dim_yx()
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
