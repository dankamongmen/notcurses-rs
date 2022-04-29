//!

use crate::{sys::NcPixelImpl, Blitter, Notcurses};

/// The detected current terminal capabilities.
///
/// It can also be generated from
/// [`Notcurses.capabilities`][crate::Notcurses#method.capabilities] and
#[derive(Clone, Copy, Debug)]
pub struct Capabilities {
    pub(crate) halfblock: bool,
    pub(crate) quadrant: bool,
    pub(crate) sextant: bool,
    pub(crate) braille: bool,
    pub(crate) utf8: bool,
    pub(crate) images: bool,
    pub(crate) videos: bool,
    pub(crate) pixel: bool,
    pub(crate) pixel_impl: NcPixelImpl,
    pub(crate) fade: bool,
    pub(crate) truecolor: bool,
    pub(crate) palette_change: bool,
    pub(crate) palette_size: u32,
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

    /// Returns true if the provided [`Blitter`] is among the capabilities.
    pub fn can_blitter(&self, blitter: Blitter) -> bool {
        use Blitter::*;
        match blitter {
            Default => true,
            Ascii => true,
            Half => self.halfblock,
            Quadrant => self.quadrant,
            Sextant => self.sextant,
            Braille => self.braille,
            Pixel => self.pixel,
            _4x1 => self.utf8,
            _8x1 => self.utf8,
            _ => false,
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

    /// Returns the detected pixel-blitting mechanism.
    pub fn pixel_impl(&self) -> NcPixelImpl {
        self.pixel_impl
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

    ///
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
