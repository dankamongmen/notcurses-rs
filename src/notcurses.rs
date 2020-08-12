use std::ptr::{null, null_mut};

use libnotcurses_sys as nc;

use enumflags2::BitFlags;
use strum::IntoEnumIterator;

use crate::error::NcError;
use crate::plane::NcPlane;

/// Log levels
///
/// By default, nothing is printed to stderr once fullscreen service begins.
/// Progressively higher log levels result in more logging to stderr:
#[repr(u32)] // = ncloglevel_e
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NcLogLevel {
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
pub enum NcOptionFlag {
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
// impl NcOptionFlag {
//     pub const EMPTY: BitFlags<NcOptionFlag> = BitFlags::empty();
// }

/// A safe wrapper over notcurses_options
///
/// notcurses_init accepts a struct notcurses_options allowing fine-grained control of notcurses behavior,
/// including signal handlers, alternative screens, and overriding the TERM environment variable.
/// A terminfo entry appropriate for the actual terminal must be available
pub struct NcOptions {
    pub(crate) data: nc::notcurses_options,
}

impl NcOptions {
    /// Returns a new options structure with no logging and no empty flags
    pub fn new_default() -> Self {
        Self::new(NcLogLevel::Silent, BitFlags::empty())
    }

    ///
    pub fn new(loglevel: NcLogLevel, flags: impl Into<BitFlags<NcOptionFlag>>) -> Self {
        NcOptions {
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

/// Style Flags
#[repr(u32)]
#[derive(BitFlags, EnumIter, Copy, Clone, Debug, PartialEq)]
pub enum NcStyle {
    Blink = nc::NCSTYLE_BLINK as u32,
    Bold = nc::NCSTYLE_BOLD as u32,
    Dim = nc::NCSTYLE_DIM as u32,
    Invis = nc::NCSTYLE_INVIS as u32,
    Italic = nc::NCSTYLE_ITALIC as u32,
    Protect = nc::NCSTYLE_PROTECT as u32,
    Reverse = nc::NCSTYLE_REVERSE as u32,
    Standout = nc::NCSTYLE_STANDOUT as u32,
    Underline = nc::NCSTYLE_UNDERLINE as u32,
    // Mask = nc::NCSTYLE_MASK as u32, // 16 first bits set
    // None = nc::NCSTYLE_NONE as u32, // Equals 0
}

extern "C" {
    // Needed for notcurses_init()
    fn libc_stdout() -> *mut nc::_IO_FILE;
}

/// A safe wrapper over a notcurses context
///
/// ## Links
/// - [man notcurses](https://nick-black.com/notcurses/notcurses.3.html)
pub struct NotCurses {
    pub(crate) data: *mut nc::notcurses,
}

impl NotCurses {

    /// Returns a NotCurses instance
    ///
    // TODO[LOW]: support custom locale
    // TODO[LOW]: support custom fn() in notcurses_init 2nd parameter
    //
    pub fn new(options: NcOptions) -> Result<Self, NcError> {
        unsafe {
            // Before calling into notcurses be sure to call setlocale with an appropriate UTF-8 LC_ALL locale. It is
            // appropriate to use setlocale(LC_ALL, ""), relying on the user to set the LANG environment variable.
            //
            // [docs.rs → libc::setlocale](https://docs.rs/libc/0.2.74/libc/fn.setlocale.html)
            //
            // let _ = setlocale(LC_ALL, CString::new("es_ES.UTF-8").unwrap().as_ptr()); // DEBUG TEST
            let _ = libc::setlocale(libc::LC_ALL, std::ffi::CString::new("").unwrap().as_ptr());
        }

        Ok(NotCurses {
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

    /// Returns a NotCurses instance perfect for unit tests
    pub(crate) fn new_default_test() -> Result<Self, NcError> {
        Self::new(NcOptions::new(
            NcLogLevel::Silent,
            NcOptionFlag::SuppressBanners
                | NcOptionFlag::NoAlternateScreen
                | NcOptionFlag::NoWinchSighandler
                | NcOptionFlag::NoQuitSighandlers,
        ))
    }

    /// Returns the standard NcPlane for the current context
    ///
    // [man notcurses_stdplane](https://nick-black.com/notcurses/notcurses_stdplane.3.html)
    // It is an error to call ncplane_destroy, ncplane_resize, or ncplane_move
    // on the standard plane, but it can be freely moved along the z-axis.
    //
    pub fn stdplane(&mut self) -> NcPlane {
        unsafe { NcPlane::new_from(nc::notcurses_stdplane(self.data)) }
    }

    /// Returns a flag that indicates the supported styles for the current terminal
    pub fn supported_styles(&self) -> u32 {
        unsafe { nc::notcurses_supported_styles(self.data) }
    }

    /// Returns the name of the flags supported
    pub fn supported_styles_str(&self) -> String {
        let sf = self.supported_styles();
        let mut sstr = String::new();

        for s in NcStyle::iter() {
	    if s as u32 & sf != 0 {
                sstr = format!{"{} {:?}", sstr, s};
            }
        }
        sstr.trim().to_owned()
    }
}

impl Drop for NotCurses {
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
    fn () -> Result<(), NcError> {
        let mut nc = NotCurses::new_default_test();
        let plane = NcPlane::new(&mut nc, 50, 100, 0, 0);
        assert_eq!(, );
    }
    Ok(())
    */

    #[test]
    fn new() -> Result<(), NcError> {
        let o = NcOptions::new(NcLogLevel::Silent, NcOptionFlag::SuppressBanners);
        let _ = NotCurses::new(o)?;
        Ok(())
    }

    #[test]
    fn new_default_test() -> Result<(), NcError> {
        let _ = NotCurses::new_default_test()?;
        Ok(())
    }

    #[test]
    fn stdplane() -> Result<(), NcError> {
        let mut nc = NotCurses::new_default_test()?;
        let _p = nc.stdplane();
        Ok(())
    }
}

