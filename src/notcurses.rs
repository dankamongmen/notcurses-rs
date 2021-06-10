//! `Nc` wrapper struct and traits implementations.

use crate::{sys::Notcurses, Dimension, Result};

/// The main **notcurses** context.
///
/// A wrapper around `sys::`[`Notcurses`].
#[derive(Debug)]
pub struct Nc<'a> {
    pub(crate) raw: &'a mut Notcurses,
}

impl<'a> Drop for Nc<'a> {
    /// Destroys the Nc context.
    fn drop(&mut self) {
        let _ = self.raw.stop();
    }
}

impl<'a> Nc<'a> {
    /// New Notcurses instance.
    pub fn new() -> Result<Self> {
        Ok(Self {
            raw: Notcurses::new()?,
        })
    }

    ///
    pub fn termsize(&self) -> (Dimension, Dimension) {
        self.raw.term_dim_yx()
    }
}
