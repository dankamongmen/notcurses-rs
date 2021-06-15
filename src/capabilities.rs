//!

// TODO
// - direct mode capabilities
//   waiting for https://github.com/dankamongmen/notcurses/issues/1768
//
// MAYBE: export as a bitfield


/// A unified structure that contains the current terminal capabilities.
///
/// It is created by [`Nc.capabilities`][crate::Nc#method.capabilities]
///
// and [`NcD.capabilities`][crate::NcD#method.capabilities].
pub struct Capabilities {
    pub halfblock: bool,
    pub quadrant: bool,
    pub sextant: bool,
    pub braille: bool,
    pub utf8: bool,
    pub images: bool,
    pub videos: bool,
    pub pixel: bool,
    pub fade: bool,
    pub truecolor: bool,
    pub palette_change: bool,
    pub palette_size: u32,
}

// impl Capabilities {
//     pub fn update(&mut self, &Nc) {
//
//     }
// }
