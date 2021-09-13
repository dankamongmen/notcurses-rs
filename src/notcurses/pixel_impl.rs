// #![allow(dead_code)]

use crate::sys::{self, NcPixelImpl};

/// A `u8` of pixel blitting implementation. (Informative only)
//
// data type in C: u32
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum PixelImpl {
    /// No pixel support.
    None = crate::sys::ffi::ncpixelimpl_e_NCPIXEL_NONE as u8,
    /// Sixel
    Sixel = crate::sys::ffi::ncpixelimpl_e_NCPIXEL_SIXEL as u8,
    /// Linux framebuffer.
    LinuxFb = crate::sys::ffi::ncpixelimpl_e_NCPIXEL_LINUXFB as u8,
    /// iTerm2
    Iterm2 = crate::sys::ffi::ncpixelimpl_e_NCPIXEL_ITERM2 as u8,
    /// Kitty prior to C=1 and animation.
    KittyStatic = crate::sys::ffi::ncpixelimpl_e_NCPIXEL_KITTY_STATIC as u8,
    /// Kitty with animation but not reflexive composition.
    KittyAnimated = crate::sys::ffi::ncpixelimpl_e_NCPIXEL_KITTY_ANIMATED as u8,
    /// Kitty with reflexive composition.
    KittySelfRef = crate::sys::ffi::ncpixelimpl_e_NCPIXEL_KITTY_SELFREF as u8,
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
            sys::NCPIXEL_NONE => PixelImpl::None,
            sys::NCPIXEL_SIXEL => PixelImpl::Sixel,
            sys::NCPIXEL_LINUXFB => PixelImpl::LinuxFb,
            sys::NCPIXEL_ITERM2 => PixelImpl::Iterm2,
            sys::NCPIXEL_KITTY_STATIC => PixelImpl::KittyStatic,
            sys::NCPIXEL_KITTY_ANIMATED => PixelImpl::KittyAnimated,
            sys::NCPIXEL_KITTY_SELFREF => PixelImpl::KittySelfRef,
            _ => PixelImpl::None,
        }
    }
}
