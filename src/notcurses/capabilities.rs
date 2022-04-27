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
    pub(crate) cursor: bool,
}

impl Capabilities {
    /// New `Capabilities` from a [`Notcurses`] context.
    pub fn new(nc: &Notcurses) -> Self {
        nc.capabilities()
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

    ///
    pub fn halfblock(&self) -> bool {
        self.halfblock
    }

    ///
    pub fn quadrant(&self) -> bool {
        self.quadrant
    }

    ///
    pub fn sextant(&self) -> bool {
        self.sextant
    }

    ///
    pub fn braille(&self) -> bool {
        self.braille
    }

    ///
    pub fn utf8(&self) -> bool {
        self.utf8
    }

    ///
    pub fn images(&self) -> bool {
        self.images
    }

    ///
    pub fn videos(&self) -> bool {
        self.videos
    }

    ///
    pub fn pixel(&self) -> bool {
        self.pixel
    }

    ///
    pub fn pixel_impl(&self) -> NcPixelImpl {
        self.pixel_impl
    }

    ///
    pub fn fade(&self) -> bool {
        self.fade
    }

    ///
    pub fn truecolor(&self) -> bool {
        self.truecolor
    }

    ///
    pub fn palette_change(&self) -> bool {
        self.palette_change
    }

    ///
    pub fn palette_size(&self) -> u32 {
        self.palette_size
    }

    ///
    pub fn cursor(&self) -> bool {
        self.cursor
    }
}
