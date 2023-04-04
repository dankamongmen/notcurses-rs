// notcurses::notcurses::notcurses
//
//!
//

use once_cell::sync::OnceCell;

use super::{Capabilities, Statistics};
use crate::{
    color::{Palette, Rgb},
    error::{NotcursesError as Error, NotcursesResult as Result},
    input::{Input, MiceEvents},
    plane::{Plane, PlaneGeometry, Position, Size, Style},
    sys::{Nc, NcInput, NcOptionsBuilder},
    visual::{Blitter, Visual, VisualGeometry},
    CLI_PLANE_LOCK, NOTCURSES_LOCK,
};

/// *Notcurses* state for a given terminal, composed of [`Plane`][crate::plane::Plane]s.
///
/// There can only be a single `Notcurses` instance per thread at any given moment.
pub struct Notcurses {
    pub(super) nc: *mut Nc,
    pub(super) options: NcOptionsBuilder,
}

mod core_impls {
    use super::{Notcurses, OnceCell, NOTCURSES_LOCK};
    use core::fmt;

    impl Drop for Notcurses {
        fn drop(&mut self) {
            unsafe { self.into_ref_mut().drop_planes() };
            unsafe { self.into_ref_mut().stop().expect("Notcurses.drop()") };
            // Allows initializing a new Notcurses instance again.
            NOTCURSES_LOCK.with(|refcell| {
                refcell.replace(OnceCell::new());
            });
        }
    }

    impl fmt::Debug for Notcurses {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let (mt, mr, mb, ml) = self.options.get_margins();
            let margins = if mt + mr + mb + ml == 0 {
                String::from("[]")
            } else {
                format!["margins:[{mt},{mr},{mb},{ml}]"]
            };
            let log = self.options.get_log_level();

            let mut flags = String::new();
            //
            if self.options.is_cli_mode() {
                flags += "CliMode[";
                if self.options.is_no_alternate_screen() {
                    flags += "NoAlternateScreen+";
                }
                if self.options.is_no_clear_bitmaps() {
                    flags += "NoClearBitmaps+";
                }
                if self.options.is_preserve_cursor() {
                    flags += "PreserveCursor+";
                }
                if self.options.is_scrolling() {
                    flags += "Scrolling+";
                }
            }
            if self.options.is_cli_mode() {
                flags.pop();
                flags += ")+";
            }
            //
            if self.options.is_no_font_changes() {
                flags += "NoFontChanges+";
            }
            if self.options.is_suppress_banners() {
                flags += "SuppressBanners+";
            }
            if self.options.is_drain_input() {
                flags += "DrainInput+";
            }
            if self.options.is_inhibit_set_locale() {
                flags += "InhibitSetLocale+";
            }
            if self.options.is_no_quit_sig_handlers() {
                flags += "NoQuitSigHandlers+";
            }
            if self.options.is_no_winch_sig_handler() {
                flags += "NoWinchSigHandler+";
            }
            flags.pop();

            write!(f, "Notcurses {{ {log} {margins} {flags} }}")
        }
    }
}

// private functions
impl Notcurses {
    // Errors if there's already one `Notcurses` instance in this thread.
    // Activates the lock otherwise.
    pub(super) fn lock_notcurses() -> Result<()> {
        NOTCURSES_LOCK.with(|refcell| {
            let cell = refcell.borrow_mut();
            if cell.get().is_none() {
                cell.set(true).unwrap();
                Ok(())
            } else {
                Error::msg("Only one `Notcurses` instance is allowed per thread, at the same time.")
            }
        })
    }

    /// Returns `true` if there's already a notcurses instance initialized in this thread.
    pub fn is_initialized() -> bool {
        NOTCURSES_LOCK.with(|refcell| refcell.borrow().get().is_some())
    }

    // Errors if there's already one `Plane` that refers to the standard plane in this thread.
    pub(crate) fn lock_cli_plane() -> Result<()> {
        CLI_PLANE_LOCK.with(|refcell| {
            let cell = refcell.borrow_mut();
            if cell.get().is_none() {
                cell.set(true).unwrap();
                Ok(())
            } else {
                Error::msg("Only one *CLI* `Plane` is allowed per `Notcurses` instance.")
            }
        })
    }
}

/// # constructors & deconstructors.
impl Notcurses {
    /// Returns a new `Notcurses` context.
    pub fn new() -> Result<Self> {
        Self::lock_notcurses()?;
        let options = NcOptionsBuilder::new().suppress_banners(true);
        let nc = unsafe { Nc::with_options(options.build())? };
        Ok(Notcurses { nc, options })
    }

    /// Returns a new `Notcurses` context, with banners.
    pub fn with_banners() -> Result<Self> {
        Self::lock_notcurses()?;
        let options = NcOptionsBuilder::new();
        let nc = unsafe { Nc::with_options(options.build())? };
        Ok(Notcurses { nc, options })
    }

    /// Returns a new `Notcurses` context in `CLI` mode.
    pub fn new_cli() -> Result<Self> {
        Self::lock_notcurses()?;
        let options = NcOptionsBuilder::new()
            .suppress_banners(true)
            .cli_mode(true);
        let nc = unsafe { Nc::with_options(options.build())? };
        Ok(Notcurses { nc, options })
    }

    /// Returns a new `Notcurses` context in `CLI` mode, with banners.
    pub fn with_banners_cli() -> Result<Self> {
        Self::lock_notcurses()?;
        let options = NcOptionsBuilder::new().cli_mode(true);
        let nc = unsafe { Nc::with_options(options.build())? };
        Ok(Notcurses { nc, options })
    }

    //

    /// Returns a shared reference to the inner [`Nc`].
    pub fn into_ref(&self) -> &Nc {
        unsafe { &*self.nc }
    }

    /// Returns an exclusive reference to the inner [`Nc`].
    pub fn into_ref_mut(&mut self) -> &mut Nc {
        unsafe { &mut *self.nc }
    }
}

/// # constructors for other types.
impl Notcurses {
    pub fn cli_plane(&mut self) -> Result<Plane> {
        Self::lock_cli_plane()?;
        Ok(unsafe { self.into_ref_mut().stdplane().into() })
    }

    pub fn new_palette(&mut self) -> Palette {
        Palette::new(self)
    }
}

/// # event methods
impl Notcurses {
    /// Refreshes the physical screen to match what was last rendered (i.e.,
    /// without reflecting any changes since the last call to
    /// [`render`][crate::Notcurses#method.render]).
    ///
    /// Returns the current screen geometry (`y`, `x`).
    ///
    /// This is primarily useful if the screen is externally corrupted, or if a
    /// resize] event has been read and you're not yet ready to render.
    pub fn refresh(&mut self) -> Result<(u32, u32)> {
        Ok(self.into_ref_mut().refresh()?)
    }

    /// Enables receiving the provided mice events.
    pub fn mice_enable(&mut self, input: MiceEvents) -> Result<()> {
        Ok(self.into_ref_mut().mice_enable(input.into())?)
    }

    /// Disables receiving the mice events.
    pub fn mice_disable(&mut self) -> Result<()> {
        self.mice_enable(MiceEvents::None)
    }

    /// Waits for an event, blocking.
    pub fn get_event(&mut self) -> Result<Input> {
        let mut input = NcInput::new_empty();
        let received = self.into_ref_mut().get_blocking(Some(&mut input))?;
        Ok((received, input).into())
    }

    /// Tries to get an event, non blocking.
    pub fn poll_event(&mut self) -> Result<Input> {
        let mut input = NcInput::new_empty();
        let received = self.into_ref_mut().get_nblock(Some(&mut input))?;
        Ok((received, input).into())
    }

    // /// Gets a file descriptor suitable for input event poll()ing.
    // ///
    // /// When this descriptor becomes available, you can call
    // /// [poll_event][Notcurses#method.poll_event], and input ought be ready.
    // ///
    // pub fn input_ready(&mut self) -> Result<i32> {
    //     Ok(self.into_ref_mut().inputready_fd()?)
    // }
}

/// # general query methods
impl Notcurses {
    /// Returns the terminal size.
    pub fn size(&self) -> Size {
        Size::from(self.into_ref().term_dim_yx()).swap()
    }

    /// Returns the terminal geometry with the best resolution blitter available,
    /// using the following rules of *graceful degradation*:
    ///
    /// [`Pixel`] > [`Sextant`] > [`Quadrant`] > [`Half`] > [`Ascii`].
    ///
    /// [`Pixel`]: crate::visual::Blitter#variant.Pixel
    /// [`Sextant`]: crate::visual::Blitter#variant.Sextant
    /// [`Quadrant`]: crate::visual::Blitter#variant.Quadrant
    /// [`Half`]: crate::visual::Blitter#variant.Half
    /// [`Ascii`]: crate::visual::Blitter#variant.Ascii
    pub fn geometry_best(&self) -> PlaneGeometry {
        PlaneGeometry::from_term(self, self.capabilities().best_blitter())
    }

    /// Returns the terminal geometry using the requested blitter, if available.
    pub fn geometry_try(&self, blitter: Blitter) -> Option<PlaneGeometry> {
        if self.capabilities().can_blitter(blitter) {
            Some(PlaneGeometry::from_term(self, blitter))
        } else {
            None
        }
    }

    /// Returns the first terminal geometry available from the provided list.
    pub fn geometry_first(&self, blitters: &[Blitter]) -> Option<PlaneGeometry> {
        PlaneGeometry::from_term_first(self, blitters)
    }

    /// Returns all the availeble terminal geometries from the provided list.
    pub fn geometries_all(&self, blitters: &[Blitter]) -> Vec<PlaneGeometry> {
        PlaneGeometry::from_term_all(self, blitters)
    }

    /// Returns the visual geometry of a visual.
    pub fn visual_geometry(&self, visual: &Visual) -> Result<VisualGeometry> {
        Ok(self
            .into_ref()
            .visual_geom(Some(visual.into_ref()), Some(&visual.options()))?
            .into())
    }

    /// Returns the capabilities of the terminal.
    pub fn capabilities(&self) -> Capabilities {
        Capabilities {
            halfblock: self.into_ref().canhalfblock(),
            quadrant: self.into_ref().canquadrant(),
            sextant: self.into_ref().cansextant(),
            braille: self.into_ref().canbraille(),
            utf8: self.into_ref().canutf8(),
            images: self.into_ref().canopen_images(),
            videos: self.into_ref().canopen_videos(),
            pixel: self.into_ref().canpixel(),
            pixel_implementation: self.into_ref().check_pixel_support().into(),
            truecolor: self.into_ref().cantruecolor(),
            fade: self.into_ref().canfade(),
            palette_change: self.into_ref().canchangecolor(),
            palette_size: self.into_ref().palette_size().unwrap_or(0),
        }
    }

    /// Returns an [`Style`] with the supported curses-style attributes.
    ///
    /// The attribute is only indicated as supported if the terminal can support
    /// it together with color.
    pub fn supported_styles(&self) -> Style {
        self.into_ref().supported_styles().into()
    }

    /// Returns the default background color, if it is known.
    pub fn default_background(&self) -> Option<Rgb> {
        self.into_ref().default_background().map(|rgb| rgb.into())
    }

    /// Returns the default foreground color, if it is known.
    pub fn default_foreground(&self) -> Option<Rgb> {
        self.into_ref().default_foreground().map(|rgb| rgb.into())
    }

    /// Returns a human-readable string describing the running notcurses version.
    pub fn version() -> String {
        Nc::version()
    }

    /// Returns the running notcurses version components
    /// (major, minor, patch, tweak).
    pub fn version_components() -> (u32, u32, u32, u32) {
        Nc::version_components()
    }

    /// Returns the name of the user under which we are running.
    pub fn accountname() -> String {
        Nc::accountname()
    }

    /// Returns the name of the local hostname.
    pub fn hostname() -> String {
        Nc::hostname()
    }

    /// Returns the name of the detected OS version.
    pub fn osversion() -> String {
        Nc::osversion()
    }

    /// Returns the name of the detected terminal.
    pub fn detected_terminal(&self) -> String {
        self.into_ref().detected_terminal()
    }
}

/// # settings methods
impl Notcurses {
    /// Disables the terminal's cursor.
    pub fn cursor_disable(&mut self) -> Result<()> {
        Ok(self.into_ref_mut().cursor_disable()?)
    }

    /// Enables the terminal's cursor, if available, placing it at `position`.
    pub fn cursor_enable(&mut self, position: impl Into<Position>) -> Result<()> {
        let (y, x) = position.into().into();
        Ok(self.into_ref_mut().cursor_enable(y, x)?)
    }

    /// Leaves the alternate screen.
    pub fn leave_alternate_screen(&mut self) -> Result<()> {
        self.options.set_no_alternate_screen(true);
        Ok(self.into_ref_mut().leave_alternate_screen()?)
    }

    /// Enters the alternate screen, if available.
    ///
    /// Entering the alternate screen turns off scrolling for the *CLI* plane.
    pub fn enter_alternate_screen(&mut self) -> Result<()> {
        self.options.set_no_alternate_screen(false);
        Ok(self.into_ref_mut().enter_alternate_screen()?)
    }

    /// Disables signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z). They are enabled by default.
    pub fn signals_disable(&mut self) -> Result<()> {
        self.options.set_no_quit_sig_handlers(true);
        Ok(self.into_ref_mut().linesigs_disable()?)
    }

    /// Restores signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z), if disabled.
    pub fn signals_enable(&mut self) -> Result<()> {
        self.options.set_no_quit_sig_handlers(false);
        Ok(self.into_ref_mut().linesigs_enable()?)
    }
}

/// # statistics methods
impl Notcurses {
    /// Allocates a [`Statistics`] object.
    pub fn statistics(&mut self) -> Statistics {
        Statistics::new(self)
    }

    /// Resets all cumulative statistics.
    ///
    /// Immediate ones, such as fbbytes, are not reset.
    pub fn statistics_reset(&mut self, mut stats: Statistics) {
        stats.reset(self)
    }

    /// Acquires an atomic snapshot of the notcurses statistics.
    pub fn statistics_update(&mut self, mut stats: Statistics) {
        stats.update(self)
    }
}
