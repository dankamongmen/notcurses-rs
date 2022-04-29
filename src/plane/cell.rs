// notcurses::cell::cell
//
//!
//

use crate::sys::NcCell;

/// A `Cell` corresponds to a single *[grapheme cluster]* on some [`Plane`],
///
/// A `Cell` is bounded to n `Plane`, but the cell doesn't store anything
/// about the plane.
///
/// At any `NcCell`, we can have a theoretically arbitrarily long UTF-8 string,
/// a foreground color, a background color, and an [`NcStyle`][crate::NcStyle] attribute set.
///
/// [grapheme cluster]: http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Cell {
    nc: NcCell,
}

mod std_impls {
    use super::{Cell, NcCell};

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
}

/// Constructors.
impl Cell {}

/// Methods.
impl Cell {}
