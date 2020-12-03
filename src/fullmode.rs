// methods: 39
// ------------------------------------------ (done / remaining)
// (+) done: 12 / 27
// (#) test:  0 / 39
// ------------------------- ↓ from bindgen
//± notcurses_at_yx
//+ notcurses_canchangecolor
//+ notcurses_canfade
//+ notcurses_canopen_images
//+ notcurses_canopen_videos
//+ notcurses_cansixel
//+ notcurses_cantruecolor
//+ notcurses_canutf8
//+ notcurses_cursor_disable
//+ notcurses_cursor_enable
//  notcurses_debug
//+ notcurses_drop_planes
//  notcurses_getc
//+ notcurses_init             // inside new() and the other constructors
//  notcurses_inputready_fd
//  notcurses_lex_blitter
//  notcurses_lex_margins
//  notcurses_lex_scalemode
//  notcurses_mouse_disable
//  notcurses_mouse_enable
//  notcurses_palette_size
//  notcurses_refresh
//+ notcurses_render
//  notcurses_render_to_file
//  notcurses_reset_stats
//  notcurses_stats
//+ notcurses_stdplane
//  notcurses_stdplane_const
//x notcurses_stop             // in Drop Trait
//  notcurses_str_blitter
//  notcurses_str_scalemode
//  notcurses_supported_styles
//  notcurses_top
//  notcurses_version
//  notcurses_version_components
// ------------------------- ↓ static inlines reimplemented
//  notcurses_getc_blocking
//  notcurses_getc_nblock
//  notcurses_stddim_yx
//  notcurses_term_dim_yx

use core::ptr::{null, null_mut};

use enumflags2::BitFlags;
use strum::IntoEnumIterator;

use crate::{sys, Cell, Error, LogLevel, Plane, Style};

/// [`FullMode`] Option Flags
///
#[repr(u64)]
#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
pub enum FullModeFlag {
    InhibitSetlocale = sys::NCOPTION_INHIBIT_SETLOCALE as u64,
    VerifySixel = sys::NCOPTION_VERIFY_SIXEL as u64,
    NoWinchSighandler = sys::NCOPTION_NO_WINCH_SIGHANDLER as u64,
    NoQuitSighandlers = sys::NCOPTION_NO_QUIT_SIGHANDLERS as u64,
    /// Remove the startup diagnostics
    SuppressBanners = sys::NCOPTION_SUPPRESS_BANNERS as u64,
    /// Don't use the alternate screen
    NoAlternateScreen = sys::NCOPTION_NO_ALTERNATE_SCREEN as u64,
    /// Don't change the font
    NoFontChange = sys::NCOPTION_NO_FONT_CHANGES as u64,
}
// NOTE: This doesn't work right now, waiting for the next release of enumflags2
// with const support:
// ```
// impl FullModeFlag {
//     pub const EMPTY: BitFlags<FullModeFlag> = BitFlags::empty();
// }
// ```

/// Options for [`FullMode`]
///
/// notcurses_init accepts a struct notcurses_options allowing fine-grained
/// control of notcurses behavior, including signal handlers, alternative
/// screens, and overriding the TERM environment variable.
///
/// A terminfo entry appropriate for the actual terminal must be available
pub struct FullModeOptions {
    pub data: sys::NotcursesOptions,
}

impl FullModeOptions {
    // CONSTRUCTORS new()

    /// Return a new customized FullModeOptions structure
    pub fn new(loglevel: LogLevel, flags: impl Into<BitFlags<FullModeFlag>>) -> Self {
        FullModeOptions {
            data: sys::NotcursesOptions {
                // Progressively higher log levels result in more logging to stderr. By
                // default, nothing is printed to stderr once fullscreen service begins.
                loglevel: loglevel as u32,

                // General flags; see NCOPTION_*. This is expressed as a bitfield so that
                // future options can be added without reshaping the struct. Undefined bits
                // must be set to 0.
                flags: flags.into().bits(),

                // The name of the terminfo database entry describing this terminal. If NULL
                // the environment variable TERM is used. Failure to open the terminal
                // definition will result in failure to initialize notcurses.
                //
                // BUG: see /examples/error-1.rs
                // termtype: std::ffi::CString::new("xterm-256color").unwrap().as_ptr(), // DEBUG doesn't work
                termtype: null(),

                // If non-NULL, notcurses_render() will write each rendered frame to
                // this FILE* in addition to outfp. This is used primarily for debugging.
                renderfp: null_mut(),

                // Desirable margins. If all are 0 (default) we will render to the entirety of the screen.
                // If the screen is too small, we do what we can--this is strictly best-effort.
                // Absolute coordinates are relative to the rendering area (with (0, 0) as the origin).
                margin_t: 0,
                margin_r: 0,
                margin_b: 0,
                margin_l: 0,
            },
        }
    }
}

/// Full Mode `notcurses` Context
///
/// ## Links
/// - [man notcurses](https://nick-black.com/notcurses/notcurses.3.html)
pub struct FullMode {
    pub data: *mut sys::bindgen::notcurses,
}

/// # `FullMode` Constructors
impl FullMode {
    // CONSTRUCTORS: -----------------------------------------------------------
    // - new()
    // - new_test() /*private*/           // the preferred format for unit tests
    // - with_banners()
    // - with_options()
    // - without_altmode()

    /// Return a [`FullMode`] instance that:
    ///
    /// - uses the alternate mode
    /// - doesn't show the info banners
    ///
    pub fn new() -> Result<Self, Error> {
        Self::with_options(FullModeOptions::new(
            LogLevel::Silent,
            FullModeFlag::SuppressBanners,
        ))
    }

    /// Return a [`FullMode`] instance perfect for unit tests
    pub(crate) fn new_test() -> Result<Self, Error> {
        FullMode::with_options(FullModeOptions::new(
            LogLevel::Silent,
            FullModeFlag::InhibitSetlocale
                | FullModeFlag::SuppressBanners
                | FullModeFlag::NoAlternateScreen
                | FullModeFlag::NoWinchSighandler
                | FullModeFlag::NoQuitSighandlers,
        ))
    }

    /// Return a [`FullMode`] instance that:
    ///
    /// - uses the alternate mode
    /// - shows the info banners
    ///
    pub fn with_banners() -> Result<Self, Error> {
        Self::with_options(FullModeOptions::new(LogLevel::Silent, BitFlags::empty()))
    }

    /// Return a [`FullMode`] instance with custom options
    ///
    // TODO: move constructors from options to here (without_altmode, etc.)
    // TODO:
    // (1) always call setlocale as the first thing you do, using LC_ALL, "" as arguments.
    // document that users of your crate ought have LANG properly defined.
    // (2) pass the FullModeFlag::InhibitSetlocale once you're doing so
    // [link](https://github.com/dankamongmen/notcurses/issues/866#issuecomment-672921476)
    //
    pub fn with_options(options: FullModeOptions) -> Result<Self, Error> {
        unsafe {
            // Before calling into notcurses be sure to call setlocale with an appropriate UTF-8 LC_ALL locale. It is
            // appropriate to use setlocale(LC_ALL, ""), relying on the user to set the LANG environment variable.
            //
            // [docs.rs → libc::setlocale](https://docs.rs/libc/0.2.74/libc/fn.setlocale.html)

            let _ = libc::setlocale(libc::LC_ALL, std::ffi::CString::new("").unwrap().as_ptr());
        }

        Ok(FullMode {
            // notcurses_init prepares the terminal for cursor-addressable (multiline) mode.
            //
            // notcurses_init accepts a struct notcurses_options allowing fine-grained control of notcurses behavior,
            // including signal handlers, alternative screens, and overriding the TERM environment variable.
            // A terminfo entry appropriate for the actual terminal must be available.
            //
            // ## arguments:
            //
            // 1. The FILE provided as fp must be writable and attached to a terminal, or NULL which will open /dev/tty.
            // 2. The struct notcurses_option passed as opts controls behavior.
            //    Only one instance should be associated with a given terminal at a time,
            //    though it is no problem to have multiple instances in a given process.
            //
            // - [man notcurses_init](https://nick-black.com/notcurses/notcurses_init.3.html)
            //
            data: unsafe { sys::notcurses_init(&options.data, null_mut()) },
        })
    }

    /// Return a [`FullMode`] instance that:
    ///
    /// - doesn't use the alternate mode
    /// - doesn't show the info banners
    ///
    pub fn without_altmode() -> Result<Self, Error> {
        Self::with_options(FullModeOptions::new(
            LogLevel::Silent,
            FullModeFlag::NoAlternateScreen | FullModeFlag::SuppressBanners,
        ))
    }

    // /// Return
    // pub fn raw_mut() -> *mut {
    //    self.data
    // }
}

/// # `FullMode` Methods
impl FullMode {
    // TODO:
    // /// Retrieve the [`Cell`] at  the specified coordinates as last rendered.
    // ///
    // // Returns the EGC or NULL on error. This EGC must be free()d by the caller. The stylemask and channels are written to 'stylemask' and 'channels', respectively.
    // pub fn at_yx(&self, y: u32, x: u32) -> Result<Cell, Error> {
    //     let (mut style, mut channels) = (0, 0);
    //     let egc = unsafe {
    //         sys::notcurses_at_yx(self.data, y as i32, x as i32, &mut style, &mut channels)
    //     };
    //     if egc.is_null() {
    //         return Err(Error::Cell);
    //     }
    //     Ok(Cell::new_blank())
    // }

    /// Can we set the "hardware" palette?
    ///
    /// Requires the "ccc" terminfo capability.
    pub fn can_change_color(&self) -> bool {
        unsafe { sys::notcurses_canchangecolor(self.data) }
    }

    /// Can we fade?
    ///
    /// Requires either the "rgb" or "ccc" terminfo capability.
    pub fn can_fade(&self) -> bool {
        unsafe { sys::notcurses_canfade(self.data) }
    }

    /// Can we load images?
    ///
    /// Requires being built against FFmpeg/OIIO.
    pub fn can_open_images(&self) -> bool {
        unsafe { sys::notcurses_canopen_images(self.data) }
    }

    /// Can we load videos?
    ///
    /// Requires being built against FFmpeg.
    pub fn can_open_videos(&self) -> bool {
        unsafe { sys::notcurses_canopen_videos(self.data) }
    }

    /// Can we blit to Sixel?
    pub fn can_sixel(&self) -> bool {
        unsafe { sys::notcurses_cansixel(self.data) }
    }

    /// Can we directly specify RGB values per cell?
    ///
    /// If not, we can only use palettes.
    pub fn can_truecolor(&self) -> bool {
        unsafe { sys::notcurses_cantruecolor(self.data) }
    }

    /// Is our encoding UTF-8?
    ///
    /// Requires LANG being set to a UTF8 locale.
    pub fn can_utf8(&self) -> bool {
        unsafe { sys::notcurses_canutf8(self.data) }
    }

    /// Disables the cursor
    pub fn cursor_disable(&mut self) {
        unsafe {
            sys::notcurses_cursor_disable(self.data);
        }
    }

    /// Enables the cursor
    pub fn cursor_enable(&mut self, y: i32, x: i32) {
        unsafe {
            sys::notcurses_cursor_enable(self.data, y, x);
        }
    }

    /// Return the dimensions of the terminal as a tuple of (rows, cols)
    //
    // TODO: rename
    pub fn dim_yx(&mut self) -> (u32, u32) {
        let mut y = 0;
        let mut x = 0;
        unsafe {
            sys::notcurses_term_dim_yx(&*self.data, &mut y, &mut x);
        }
        (y, x)
    }

    /// Destroy all ncplanes other than the stdplane.
    pub fn drop_planes(&mut self) {
        unsafe {
            sys::notcurses_drop_planes(self.data);
        }
    }

    pub fn render(&mut self) {
        unsafe {
            sys::notcurses_render(self.data);
        }
    }

    /// Returns the standard Plane for the current context
    ///
    ///
    /// NOTE: It is an error to call ncplane_destroy, ncplane_resize, or ncplane_move
    /// on the standard plane, but it can be freely moved along the z-axis.
    ///
    /// [man notcurses_stdplane](https://nick-black.com/notcurses/notcurses_stdplane.3.html)
    ///
    pub fn stdplane(&mut self) -> Plane {
        unsafe { Plane::from_ncplane(sys::notcurses_stdplane(self.data)) }
    }

    /// Returns a flag that indicates the supported styles for the current terminal
    pub fn supported_styles(&self) -> u16 {
        unsafe { sys::notcurses_supported_styles(self.data) as u16 }
    }

    /// Returns the name of the flags supported
    pub fn supported_styles_str(&self) -> String {
        let sf = self.supported_styles();
        let mut sstr = String::new();

        for s in Style::iter() {
            if s as u16 & sf != 0 {
                sstr += &format! {" {:?}", s};
            }
        }
        sstr.trim().to_owned()
    }
}

impl Drop for FullMode {
    fn drop(&mut self) {
        // It is important to reset the terminal before exiting, whether
        // terminating due to intended operation or a received signal.
        // This is usually accomplished by explicitly calling notcurses_stop.
        //
        // For convenience, notcurses by default installs signal handlers for
        // various signals typically resulting in process termination.
        // These signal handlers call notcurses_stop for each struct notcurses
        // in the process, and then propagate the signal to any previously-
        // configured handler.
        // These handlers are disabled upon entry to notcurses_stop.
        //
        // notcurses in full or direct mode is always supposed to leave you with:
        // - palette reset (oc terminfo)
        // - cursor visible (cnorm terminfo)
        // - all styles reset (sgr0 terminfo)
        //
        // [API](https://nick-black.com/notcurses/notcurses_stop.3.html)
        unsafe {
            sys::notcurses_stop(self.data);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() -> Result<(), Error> {
        let _ = FullMode::new()?;
        Ok(())
    }

    #[test]
    fn for_testing() -> Result<(), Error> {
        let _ = FullMode::new_test()?;
        Ok(())
    }

    #[test]
    fn stdplane() -> Result<(), Error> {
        let mut nc = FullMode::new_test()?;
        let _p = nc.stdplane();
        Ok(())
    }
}
