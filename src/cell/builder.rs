use crate::{sys::NcCell, Cell, Channels, NError, NResult, Plane, Style};

/// A [`Cell`] builder.
pub struct CellBuilder<'plane, 'ncplane> {
    ch: char,
    // backstop: 0,
    // width: u8,
    style: Style,
    channels: Channels,

    plane: Option<&'plane mut Plane<'ncplane>>,
}

impl<'plane, 'ncplane> Default for CellBuilder<'plane, 'ncplane> {
    fn default() -> Self {
        Self {
            ch: ' ',
            //width: 0,
            style: Style::default(),
            channels: Channels::default(),
            plane: None,
        }
    }
}

impl<'plane, 'ncplane> CellBuilder<'plane, 'ncplane> {
    /// Sets the [`Style`].
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the [`Channels`].
    pub fn channels(mut self, channels: Channels) -> Self {
        self.channels = channels;
        self
    }

    /// Sets a [`char`].
    pub fn char(mut self, ch: char) -> Self {
        self.ch = ch;
        self
    }

    /// A `Plane` is needed to store more than a 7-bit ASCII `char`s.
    pub fn plane(mut self, plane: &'plane mut Plane<'ncplane>) -> Self {
        self.plane = Some(plane);
        self
    }

    /// Finishes the build and returns a [`Cell`].
    // WIP
    pub fn finish(self) -> NResult<Cell> {
        if self.ch as u32 > 127 {
            if let Some(_plane) = self.plane {
                // TEMP
                let nccell = NcCell::from_char7b(self.ch);
                Ok(Cell { nccell })
            } else {
                Err(NError::BuildIncomplete(
                    "Cells with characters bigger than
                    7-bit ASCII needs to have a Plane assigned."
                        .into(),
                ))
            }
        } else {
            // TEMP
            let nccell = NcCell::from_char7b(self.ch);
            Ok(Cell { nccell })
        }
    }
}
