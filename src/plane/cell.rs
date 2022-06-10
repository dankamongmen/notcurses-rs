// notcurses::plane::cell
//
//!
//

use crate::{
    color::{Alpha, Channel, Channels},
    error::Result,
    plane::{Plane, Style},
    sys::NcCell,
};

/// A `Cell` corresponds to a single *[grapheme cluster]* on some [`Plane`],
///
/// A `Cell` is bounded to n `Plane`, but the cell doesn't store anything
/// about the plane.
///
/// At any `NcCell`, we can have a theoretically arbitrarily long UTF-8 string,
/// a foreground color, a background color, and a [`Style`] attribute set.
///
/// [grapheme cluster]: http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Cell {
    nc: NcCell,
}

mod std_impls {
    use super::{Cell, Channels, NcCell, Style};
    use std::fmt;

    impl fmt::Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let width = self.nc.width;
            let egc = if let Some(s) = self.try_egc() {
                format!["\"{s}\""]
            } else {
                "&ref".into()
            };

            let style = Style::from(self.nc.stylemask);
            let channels = Channels::from(self.nc.channels);

            write!(f, "{egc} {width} {style} {channels}")
        }
    }
    impl fmt::Debug for Cell {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let width = self.nc.width;
            let egc = if let Some(s) = self.try_egc() {
                format!["\"{s}\""]
            } else {
                "&ref".into()
            };

            let style = Style::from(self.nc.stylemask);
            let channels = Channels::from(self.nc.channels);

            write!(f, "Cell {{{egc:} width:{width} {style:?} {channels}}}")
        }
    }

    impl From<NcCell> for Cell {
        fn from(nc: NcCell) -> Cell {
            Self { nc }
        }
    }

    impl From<Cell> for NcCell {
        fn from(c: Cell) -> NcCell {
            c.nc
        }
    }
    impl<'c> From<&'c Cell> for &'c NcCell {
        fn from(c: &'c Cell) -> &'c NcCell {
            &c.nc
        }
    }
}

/// # constructors
impl Cell {
    /// Creates an empty cell.
    pub fn new() -> Cell {
        NcCell::new().into()
    }

    /// Returns a Cell from a string.
    ///
    /// It only stores the first extended grapheme cluster from the string.
    pub fn from_str(plane: &mut Plane, string: &str) -> Result<Cell> {
        Ok(NcCell::from_str(plane.into_ref_mut(), string)?.into())
    }
}

/// # *egc* methods
impl Cell {
    /// Returns `true` if the egc is stored in the associated plane's *egc pool*,
    /// or `false` if the egc is stored entirely within the cell,
    ///
    /// Egcs of up to 4 bytes are stored in the cell.
    #[inline]
    pub const fn uses_egcpool(&self) -> bool {
        // If the first byte is 0x01, the rest is a 24-bit adress to an egcpool
        self.nc.gcluster >> 24 == 0x01
    }

    /// Returns the extended grapheme cluster only if it's stored in the cell.
    ///
    /// Returns none if the egc is stored in the associated plane.
    pub fn try_egc(&self) -> Option<String> {
        if self.uses_egcpool() {
            None
        } else {
            let bytes = self.nc.gcluster.to_ne_bytes();
            let no_nuls = bytes.split(|b| *b == 0).next().unwrap();
            std::str::from_utf8(no_nuls).ok().map(|s| s.to_string())
        }
    }

    /// Returns a reference to the *egc*.
    pub fn egc(&self, plane: &mut Plane) -> &str {
        self.nc.egc(plane.into_ref_mut())
    }
}

/// # channel methods
impl Cell {
    /// Gets the channels.
    pub fn channels(&self) -> Channels {
        self.nc.channels().into()
    }

    /// Gets the foreground channel.
    pub fn fg(&self) -> Channel {
        self.nc.fchannel().into()
    }

    /// Gets the background channel.
    pub fn bg(&self) -> Channel {
        self.nc.bchannel().into()
    }

    /// Sets the channels, returning the previous value.
    pub fn set_channels(&mut self, channels: impl Into<Channels>) -> Channels {
        let prev = self.channels();
        self.nc.set_channels(channels.into());
        prev
    }

    /// Sets the foreground channel, returning the previous value.
    pub fn set_fg(&mut self, channel: impl Into<Channel>) -> Channel {
        let fg = self.fg();
        self.nc.set_fchannel(channel.into());
        fg
    }

    /// Sets the background channel, returning the previous value.
    pub fn set_bg(&mut self, channel: impl Into<Channel>) -> Channel {
        let bg = self.fg();
        self.nc.set_bchannel(channel.into());
        bg
    }

    //

    /// Chain-sets the channels.
    pub fn cset_channels(mut self, channels: impl Into<Channels>) -> Self {
        self.nc.set_channels(channels.into());
        self
    }

    /// Chain-sets the foreground channel.
    pub fn cset_fg(mut self, channel: impl Into<Channel>) -> Self {
        self.nc.set_fchannel(channel.into());
        self
    }

    /// Chain-sets the foreground channel.
    pub fn cset_bg(mut self, channel: impl Into<Channel>) -> Self {
        self.nc.set_bchannel(channel.into());
        self
    }
}

/// # alpha methods
impl Cell {
    /// Gets the foreground alpha.
    pub fn fg_alpha(&self) -> Alpha {
        self.nc.fg_alpha().into()
    }

    /// Gets the background alpha.
    pub fn bg_alpha(&self) -> Alpha {
        self.nc.bg_alpha().into()
    }

    /// Sets the `foreground` alpha, returning the previous value.
    pub fn set_fg_alpha(&mut self, foreground: Alpha) -> Alpha {
        let prev = self.fg_alpha();
        self.nc.set_fg_alpha(foreground);
        prev
    }

    /// Gets the background alpha, returning the previous value.
    pub fn set_bg_alpha(&mut self, background: Alpha) -> Alpha {
        let prev = self.bg_alpha();
        self.nc.set_bg_alpha(background);
        prev
    }
}

/// # style methods
impl Cell {
    /// Gets the styles.
    pub fn styles(&self) -> Style {
        self.nc.styles().into()
    }

    /// Sets the `styles`, returning the previous value.
    pub fn set_styles(&mut self, styles: impl Into<Style>) -> Style {
        let prev = self.styles();
        self.nc.styles_set(styles.into());
        prev
    }

    /// Adds the specified `styles`, returning the previous value.
    pub fn add_styles(&mut self, styles: impl Into<Style>) -> Style {
        let prev = self.styles();
        self.nc.styles_on(styles.into());
        prev
    }

    /// Deletes the specified `styles`, returning the previous value.
    pub fn del_styles(&mut self, styles: impl Into<Style>) -> Style {
        let prev = self.styles();
        self.nc.styles_off(styles.into());
        prev
    }

    /// Chain-sets the `styles`.
    pub fn cset_style(mut self, styles: impl Into<Style>) -> Self {
        self.nc.styles_set(styles.into());
        self
    }
}
