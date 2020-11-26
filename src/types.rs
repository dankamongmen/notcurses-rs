use enumflags2::BitFlags;

use crate::sys;

// Reexports
//
// TODO: wrap them up in new types
pub use sys::{NcChannels, NcRgb};

/// Alignment within a plane or terminal. Left/right-justified, or centered.
///
/// [C sourcecode](https://nick-black.com/notcurses/html/notcurses_8h_source.html#l00063)
#[repr(u32)] // = ncalign_e
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Align {
    Left = sys::NCALIGN_LEFT as sys::NcAlign,
    Center = sys::NCALIGN_CENTER as sys::NcAlign,
    Right = sys::NCALIGN_RIGHT as sys::NcAlign,
}

/// Blitter Modes
///
/// NOTE: Blitter::_1x1x4 & Blitter::_4x1 are still unimplemented,
/// they both ought be falling back to 1x1 with a top half.
// each has the empty cell in addition to the product of its dimensions. i.e.
// NCBLIT_1x1 has two states: empty and full block. NCBLIT_1x1x4 has five
// states: empty, the three shaded blocks, and the full block.
#[repr(u32)] // = ncblitter_e
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Blitter {
    /// full block                █
    _1x1 = sys::NCBLIT_1x1 as sys::NcBlitter,

    /// upper half + 1x1          ▀█
    _2x1 = sys::NCBLIT_2x1 as sys::NcBlitter,

    /// quadrants + 2x1           ▗▐ ▖▀▟▌▙█
    _2x2 = sys::NCBLIT_2x2 as sys::NcBlitter,

    /// sextants 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
    _3x2 = sys::NCBLIT_3x2 as sys::NcBlitter,

    /// four vertical levels      █▆▄▂
    _4x1 = sys::NCBLIT_4x1 as sys::NcBlitter,

    /// eight vertical levels     █▇▆▅▄▃▂▁
    _8x1 = sys::NCBLIT_8x1 as sys::NcBlitter,

    /// 4 rows, 2 cols (braille)  ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    Braille = sys::NCBLIT_BRAILLE as sys::NcBlitter,

    /// let the ncvisual pick
    Default = sys::NCBLIT_DEFAULT as sys::NcBlitter,

    /// 6 rows, 1 col (RGB), spotty support among terminals
    Sixel = sys::NCBLIT_SIXEL as sys::NcBlitter,
}

/// Log levels
///
/// By default, nothing is printed to stderr once fullscreen service begins.
/// Progressively higher log levels result in more logging to stderr:
#[repr(u32)] // = ncloglevel_e
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LogLevel {
    Silent = sys::NCLOGLEVEL_SILENT as sys::NcLogLevel,
    Panic = sys::NCLOGLEVEL_PANIC as sys::NcLogLevel,
    Fatal = sys::NCLOGLEVEL_FATAL as sys::NcLogLevel,
    Error = sys::NCLOGLEVEL_ERROR as sys::NcLogLevel,
    Warning = sys::NCLOGLEVEL_WARNING as sys::NcLogLevel,
    Info = sys::NCLOGLEVEL_INFO as sys::NcLogLevel,
    Debug = sys::NCLOGLEVEL_DEBUG as sys::NcLogLevel,
    Trace = sys::NCLOGLEVEL_TRACE as sys::NcLogLevel,
}

/// Scale
#[repr(u32)] // = ncscale_e
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Scale {
    None = sys::NCSCALE_NONE as sys::NcScale,
    Scale = sys::NCSCALE_SCALE as sys::NcScale,
    Stretch = sys::NCSCALE_STRETCH as sys::NcScale,
}

/// Style Flags
#[repr(u32)]
#[derive(BitFlags, EnumIter, Copy, Clone, Debug, PartialEq)]
pub enum Style {
    Blink = sys::NCSTYLE_BLINK as u32,
    Bold = sys::NCSTYLE_BOLD as u32,
    Dim = sys::NCSTYLE_DIM as u32,
    Invis = sys::NCSTYLE_INVIS as u32,
    Italic = sys::NCSTYLE_ITALIC as u32,
    Protect = sys::NCSTYLE_PROTECT as u32,
    Reverse = sys::NCSTYLE_REVERSE as u32,
    Standout = sys::NCSTYLE_STANDOUT as u32,
    Underline = sys::NCSTYLE_UNDERLINE as u32,
    Struck = sys::NCSTYLE_STRUCK as u32,
    // Mask = sys::NCSTYLE_MASK as u32, // 16 first bits set
    // None = sys::NCSTYLE_NONE as u32, // Equals 0
}
