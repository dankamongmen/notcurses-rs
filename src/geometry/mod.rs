// notcurses::geometry
//
//!
//

use crate::{sys::NcPixelGeometry, Blitter, Notcurses};

mod pairs;
pub use pairs::{Coord, Offset, Size};

/// The geometry of a [`Plane`][crate::Plane] or the terminal.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Geometry {
    /// The selected blitter.
    pub blitter: Blitter,

    /// Total size, in `Cell`s.
    pub in_cells: Size,

    /// Total size, in `blitter` dots.
    pub in_blits: Size,

    /// Total size, in pixels.
    pub in_pixels: Size,

    /// A `Cell`s size, in `blitter` dots.
    pub cell_in_blits: Size,

    /// A `Cell`'s size, in pixels.
    pub cell_in_pixels: Size,

    /// The maximum supported bitmap size, in pixels.
    ///
    /// Or None if bitmaps are not supported.
    pub bitmap_max_pixels: Option<Size>,
}

/// # Getters
impl Geometry {
    /// Returns some size in pixels of the maximum supported bitmap,
    /// or none if bitmaps are not supported.
    pub fn bitmap_max_pixels(&self) -> Option<Size> {
        self.bitmap_max_pixels
    }
}

impl Geometry {
    /// Returns the calculated geometry of the terminal using the desired `Blitter`.
    pub fn from_term(nc: &Notcurses, blitter: Blitter) -> Self {
        let pg: NcPixelGeometry = unsafe { nc.into_ref().stdplane_const().pixel_geom() }.into();

        let cell_in_pixels = Size::new(pg.cell_y, pg.cell_x);
        let in_pixels = Size::new(pg.term_y, pg.term_x);
        let in_cells = Size::new(pg.term_y / pg.cell_y, pg.term_x / pg.cell_x);

        let cell_in_blits = Size::new(
            blitter.cell_height().unwrap_or(0) as u32,
            blitter.cell_width().unwrap_or(0) as u32,
        );
        let in_blits = Size::new(
            in_cells.h() * cell_in_blits.h(),
            in_cells.w() * cell_in_blits.w(),
        );

        let bitmap_max_pixels = if pg.max_bitmap_y + pg.max_bitmap_x > 0 {
            Some(Size::new(pg.max_bitmap_y, pg.max_bitmap_x))
        } else {
            None
        };

        Self {
            blitter,
            in_cells,
            in_blits,
            in_pixels,
            cell_in_blits,
            cell_in_pixels,
            bitmap_max_pixels,
        }
    }

    /// Returns the geometry for the first [`Blitter`] supported by the terminal,
    /// from the ones provided.
    pub fn from_term_first(nc: &Notcurses, blitters: Vec<Blitter>) -> Option<Self> {
        let caps = nc.capabilities();

        let mut blitter = None;
        for b in blitters {
            if caps.can_blitter(b) {
                blitter = Some(b);
                break;
            }
        }
        if let Some(blitter) = blitter {
            Some(Self::from_term(nc, blitter))
        } else {
            None
        }
    }

    /// Returns the geometries of the [`Blitter`]s supported by the terminal,
    /// from the ones provided.
    pub fn from_term_all(nc: &Notcurses, blitters: Vec<Blitter>) -> Vec<Self> {
        let caps = nc.capabilities();

        let mut all = vec![];
        for b in blitters {
            if caps.can_blitter(b) {
                all.push(Self::from_term(nc, b));
            }
        }

        all
    }
}
