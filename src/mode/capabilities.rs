//!

use crate::{Notcurses, NotcursesDirect};

/// The detected terminal capabilities.
///
/// It can be generated with
/// [`Notcurses.capabilities`][crate::Notcurses#method.capabilities] and
/// [`NotcursesDirect.capabilities`][crate::NotcursesDirect#method.capabilities]
pub struct Capabilities {
    pub(crate) halfblock: bool,
    pub(crate) quadrant: bool,
    pub(crate) sextant: bool,
    pub(crate) braille: bool,
    pub(crate) utf8: bool,
    pub(crate) images: bool,
    pub(crate) videos: bool,
    pub(crate) pixel: bool,
    pub(crate) fade: bool,
    pub(crate) truecolor: bool,
    pub(crate) palette_change: bool,
    pub(crate) palette_size: u32,
    pub(crate) cursor: bool,
}

impl Capabilities {
    /// New `Capabilities` from a [`Notcurses`] context.
    pub fn from_notcurses(nc: &Notcurses) -> Self {
        nc.capabilities()
    }

    /// New `Capabilities` from a [`NotcursesDirect`] context.
    pub fn from_notcurses_direct(ncd: &NotcursesDirect) -> Self {
        ncd.capabilities()
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
