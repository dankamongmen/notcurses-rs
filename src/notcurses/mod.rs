// notcurses::notcurses
//
//!
//

use crate::{
    sys::{Nc, NcInput},
    Blitter, Error, Event, MiceEvents, Palette, Plane, PlaneGeometry, Result, Rgb, Size,
    Statistics, Style,
};
use core::cell::RefCell;
use once_cell::sync::OnceCell;

mod capabilities;
pub use capabilities::Capabilities;

thread_local!(
    /// Restricts initializing more than one `Notcurses` instance per thread, at the same time.
    static NOTCURSES_LOCK: RefCell<OnceCell<bool>> = RefCell::new(OnceCell::new());

    /// Restricts instancing the standard `Plane` more than once per `Notcurses` instance.
    pub(crate) static CLI_PLANE_LOCK: RefCell<OnceCell<bool>> = RefCell::new(OnceCell::new());
);

/// *Notcurses* state for a given terminal, composed of [`Plane`][crate::Plane]s.
///
/// There can only be a single `Notcurses` instance per thread at any given moment.
#[derive(Debug)]
pub struct Notcurses {
    nc: *mut Nc,
}

mod std_impls {
    use super::{Notcurses, OnceCell, NOTCURSES_LOCK};

    impl Drop for Notcurses {
        fn drop(&mut self) {
            let _ = unsafe { self.into_ref_mut().stop().expect("Notcurses.drop()") };
            // Allows initializing a new Notcurses instance again.
            NOTCURSES_LOCK.with(|refcell| {
                refcell.replace(OnceCell::new());
            });
        }
    }
}

// private functions
impl Notcurses {
    // Errors if there's already one `Notcurses` instance in this thread.
    fn already_notcurses() -> Result<()> {
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

    // Errors if there's already one `Plane` that refers to the standard plane in this thread.
    pub(crate) fn already_cli_plane() -> Result<()> {
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
        Self::already_notcurses()?;
        let nc = unsafe { Nc::new()? };
        Ok(Notcurses { nc })
    }

    /// Returns a new `Notcurses` context, without banners.
    pub fn with_banners() -> Result<Self> {
        Self::already_notcurses()?;
        let nc = unsafe { Nc::with_banners()? };
        Ok(Notcurses { nc })
    }

    /// Returns a new `Notcurses` context in `CLI` mode.
    pub fn new_cli() -> Result<Self> {
        Self::already_notcurses()?;
        let nc = unsafe { Nc::new_cli()? };
        Ok(Notcurses { nc })
    }

    /// Returns a new `Notcurses` context in `CLI` mode, without banners.
    pub fn with_banners_cli() -> Result<Self> {
        Self::already_notcurses()?;
        let nc = unsafe { Nc::with_banners_cli()? };
        Ok(Notcurses { nc })
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
        Self::already_cli_plane()?;
        Ok(unsafe { self.into_ref_mut().stdplane().into() })
    }

    pub fn new_palette(&mut self) -> Palette {
        Palette::new(self)
    }
}

/// # event methods
impl Notcurses {
    /// Enables receiving the provided mice `events`.
    pub fn mice_enable(&mut self, events: MiceEvents) -> Result<()> {
        Ok(self.into_ref_mut().mice_enable(events)?)
    }

    /// Disables receiving the mice events.
    pub fn mice_disable(&mut self) -> Result<()> {
        self.mice_enable(MiceEvents::None)
    }

    /// Waits for an event, blocking.
    pub fn get_event(&mut self) -> Result<Event> {
        let mut input = NcInput::new_empty();
        let received = self.into_ref_mut().get_blocking(Some(&mut input))?;
        Ok((received, input).into())
    }

    /// Tries to get an event, non blocking.
    pub fn poll_event(&mut self) -> Result<Event> {
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
            pixel_impl: self.into_ref().check_pixel_support(),
            truecolor: self.into_ref().cantruecolor(),
            fade: self.into_ref().canfade(),
            palette_change: self.into_ref().canchangecolor(),
            palette_size: self.into_ref().palette_size().unwrap_or(0),
        }
    }

    //

    /// Returns an [`Style`] with the supported curses-style attributes.
    ///
    /// The attribute is only indicated as supported if the terminal can support
    /// it together with color.
    pub fn supported_styles(&self) -> Style {
        self.into_ref().supported_styles()
    }

    /// Returns the default background color, if it is known.
    pub fn default_background(&self) -> Option<Rgb> {
        self.into_ref().default_background()
    }

    /// Returns the default foreground color, if it is known.
    pub fn default_foreground(&self) -> Option<Rgb> {
        self.into_ref().default_foreground()
    }

    /// Returns the terminal geometry with the best resolution blitter available,
    /// using the following rules of *graceful degradation*:
    ///
    /// [`Pixel`] > [`Sextant`] > [`Quadrant`] > [`Half`] > [`Ascii`].
    ///
    /// [`Pixel`]: crate::sys::NcBlitter#variant.Pixel
    /// [`Sextant`]: crate::sys::NcBlitter#variant.Sextant
    /// [`Quadrant`]: crate::sys::NcBlitter#variant.Quadrant
    /// [`Half`]: crate::sys::NcBlitter#variant.Half
    /// [`Ascii`]: crate::sys::NcBlitter#variant.Ascii
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
    pub fn geometry_first(&self, blitters: Vec<Blitter>) -> Option<PlaneGeometry> {
        PlaneGeometry::from_term_first(self, blitters)
    }

    /// Returns all the availeble terminal geometries from the provided list.
    pub fn geometries_all(&self, blitters: Vec<Blitter>) -> Vec<PlaneGeometry> {
        PlaneGeometry::from_term_all(self, blitters)
    }

    /// Returns the terminal size `(height, width)`.
    pub fn size(&self) -> Size {
        self.into_ref().term_dim_yx().into()
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

    /// Returns the name of the detected terminal.
    pub fn detected_terminal(&self) -> String {
        self.into_ref().detected_terminal()
    }

    /// Returns the name of the detected OS version.
    pub fn osversion(&self) -> String {
        self.into_ref().osversion()
    }

    /// Acquires an atomic snapshot of the notcurses object's stats.
    pub fn stats(&mut self, stats: &mut Statistics) {
        self.into_ref_mut().stats(stats)
    }

    /// Allocates a [`Statistics`] object.
    ///
    /// Use this rather than allocating your own, since future versions of
    /// notcurses might enlarge this structure.
    pub fn stats_alloc(&mut self) -> &mut Statistics {
        self.into_ref_mut().stats_alloc()
    }

    /// Resets all cumulative stats (immediate ones, such as fbbytes, are not reset).
    pub fn stats_reset(&mut self, stats: &mut Statistics) {
        self.into_ref_mut().stats_reset(stats)
    }
}
