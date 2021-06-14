//!

use crate::sys;

bitflags! {
    /// How to scale a [`Visual`] during rendering.
    ///
    /// See also: [sys::NcScale].
    pub struct Scale: u32 {
        /// Maintains original size.
        const NONE = sys::NCSCALE_NONE;

        /// Maintains aspect ratio.
        const SCALE = sys::NCSCALE_SCALE;

        /// Throws away aspect ratio.
        const STRETCH = sys::NCSCALE_STRETCH;

        /// Maintains original size, admitting high-resolution blitters
        /// that don't preserve aspect ratio.
        const NONE_HIRES = sys::NCSCALE_NONE_HIRES;

        /// Maintains aspect ratio, admitting high-resolution blitters
        /// that don't preserve aspect ratio.
        const SCALE_HIRES = sys::NCSCALE_SCALE_HIRES;
    }
}
