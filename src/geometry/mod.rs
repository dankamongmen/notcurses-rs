// notcurses::geometry
//
//!
//

use crate::{sys::NcPixelGeometry, Blitter, Notcurses};

mod tuples;
pub use tuples::{Position, Size};

/// The geometry of a [`Plane`][crate::Plane] or terminal.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct PlaneGeometry {
    /// The selected blitter.
    blitter: Blitter,

    /// Total size, in `Cell`s.
    in_cells: Size,

    /// Total size, in `blitter` dots.
    in_blits: Size,

    /// Total size, in pixels.
    in_pixels: Size,

    // /// A `Cell`s size, in `blitter` dots.
    // cell_in_blits: Size,
    /// A `Cell`'s size, in pixels.
    cell_in_pixels: Size,

    /// The maximum supported bitmap size, in pixels.
    ///
    /// Or None if bitmaps are not supported.
    pub max_bitmap_in_pixels: Option<Size>,
}

/// # Getters
// WIP
impl PlaneGeometry {
    /// The associated blitter.
    #[inline]
    pub const fn blitter(&self) -> Blitter {
        self.blitter
    }

    /// Total size, in `Cell`s.
    #[inline]
    pub const fn in_cells(&self) -> Size {
        todo![]
    }

    /// Total size, in `blitter` dots.
    #[inline]
    pub const fn in_blits(&self) -> Size {
        todo![]
    }

    /// Total size, in pixels.
    #[inline]
    pub const fn in_pixels(&self) -> Size {
        todo![]
    }

    /// A `Cell`s size, in `blitter` dots.
    #[inline]
    pub const fn cell_in_blits(&self) -> Size {
        todo![]
        // self.blitter()
    }

    /// A `Cell`'s size, in pixels.
    #[inline]
    pub const fn cell_in_pixels(&self) -> Size {
        self.cell_in_pixels
    }

    /// Returns the maximum supported bitmap size,
    /// in pixels, or none if bitmaps are not supported.
    pub const fn max_bitmap_in_pixels(&self) -> Option<Size> {
        self.max_bitmap_in_pixels
    }

    /// Returns the maximum supported bitmap size, in `blitter` *blits*,
    /// or none if bitmaps are not supported.
    pub fn max_bitmap_in_blits(&self, blitter: Blitter) -> Option<Size> {
        if let Some(size) = self.max_bitmap_in_pixels {
            blitter
                .cell_size()
                .map(|cell_size| size * Size::from(cell_size))
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

///
// WIP
impl PlaneGeometry {}

/// # Constructors
impl PlaneGeometry {
    /// Returns the calculated geometry of the terminal using the desired `Blitter`.
    pub fn from_term(nc: &Notcurses, blitter: Blitter) -> Self {
        let pg: NcPixelGeometry = unsafe { nc.into_ref().stdplane_const().pixel_geom() };

        let cell_in_pixels = Size::new(pg.cell_y, pg.cell_x);
        let in_pixels = Size::new(pg.term_y, pg.term_x);
        let in_cells = Size::new(pg.term_y / pg.cell_y, pg.term_x / pg.cell_x);

        let cell_in_blits = Size::from((
            blitter.cell_height().unwrap_or(0),
            blitter.cell_width().unwrap_or(0),
        ));

        let in_blits = in_cells * cell_in_blits;

        let max_bitmap_in_pixels = if pg.max_bitmap_y + pg.max_bitmap_x > 0 {
            Some(Size::from((pg.max_bitmap_y, pg.max_bitmap_x)))
        } else {
            None
        };

        Self {
            blitter,
            in_cells,
            in_blits,
            in_pixels,
            // cell_in_blits,
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
