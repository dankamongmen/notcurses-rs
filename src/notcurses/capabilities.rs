//!

use crate::{
    visual::{Blitter, PixelImplementation},
    Notcurses,
};

/// The detected current terminal capabilities.
///
/// It can be generated from
/// [`Notcurses.capabilities()`][crate::Notcurses#method.capabilities].
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capabilities {
    pub(crate) utf8: bool,
    pub(crate) halfblock: bool,
    pub(crate) quadrant: bool,
    pub(crate) sextant: bool,
    pub(crate) braille: bool,
    pub(crate) pixel: bool,
    pub(crate) pixel_implementation: PixelImplementation,
    pub(crate) images: bool,
    pub(crate) videos: bool,
    pub(crate) fade: bool,
    pub(crate) truecolor: bool,
    pub(crate) palette_size: u32,
    pub(crate) palette_change: bool,
}

mod core_impls {
    use super::Capabilities;
    use core::fmt::{self, Write as _};

    impl fmt::Display for Capabilities {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut string = String::new();
            if self.utf8 {
                string += "utf8 "
            }
            if self.halfblock {
                string += "halfblock "
            }
            if self.quadrant {
                string += "quadrant "
            }
            if self.sextant {
                string += "sextant "
            }
            if self.braille {
                string += "braille "
            }
            if self.pixel {
                let _ = write![string, "pixel:{} ", self.pixel_implementation];
            }
            if self.images {
                string += "images "
            }
            if self.videos {
                string += "videos "
            }
            if self.fade {
                string += "fade "
            }
            if self.truecolor {
                string += "rgb "
            }
            let _ = write![string, "palette:{} ", self.palette_size];
            if self.palette_change {
                string += "palchange "
            }
            let _ = string.pop();
            write!(f, "{}", string)
        }
    }
}

impl Capabilities {
    /// New `Capabilities` from a [`Notcurses`] context.
    pub fn new(nc: &Notcurses) -> Self {
        nc.capabilities()
    }

    /// Returns the best Blitter available, using the following rules
    /// of *graceful degradation*:
    ///
    /// [`Pixel`] > [`Sextant`] > [`Quadrant`] > [`Half`] > [`Ascii`].
    ///
    /// [`Pixel`]: crate::sys::NcBlitter#variant.Pixel
    /// [`Sextant`]: crate::sys::NcBlitter#variant.Sextant
    /// [`Quadrant`]: crate::sys::NcBlitter#variant.Quadrant
    /// [`Half`]: crate::sys::NcBlitter#variant.Half
    /// [`Ascii`]: crate::sys::NcBlitter#variant.Ascii
    pub fn best_blitter(&self) -> Blitter {
        if self.pixel {
            Blitter::Pixel
        } else if self.sextant {
            Blitter::Sextant
        } else if self.quadrant {
            Blitter::Quadrant
        } else if self.halfblock {
            Blitter::Half
        } else {
            Blitter::Ascii
        }
    }

    /// Returns `true` if the provided [`Blitter`] is among the capabilities.
    pub fn can_blitter(&self, blitter: Blitter) -> bool {
        match blitter {
            Blitter::Default => true,
            Blitter::Ascii => true,
            Blitter::Half => self.halfblock,
            Blitter::Quadrant => self.quadrant,
            Blitter::Sextant => self.sextant,
            Blitter::Braille => self.braille,
            Blitter::Pixel => self.pixel,
            Blitter::_4x1 => self.utf8,
            Blitter::_8x1 => self.utf8,
        }
    }

    /// Returns *true* if we can reliably use Unicode half blocks.
    pub fn halfblock(&self) -> bool {
        self.halfblock
    }

    /// Returns *true* if we can reliably use Unicode quadrant blocks.
    pub fn quadrant(&self) -> bool {
        self.quadrant
    }

    /// Returns *true* if we can reliably use Unicode sextant blocks.
    pub fn sextant(&self) -> bool {
        self.sextant
    }

    /// Returns *true* if we can reliably use Unicode Braille.
    pub fn braille(&self) -> bool {
        self.braille
    }

    /// Returns *true* if the encoding is UTF-8.
    pub fn utf8(&self) -> bool {
        self.utf8
    }

    /// Returns *true* if loading images is possible.
    ///
    /// This requires that notcurse is built against FFmpeg/OIIO.
    pub fn images(&self) -> bool {
        self.images
    }

    /// Returns *true* if loading videos is possible.
    ///
    /// This requires that notcurse is built against FFmpeg/OIIO.
    pub fn videos(&self) -> bool {
        self.videos
    }

    /// Returns *true* if we can blit pixel-accurate bitmaps.
    pub fn pixel(&self) -> bool {
        self.pixel
    }

    /// Returns the detected pixel-blitting implementation.
    pub const fn pixel_implementation(&self) -> PixelImplementation {
        self.pixel_implementation
    }

    /// Returns *true* if fading is possible.
    pub fn fade(&self) -> bool {
        self.fade
    }

    /// Returns *true* if it's possible to directly specify RGB values per Cell,
    /// or *false* if it's only possible to use palettes.
    pub fn truecolor(&self) -> bool {
        self.truecolor
    }

    /// Returns *true* if the "hardware" palette can be changed.
    ///
    /// Requires the "ccc" terminfo capability, and that the number of colors
    /// supported is at least the size of the `Palette` structure.
    pub fn palette_change(&self) -> bool {
        self.palette_change
    }

    /// Returns the number of simultaneous colors claimed to be supported,
    /// if there is color support.
    ///
    /// Note that several terminal emulators advertise more colors than they
    /// actually support, downsampling internally.
    pub fn palette_size(&self) -> u32 {
        self.palette_size
    }
}
