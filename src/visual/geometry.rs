// notcurses::visual::geometry
//
//!
//

use crate::{sys::NcVisualGeometry, Blitter, Notcurses, PlaneGeometry};

/// The geometry of a [`Visual`][crate::Visual].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct VisualGeometry {
    /// The when this was calculated
    //
    reference: PlaneGeometry,
}

mod std_impls {
    use crate::{sys::NcPixelGeometry, Blitter, Size, VisualGeometry};
    use std::fmt;

    // #[rustfmt::skip]
    // impl fmt::Debug for VisualGeometry {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     }
    // }
}

/// # constructors
impl VisualGeometry {}

/// # methods
impl VisualGeometry {}
