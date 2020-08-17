// methods: 39
// ------------------------------------------ (done / wont / remaining)
// (+) implemented: 11 / … / 28
// (#) +unit tests:  0 / … / 39
// ------------------------- ↓ from bindgen
// notcurses_at_yx
//+notcurses_canchangecolor
//+notcurses_canfade
//+notcurses_canopen_images
//+notcurses_canopen_videos
//+notcurses_cansixel
//+notcurses_cantruecolor
//+notcurses_canutf8
//+notcurses_cursor_disable
//+notcurses_cursor_enable
// notcurses_debug
//+notcurses_drop_planes
// notcurses_getc
//+notcurses_init             // inside new()
// notcurses_inputready_fd
// notcurses_lex_blitter
// notcurses_lex_margins
// notcurses_lex_scalemode
// notcurses_mouse_disable
// notcurses_mouse_enable
// notcurses_palette_size
// notcurses_refresh
// notcurses_render
// notcurses_render_to_file
// notcurses_reset_stats
// notcurses_stats
// notcurses_stdplane
// notcurses_stdplane_const
//xnotcurses_stop             // in Drop Trait
// notcurses_str_blitter
// notcurses_str_scalemode
// notcurses_supported_styles
// notcurses_top
// notcurses_version
// notcurses_version_components
// ------------------------- ↓ static inlines reimplemented
// notcurses_getc_blocking
// notcurses_getc_nblock
// notcurses_stddim_yx
// notcurses_term_dim_yx

use std::ptr::{null, null_mut};

use libnotcurses_sys as nc;

use enumflags2::BitFlags;
use strum::IntoEnumIterator;

use crate::error::Error;
use crate::plane::Plane;
use crate::types::Style;

/// Log levels
///
/// By default, nothing is printed to stderr once fullscreen service begins.
/// Progressively higher log levels result in more logging to stderr:
#[repr(u32)] // = ncloglevel_e
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LogLevel {
    Silent = nc::ncloglevel_e_NCLOGLEVEL_SILENT as nc::ncloglevel_e,
    Panic = nc::ncloglevel_e_NCLOGLEVEL_PANIC as nc::ncloglevel_e,
    Fatal = nc::ncloglevel_e_NCLOGLEVEL_FATAL as nc::ncloglevel_e,
    Error = nc::ncloglevel_e_NCLOGLEVEL_ERROR as nc::ncloglevel_e,
    Warning = nc::ncloglevel_e_NCLOGLEVEL_WARNING as nc::ncloglevel_e,
    Info = nc::ncloglevel_e_NCLOGLEVEL_INFO as nc::ncloglevel_e,
    Debug = nc::ncloglevel_e_NCLOGLEVEL_DEBUG as nc::ncloglevel_e,
    Trace = nc::ncloglevel_e_NCLOGLEVEL_TRACE as nc::ncloglevel_e,
}

/// Option Flags
///
#[repr(u64)]
#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
pub enum OptionFlag {
    InhibitSetlocale = nc::NCOPTION_INHIBIT_SETLOCALE as u64,
    VerifySixel = nc::NCOPTION_VERIFY_SIXEL as u64,
    NoWinchSighandler = nc::NCOPTION_NO_WINCH_SIGHANDLER as u64,
    NoQuitSighandlers = nc::NCOPTION_NO_QUIT_SIGHANDLERS as u64,
    RetainCursor = nc::NCOPTION_RETAIN_CURSOR as u64, // Don't hide the cursor
    SuppressBanners = nc::NCOPTION_SUPPRESS_BANNERS as u64, // Remove the startup diagnostics
    NoAlternateScreen = nc::NCOPTION_NO_ALTERNATE_SCREEN as u64, // Don't use the alternate screen
    NoFontChange = nc::NCOPTION_NO_FONT_CHANGES as u64, // Don't change the font
}
// NOTE: This doesn't work right now, waiting for the next release of enumflags2 with const support
// impl OptionFlag {
//     pub const EMPTY: BitFlags<OptionFlag> = BitFlags::empty();
// }

/// A safe wrapper over notcurses_options
///
/// notcurses_init accepts a struct notcurses_options allowing fine-grained control of notcurses behavior,
/// including signal handlers, alternative screens, and overriding the TERM environment variable.
/// A terminfo entry appropriate for the actual terminal must be available
pub struct Options {
    pub(crate) data: nc::notcurses_options,
}

impl Options {

    // CONSTRUCTORS new()

    /// Return a new customized Options structure
    pub fn new(loglevel: LogLevel, flags: impl Into<BitFlags<OptionFlag>>) -> Self {
        Options {
            data: nc::notcurses_options {
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
                // termtype: std::ffi::CString::new("xterm-256color").unwrap().as_ptr(), // DEBUG it doesn't work
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


extern "C" {
    // Needed for notcurses_init()
    fn libc_stdout() -> *mut nc::_IO_FILE;
}

/// A safe wrapper over a notcurses context
///
/// ## Links
/// - [man notcurses](https://nick-black.com/notcurses/notcurses.3.html)
pub struct Notcurses {
    pub(crate) data: *mut nc::notcurses,
}

impl Notcurses {
    // CONSTRUCTORS: new(), for_testing() with_banners() with_options() without_altmode()---------------------------------

    /// Return a Notcurses instance that:
    ///
    /// - uses the alternate mode
    /// - doesn't show the info banners
    ///
    pub fn new() -> Result<Self, Error> {
        Self::with_options(Options::new(LogLevel::Silent, OptionFlag::SuppressBanners))
    }


    /// Return a Notcurses instance with custom options
    ///
    // TODO: move constructors from options to here (without_altmode, etc.)
    // TODO:
    // (1) always call setlocale as the first thing you do, using LC_ALL, "" as arguments.
    // document that users of your crate ought have LANG properly defined.
    // (2) pass the OptionFlag::InhibitSetlocale once you're doing so
    // [link](https://github.com/dankamongmen/notcurses/issues/866#issuecomment-672921476)
    //
    pub fn with_options(options: Options) -> Result<Self, Error> {
        unsafe {
            // Before calling into notcurses be sure to call setlocale with an appropriate UTF-8 LC_ALL locale. It is
            // appropriate to use setlocale(LC_ALL, ""), relying on the user to set the LANG environment variable.
            //
            // [docs.rs → libc::setlocale](https://docs.rs/libc/0.2.74/libc/fn.setlocale.html)

            let _ = libc::setlocale(libc::LC_ALL, std::ffi::CString::new("").unwrap().as_ptr());
        }

        Ok(Notcurses {
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
            data: unsafe { nc::notcurses_init(&options.data, libc_stdout()) },
        })
    }

    /// Return a Notcurses instance that:
    ///
    /// - uses the alternate mode
    /// - shows the info banners
    ///
    pub fn with_banners() -> Result<Self, Error> {
        Self::with_options(Options::new(LogLevel::Silent, BitFlags::empty()))
    }

    /// Return a Notcurses instance that:
    ///
    /// - doesn't use the alternate mode
    /// - doesn't show the info banners
    ///
    pub fn without_altmode() -> Result<Self, Error> {
        Self::with_options(Options::new(LogLevel::Silent,
             OptionFlag::NoAlternateScreen | OptionFlag::SuppressBanners))

    }

    /// Return a Notcurses instance perfect for unit tests
    pub(crate) fn for_testing() -> Result<Self, Error> {
        Self::with_options(Options::new(
            LogLevel::Silent,
            OptionFlag::InhibitSetlocale
                | OptionFlag::SuppressBanners
                | OptionFlag::NoAlternateScreen
                | OptionFlag::NoWinchSighandler
                | OptionFlag::NoQuitSighandlers,
        ))
    }

    // ----------------------------------------------------------^ CONSTRUCTORS

    // notcurses_at_yx

    /// Can we set the "hardware" palette?
    ///
    /// Requires the "ccc" terminfo capability.
    // TODO: TEST
    pub fn can_change_color(&self) -> bool {
        unsafe { nc::notcurses_canchangecolor(self.data) }
    }

    /// Can we fade?
    ///
    /// Requires either the "rgb" or "ccc" terminfo capability.
    // TODO: TEST
    pub fn can_fade(&self) -> bool {
        unsafe { nc::notcurses_canfade(self.data) }
    }

    /// Can we load images?
    ///
    /// Requires being built against FFmpeg/OIIO.
    // TODO: TEST
    pub fn can_open_images(&self) -> bool {
        unsafe { nc::notcurses_canopen_images(self.data) }
    }

    /// Can we load videos?
    ///
    /// Requires being built against FFmpeg.
    // TODO: TEST
    pub fn can_open_videos(&self) -> bool {
        unsafe { nc::notcurses_canopen_videos(self.data) }
    }

    /// Can we blit to Sixel?
    // TODO: TEST
    pub fn can_sixel(&self) -> bool {
        unsafe { nc::notcurses_cansixel(self.data) }
    }

    /// Can we directly specify RGB values per cell?
    ///
    /// If not, we can only use palettes.
    // TODO: TEST
    pub fn can_truecolor(&self) -> bool {
        unsafe { nc::notcurses_cantruecolor(self.data) }
    }

    /// Is our encoding UTF-8?
    ///
    /// Requires LANG being set to a UTF8 locale.
    // TODO: TEST
    pub fn can_utf8(&self) -> bool {
        unsafe { nc::notcurses_canutf8(self.data) }
    }

    /// Disables the cursor
    // TODO: TEST
    pub fn cursor_disable(&mut self) {
        unsafe {
            nc::notcurses_cursor_disable(self.data);
        }
    }

    /// Enables the cursor
    // TODO: TEST
    pub fn cursor_enable(&mut self) {
        unsafe {
            nc::notcurses_cursor_enable(self.data);
        }
    }

    /// Destroy all ncplanes other than the stdplane.
    // TODO: TEST
    pub fn drop_planes(&mut self) {
        unsafe {
            nc::notcurses_drop_planes(self.data);
        }
    }

    /// Returns the standard Plane for the current context
    ///
    // [man notcurses_stdplane](https://nick-black.com/notcurses/notcurses_stdplane.3.html)
    // It is an error to call ncplane_destroy, ncplane_resize, or ncplane_move
    // on the standard plane, but it can be freely moved along the z-axis.
    //
    pub fn stdplane(&mut self) -> Plane {
        unsafe { Plane::new_from(nc::notcurses_stdplane(self.data)) }
    }

    /// Returns a flag that indicates the supported styles for the current terminal
    // TODO: TEST
    pub fn supported_styles(&self) -> u32 {
        unsafe { nc::notcurses_supported_styles(self.data) }
    }

    /// Returns the name of the flags supported
    // TODO: TEST
    pub fn supported_styles_str(&self) -> String {
        let sf = self.supported_styles();
        let mut sstr = String::new();

        for s in Style::iter() {
            if s as u32 & sf != 0 {
                sstr = format! {"{} {:?}", sstr, s};
            }
        }
        sstr.trim().to_owned()
    }
}

impl Drop for Notcurses {
    fn drop(&mut self) {
        // It is important to reset the terminal before exiting, whether terminating due to intended operation
        // or a received signal. This is usually accomplished by explicitly calling notcurses_stop.
        // For convenience, notcurses by default installs signal handlers for various signals typically resulting
        // in process termination (see signal(7)). These signal handlers call notcurses_stop for each struct notcurses
        // in the process, and then propagate the signal to any previously-configured handler.
        // These handlers are disabled upon entry to notcurses_stop
        //
        // [API](https://nick-black.com/notcurses/notcurses_stop.3.html)
        unsafe {
            nc::notcurses_stop(self.data);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /* MODEL
    #[test]
    fn () -> Result<(), Error> {
        let mut nc = Notcurses::for_testing();
        let plane = Plane::new(&mut nc, 50, 100, 0, 0);
        assert_eq!(, );
    }
    Ok(())
    */

    #[test]
    fn new() -> Result<(), Error> {
        let _ = Notcurses::new()?;
        Ok(())
    }

    #[test]
    fn for_testing() -> Result<(), Error> {
        let _ = Notcurses::for_testing()?;
        Ok(())
    }

    #[test]
    fn stdplane() -> Result<(), Error> {
        let mut nc = Notcurses::for_testing()?;
        let _p = nc.stdplane();
        Ok(())
    }
}
