use enumflags2::BitFlags;

use libnotcurses_sys as nc;

pub use nc::Rgb;
pub use nc::ChannelPair;

#[repr(u32)] // = ncalign_e
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Align {
    Left = nc::ncalign_e_NCALIGN_LEFT as nc::ncalign_e,
    Center = nc::ncalign_e_NCALIGN_CENTER as nc::ncalign_e,
    Right = nc::ncalign_e_NCALIGN_RIGHT as nc::ncalign_e,
}

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
    _1x1 = nc::ncblitter_e_NCBLIT_1x1 as nc::ncblitter_e,

    /// shaded full blocks        ▓▒░█
    _1x1x4 = nc::ncblitter_e_NCBLIT_1x1x4 as nc::ncblitter_e,

    /// upper half + 1x1          ▀█
    _2x1 = nc::ncblitter_e_NCBLIT_2x1 as nc::ncblitter_e,

    /// quadrants + 2x1           ▗▐ ▖▀▟▌▙█
    _2x2 = nc::ncblitter_e_NCBLIT_2x2 as nc::ncblitter_e,

    /// four vertical levels      █▆▄▂
    _4x1 = nc::ncblitter_e_NCBLIT_4x1 as nc::ncblitter_e,

    /// eight vertical levels     █▇▆▅▄▃▂▁
    _8x1 = nc::ncblitter_e_NCBLIT_8x1 as nc::ncblitter_e,

    /// 4 rows, 2 cols (braille)  ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    Braille = nc::ncblitter_e_NCBLIT_BRAILLE as nc::ncblitter_e,

    /// let the ncvisual pick
    Default = nc::ncblitter_e_NCBLIT_DEFAULT as nc::ncblitter_e,

    /// 6 rows, 1 col (RGB), spotty support among terminals
    Sixel = nc::ncblitter_e_NCBLIT_SIXEL as nc::ncblitter_e,
}

#[repr(u32)] // = ncscale_e
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Scale {
    None = nc::ncscale_e_NCSCALE_NONE as nc::ncscale_e,
    Scale = nc::ncscale_e_NCSCALE_SCALE as nc::ncscale_e,
    Stretch = nc::ncscale_e_NCSCALE_STRETCH as nc::ncscale_e,
}

/// Style Flags
#[repr(u32)]
#[derive(BitFlags, EnumIter, Copy, Clone, Debug, PartialEq)]
pub enum Style {
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
