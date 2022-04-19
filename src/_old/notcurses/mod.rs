//! `Notcurses` wrapper struct and traits implementations.

use crate::{ncresult, sys::{Nc, NcReceived, NcMiceEvents}, Geometry, Input, NResult};

mod builder;
mod capabilities;
mod loglevel;
mod pixel_impl;
pub use builder::NotcursesBuilder;
pub use capabilities::Capabilities;
pub use loglevel::LogLevel;
pub use pixel_impl::PixelImpl;

/// The **notcurses** context.
#[derive(Debug)]
pub struct Notcurses<'nc> {
    pub(crate) nc: &'nc mut Nc,
}

impl<'nc> Drop for Notcurses<'nc> {
    /// Destroys the Notcurses context.
    fn drop(&mut self) {
        let _ = self.nc.stop();
    }
}

/// # Constructors and converters.
impl<'nc> Notcurses<'nc> {
    /// New `Notcurses` instance.
    pub fn new() -> NResult<Self> {
        Ok(Self { nc: Nc::new()? })
    }

    /// New `Notcurses` instance, without an alternate screen.
    pub fn without_altscreen() -> NResult<Self> {
        Notcurses::build().altscreen(false).finish()
    }

    /// Returns a [`NotcursesBuilder`] used to customize a new
    /// `Notcurses` instance.
    pub fn build() -> NotcursesBuilder {
        NotcursesBuilder::default()
    }

    /// Returns a reference to the inner [`Nc`].
    pub fn as_nc(&self) -> &Nc {
        self.nc
    }

    /// Returns a mutable reference to the inner [`Nc`].
    pub fn as_nc_mut(&mut self) -> &mut Nc {
        self.nc
    }
}

impl<'nc> Notcurses<'nc> {
    // pub fn align
    // pub fn at_yx

    /// Disables the terminal cursor, if supported.
    pub fn cursor_disable(&mut self) -> NResult<()> {
        ncresult![self.nc.cursor_disable()]
    }

    /// Enables the terminal cursor, if supported, plaxing it at `x`,`y`.
    pub fn cursor_enable(&mut self, x: u32, y: u32) -> NResult<()> {
        ncresult![self.nc.cursor_enable(y, x)]
    }

    // debug
    // debug_caps

    /// Destroys all [`Plane`][crate::Plane]s.
    ///
    /// Any pre-existing `Planes` will be invalid and shouldn't be used again.
    pub fn drop_planes(&mut self) {
        self.nc.drop_planes();
    }

    // TODO:
    // getc?
    // inputready_fd

    /// Reads input blocking until an event is processed or a signal is received.
    ///
    /// Will optionally write the event details in `input`.
    pub fn get_block(&mut self, input: Option<&mut Input>) -> NResult<NcReceived> {
        ncresult![self.nc.get_blocking(input.map(|i| &mut i.ncinput))]
    }

    /// Reads input without blocking.
    ///
    /// If no event is ready, returns 0.
    pub fn get_noblock(&mut self, input: Option<&mut Input>) -> NResult<char> {
        ncresult![self.nc.get_nblock(input.map(|i| &mut i.ncinput))]
    }

    // lex_blitter
    // lex_margins
    // lex_scalemode

    /// Disables signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z). They are enabled by default.
    pub fn linesigs_disable(&mut self) -> NResult<()> {
        ncresult![self.nc.linesigs_disable()]
    }

    /// Restores signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z), if disabled.
    pub fn linesigs_enable(&mut self) -> NResult<()> {
        ncresult![self.nc.linesigs_enable()]
    }

    /// Disables mouse events.
    ///
    /// Any events in the input queue can still be delivered.
    pub fn mice_disable(&mut self) -> NResult<()> {
        ncresult![self.nc.mice_disable()]
    }

    /// Enable the mice in "button-event tracking" mode with focus detection
    /// and UTF8-style extended coordinates.
    ///
    /// On success, mice events will be published to
    /// [get()][Notcurses#method.get].
    pub fn mice_enable(&mut self, me: NcMiceEvents) -> NResult<()> {
        ncresult![self.nc.mice_enable(me)]
    }

    /// Refreshes the physical screen to match what was last rendered (i.e.,
    /// without reflecting any changes since the last call to
    /// [render][crate::Notcurses#method.render]).
    ///
    /// Returns the current terminal size (`x`, `y`).
    ///
    /// This is primarily useful if the screen is externally corrupted, or if an
    /// [NcKey::RESIZE][crate::sys::NcKey::RESIZE] event has been read and you're not
    /// yet ready to render.
    // TODO: sys::NCKEY_RESIZE reference
    pub fn refresh(&mut self) -> NResult<(u32, u32)> {
        let (y, x) = self.nc.refresh()?;
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
            halfblock: self.nc.canhalfblock(),
            quadrant: self.nc.canquadrant(),
            sextant: self.nc.cansextant(),
            braille: self.nc.canbraille(),
            utf8: self.nc.canutf8(),
            images: self.nc.canopen_images(),
            videos: self.nc.canopen_videos(),
            pixel: self.nc.check_pixel_support().into(),
            truecolor: self.nc.cantruecolor(),
            fade: self.nc.canfade(),
            palette_change: self.nc.canchangecolor(),
            palette_size: self.nc.palette_size().unwrap_or(0),
            cursor: true,
        }
    }

    /// Returns the size of the terminal in columns and rows.
    pub fn cols_rows(&self) -> (u32, u32) {
        let (h, w) = self.nc.term_dim_yx();
        (w, h)
    }

    /// Returns the `Geometry` of the terminal.
    pub fn geometry(&self) -> Geometry {
        let g = self.nc.stdplane_const().pixel_geom();
        Geometry {
            x: g.term_x,
            y: g.term_y,
            cols: g.term_x / g.cell_x,
            rows: g.term_y / g.cell_y,
            bx: g.max_bitmap_x,
            by: g.max_bitmap_y,
            bcols: g.max_bitmap_x / g.cell_x,
            brows: g.max_bitmap_y / g.cell_y,
            cx: g.cell_x,
            cy: g.cell_y,
        }
    }

    /// Returns the name of the detected terminal.
    pub fn term_name(&self) -> String {
        self.nc.detected_terminal()
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