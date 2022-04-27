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
    pub max_bitmap_in_pixels: Option<Size>,
}

/// # Getters
impl Geometry {
    /// Returns the maximum supported bitmap size,
    /// in pixels, or none if bitmaps are not supported.
    pub fn max_bitmap_in_pixels(&self) -> Option<Size> {
        self.max_bitmap_in_pixels
    }

    /// Returns the maximum supported bitmap size, in `blitter` *blits*,
    /// or none if bitmaps are not supported.
    pub fn max_bitmap_in_blits(&self, blitter: Blitter) -> Option<Size> {
        if let Some(size) = self.max_bitmap_in_pixels {
            blitter.cell_size().map(|cell_size| size * cell_size.into())
        } else {
            None
        }
    }

    /// Returns the maximum supported bitmap size, in `Cell`s,
    /// or none if bitmaps are not supported.
    pub fn max_bitmap_in_cells(&self) -> Option<Size> {
        self.max_bitmap_in_pixels
            .map(|size| size * self.cell_in_pixels)
    }
}

impl Geometry {
    /// Returns the calculated geometry of the terminal using the desired `Blitter`.
    pub fn from_term(nc: &Notcurses, blitter: Blitter) -> Self {
        let pg: NcPixelGeometry = unsafe { nc.into_ref().stdplane_const().pixel_geom() };

        let cell_in_pixels = Size::new(pg.cell_y, pg.cell_x);
        let in_pixels = Size::new(pg.term_y, pg.term_x);
        let in_cells = Size::new(pg.term_y / pg.cell_y, pg.term_x / pg.cell_x);

        let cell_in_blits = Size::new(
            blitter.cell_height().unwrap_or(0) as u32,
            blitter.cell_width().unwrap_or(0) as u32,
        );

        let in_blits = in_cells * cell_in_blits;

        let max_bitmap_in_pixels = if pg.max_bitmap_y + pg.max_bitmap_x > 0 {
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
            max_bitmap_in_pixels,
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
        blitter.map(|blitter| Self::from_term(nc, blitter))
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
