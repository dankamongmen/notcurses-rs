// #![allow(dead_code)]

use crate::sys::NcPixelImpl;

/// A `u8` of pixel blitting implementation. (Informative only)
//
// data type in C: u32
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PixelImpl {
    /// No pixel support.
    None = NcPixelImpl::None as u8,
    /// Sixel
    Sixel = NcPixelImpl::Sixel as u8,
    /// Linux framebuffer.
    LinuxFb = NcPixelImpl::LinuxFb as u8,
    /// iTerm2
    Iterm2 = NcPixelImpl::Iterm2 as u8,
    /// Kitty prior to C=1 and animation.
    KittyStatic = NcPixelImpl::KittyStatic as u8,
    /// Kitty with animation but not reflexive composition.
    KittyAnimated = NcPixelImpl::KittyAnimated as u8,
    /// Kitty with reflexive composition.
    KittySelfRef = NcPixelImpl::KittySelfRef as u8,
}

impl Default for PixelImpl {
    fn default() -> Self {
        PixelImpl::None
    }
}

impl From<PixelImpl> for NcPixelImpl {
    fn from(pi: PixelImpl) -> NcPixelImpl {
        pi as NcPixelImpl
    }
}

/// Any value that is not a valid [`NcPixelImpl`] related constant
/// will be converted to [`PixelImpl::None`].
impl From<NcPixelImpl> for PixelImpl {
    fn from(pi: NcPixelImpl) -> PixelImpl {
        match pi {
            NcPixelImpl::NOPIXEL => PixelImpl::None,
            NcPixelImpl::SIXEL => PixelImpl::Sixel,
            NcPixelImpl::LINUXFB => PixelImpl::LinuxFb,
            NcPixelImpl::ITERM2 => PixelImpl::Iterm2,
            NcPixelImpl::KITTY_STATIC => PixelImpl::KittyStatic,
            NcPixelImpl::KITTY_ANIMATED => PixelImpl::KittyAnimated,
            NcPixelImpl::KITTY_SELFREF => PixelImpl::KittySelfRef,
            _ => PixelImpl::None,
        }
    }
}
