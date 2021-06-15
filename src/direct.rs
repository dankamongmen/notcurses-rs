//! `NotcursesDirect` wrapper struct and traits implementations.

use crate::sys::NcDirect;

/// A minimal notcurses instance for styling text.
///
/// A wrapper around `sys::`[`NcDirect`].
#[derive(Debug)]
pub struct NotcursesDirect<'a> {
    pub(crate) raw: &'a mut NcDirect,
}

impl<'a> Drop for NotcursesDirect<'a> {
    /// Destroys the NotcursesDirect context.
    fn drop(&mut self) {
        let _ = self.raw.stop();
    }
}

/// # Constructors
impl<'a> NotcursesDirect<'a> {}
