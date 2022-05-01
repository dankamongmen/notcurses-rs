// notcurses::cell::cell
//
//!
//

use crate::{sys::NcCell, Channels, Plane, Result, Style};
use core::marker::PhantomData;

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
pub struct Cell<'p> {
    nc: NcCell,
    plane: PhantomData<&'p Plane>,
}

mod std_impls {
    use super::{Cell, Channels, NcCell, PhantomData, Style};
    use std::fmt;

    impl fmt::Display for Cell<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let width = self.nc.width;

            let egc = if let Some(s) = self.try_egc() {
                format!["\"{s}\""]
            } else {
                "&ref".into()
            };

            let style = Style::from(self.nc.stylemask);
            let channels = Channels::from(self.nc.channels);

            write!(f, "{egc} w:{width} s:{style} {channels}")
        }
    }
    impl fmt::Debug for Cell<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Cell {{{}}}", self)
        }
    }

    impl<'p> From<NcCell> for Cell<'p> {
        fn from(nc: NcCell) -> Cell<'p> {
            Self {
                nc,
                plane: PhantomData,
            }
        }
    }

    impl From<Cell<'_>> for NcCell {
        fn from(c: Cell) -> NcCell {
            c.nc
        }
    }
}

/// Constructors.
impl<'p> Cell<'p> {
    /// Creates an empty cell.
    pub fn new() -> Cell<'p> {
        NcCell::new().into()
    }

    /// Returns a Cell from a string.
    ///
    /// It only stores the first extended grapheme cluster from the string.
    pub fn from_str(plane: &'p mut Plane, string: &str) -> Result<Cell<'p>> {
        Ok(NcCell::from_str(plane.into_ref_mut(), string)?.into())
    }
}

/// Methods.
impl Cell<'_> {
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
}
