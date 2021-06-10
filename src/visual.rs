//!

use crate::sys::NcVisual;

/// A virtual pixel framebuffer.
pub struct Visual<'a> {
    pub(crate) raw: &'a mut NcVisual,
}

impl<'a> Drop for Visual<'a> {
    /// Destroys the Visual.
    ///
    /// Rendered elements will not be disrupted, but the visual can be neither
    /// decoded nor rendered any further.
    fn drop(&mut self) {
        let _ = self.raw.destroy();
    }
}

impl<'a> Visual<'a> {
    pub fn from_rgba() {}

    pub fn from_bgra() {}

    pub fn from_file() {}

    pub fn from_plane() {}
}
