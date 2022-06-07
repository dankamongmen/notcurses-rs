// notcurses::visual::pixel
//
//!
//

/// Pixel blitting implementations, informative only.
///
/// This is returned by [`Capabilities.pixel_implementation`].
///
/// [`Capabilities.pixel_implementation`]: crate::Capabilities#method.pixel_implementation
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PixelImplementation {
    /// No pixel support.
    ///
    /// This is the default.
    None,

    /// Sixel.
    Sixel,

    /// Linux framebuffer.
    LinuxFb,

    /// iTerm2.
    Iterm2,

    /// Kitty prior to C=1 and animation.
    KittyStatic,

    /// Kitty with animation but not reflexive composition.
    KittyAnimated,

    /// Kitty with reflexive composition.
    KittySelfRef,
}

mod std_impls {
    use super::PixelImplementation;
    use crate::sys::{c_api::NcPixelImpl_u32, NcPixelImpl};
    use std::fmt;

    impl Default for PixelImplementation {
        fn default() -> Self {
            Self::None
        }
    }

    impl fmt::Display for PixelImplementation {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use PixelImplementation::*;
            write!(
                f,
                "{}",
                match self {
                    PixelImplementation::None => "None",
                    Sixel => "Sixel",
                    LinuxFb => "LinuxFb",
                    Iterm2 => "Iterm2",
                    KittyStatic => "KittyStatic",
                    KittyAnimated => "KittyAnimated",
                    KittySelfRef => "KittySelfRef",
                }
            )
        }
    }

    impl fmt::Debug for PixelImplementation {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Pixel::{}", self)
        }
    }

    //

    impl From<NcPixelImpl> for PixelImplementation {
        fn from(nc: NcPixelImpl) -> PixelImplementation {
            match nc {
                NcPixelImpl::None => PixelImplementation::None,
                NcPixelImpl::Sixel => PixelImplementation::Sixel,
                NcPixelImpl::LinuxFb => PixelImplementation::LinuxFb,
                NcPixelImpl::Iterm2 => PixelImplementation::Iterm2,
                NcPixelImpl::KittyStatic => PixelImplementation::KittyStatic,
                NcPixelImpl::KittyAnimated => PixelImplementation::KittyAnimated,
                NcPixelImpl::KittySelfRef => PixelImplementation::KittySelfRef,
                _ => PixelImplementation::default(),
            }
        }
    }
    impl From<PixelImplementation> for NcPixelImpl {
        fn from(pi: PixelImplementation) -> NcPixelImpl {
            match pi {
                PixelImplementation::None => NcPixelImpl::None,
                PixelImplementation::Sixel => NcPixelImpl::Sixel,
                PixelImplementation::LinuxFb => NcPixelImpl::LinuxFb,
                PixelImplementation::Iterm2 => NcPixelImpl::Iterm2,
                PixelImplementation::KittyStatic => NcPixelImpl::KittyStatic,
                PixelImplementation::KittyAnimated => NcPixelImpl::KittyAnimated,
                PixelImplementation::KittySelfRef => NcPixelImpl::KittySelfRef,
            }
        }
    }

    impl From<NcPixelImpl_u32> for PixelImplementation {
        fn from(ncu: NcPixelImpl_u32) -> PixelImplementation {
            NcPixelImpl::from(ncu).into()
        }
    }
    impl From<PixelImplementation> for NcPixelImpl_u32 {
        fn from(pi: PixelImplementation) -> NcPixelImpl_u32 {
            NcPixelImpl::from(pi).into()
        }
    }
}
