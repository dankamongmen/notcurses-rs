use crate::{sys::NcCell, Plane, Style};

mod builder;
pub use builder::CellBuilder;

/// Part of a [`Cell`].
pub const BACKSTOP: u8 = 0;

/// A `u128` of [`char`] + [`BACKSTOP`] + *width* + [`Style`] + [`Channels`][crate::Channels],
/// part of a [`Plane`][crate::Plane].
///
/// # Diagram
/// ```txt
/// CCCCCCCC CCCCCCCC CCCCCCCC CCCCCCCC  char
/// BBBBBBBB WWWWWWWW 11111111 11111111  BACKSTOP + width + Style
/// ~~AA~~~~ RRRRRRRR GGGGGGGG BBBBBBBB  Foreground Channel
/// ~~AA~~~~ RRRRRRRR GGGGGGGG BBBBBBBB  Background Channel
/// ```
pub struct Cell {
    nccell: NcCell,
}

impl<'plane, 'ncplane> Cell {
    /// Returns a [`CellBuilder`] used to customize a new `Cell`.
    pub fn build() -> CellBuilder<'plane, 'ncplane> {
        CellBuilder::default()
    }

    /// Returns the `char`.
    pub fn char(&mut self, plane: &mut Plane<'ncplane>) -> char {
        self.nccell.egc(plane.as_ncplane_mut())
    }

    /// Returns the [`Style`]s.
    pub fn styles(&mut self) -> Style {
        self.nccell.styles().into()
    }

    /// Adds the specified [`Style`]s.
    pub fn add_styles(&mut self, styles: Style) {
        self.nccell.styles_on(styles.bits())
    }

    /// Deletes the specified [`Style`]s.
    pub fn del_styles(&mut self, styles: Style) {
        self.nccell.styles_off(styles.bits())
    }

    /// Sets just the specified [`Style`]s.
    pub fn set_styles(&mut self, styles: Style) {
        self.nccell.styles_set(styles.bits())
    }

    // pub fn channels(&mut self) -> Channels {
    //     self.0.channels()
    // }
}
