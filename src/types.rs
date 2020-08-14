use libnotcurses_sys as nc;

use enumflags2::BitFlags;

pub use nc::Rgb;
pub use nc::ChannelPair;


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
