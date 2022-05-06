// notcurses::plane::geometry
//
//!
//

use crate::{sys::NcPixelGeometry, Blitter, Notcurses, Size};

/// The geometry of a [`Plane`][crate::Plane] or a terminal.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct PlaneGeometry {
    /// The selected blitter.
    blitter: Blitter,

    /// Total size, in pixels.
    pixels: Size,

    /// A `Cell`'s size, in pixels.
    pixels_per_cell: Size,

    /// The maximum supported bitmap size, in pixels.
    ///
    /// Or None if bitmaps are not supported.
    max_bitmap_pixels: Option<Size>,
}

mod std_impls {
    use crate::{sys::NcPixelGeometry, Blitter, PlaneGeometry, Size};
    use std::fmt;

    #[rustfmt::skip]
    impl fmt::Debug for PlaneGeometry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let size = format![
                "[p{:?} b{:?} c{:?}]",
                self.pixels.into_tuple(),
                self.blits().into_tuple(),
                self.cells().into_tuple(),
            ];

            let max = if self.max_bitmap_pixels.is_some() {
                format![
                    "[p{:?}, b{:?}, c{:?}]",
                    self.max_bitmap_pixels.unwrap().into_tuple(),
                    self.max_bitmap_blits().unwrap().into_tuple(),
                    // self.max_bitmap_blitter(self.blitter), // .unwrap().into_tuple(),
                    self.max_bitmap_cells().unwrap().into_tuple(),
                ]
            } else {
                "None".to_string()
            };

            let cell = format![
                "[p{:?} b{:?}]",
                self.pixels_per_cell().into_tuple(),
                self.blits_per_cell().into_tuple(),
            ];

            write!(f, "PlaneGeometry {{ {} size:{size} max:{max} cell:{cell}] }}", self.blitter)
        }
    }

    /// needs both geometry & blitter information.
    impl From<(NcPixelGeometry, Blitter)> for PlaneGeometry {
        fn from(geom_blitter: (NcPixelGeometry, Blitter)) -> PlaneGeometry {
            let (g, blitter) = geom_blitter;

            let max_bitmap_pixels = if g.max_bitmap_x + g.max_bitmap_y != 0 {
                Some(Size(g.max_bitmap_y, g.max_bitmap_x))
            } else {
                None
            };

            PlaneGeometry {
                blitter,
                pixels: Size(g.term_y, g.term_x),
                pixels_per_cell: Size(g.cell_y, g.cell_x),
                max_bitmap_pixels,
            }
        }
    }

    impl From<PlaneGeometry> for NcPixelGeometry {
        fn from(g: PlaneGeometry) -> NcPixelGeometry {
            let (max_bitmap_y, max_bitmap_x) = g.max_bitmap_pixels.unwrap_or(Size(0, 0)).into();

            NcPixelGeometry {
                term_y: g.pixels.h(),
                term_x: g.pixels.w(),
                cell_y: g.pixels_per_cell.h(),
                cell_x: g.pixels_per_cell.w(),
                max_bitmap_y,
                max_bitmap_x,
            }
        }
    }
}

/// # constructors
impl PlaneGeometry {
    /// Returns the calculated geometry of the terminal using the desired `Blitter`.
    pub fn from_term(nc: &Notcurses, blitter: Blitter) -> Self {
        let pg: NcPixelGeometry = unsafe { nc.into_ref().stdplane_const().pixel_geom() };

        let pixels_per_cell = Size::new(pg.cell_y, pg.cell_x);
        let pixels = Size::new(pg.term_y, pg.term_x);
        let cells = pixels / pixels_per_cell;
        let cells2 = Size::new(pg.term_y / pg.cell_y, pg.term_x / pg.cell_x);
        assert_eq![cells, cells2];

        let max_bitmap_pixels = if pg.max_bitmap_y + pg.max_bitmap_x > 0 {
            Some(Size::from((pg.max_bitmap_y, pg.max_bitmap_x)))
        } else {
            None
        };

        Self {
            blitter,
            pixels,
            pixels_per_cell,
            max_bitmap_pixels,
        }
    }

    /// Returns the geometry for the first [`Blitter`] supported by the terminal,
    /// from the ones provided.
    pub fn from_term_first(nc: &Notcurses, blitters: &[Blitter]) -> Option<Self> {
        let caps = nc.capabilities();

        let mut blitter = None;
        for b in blitters {
            if caps.can_blitter(*b) {
                blitter = Some(b);
                break;
            }
        }
        blitter.map(|blitter| Self::from_term(nc, *blitter))
    }

    /// Returns the geometries of the [`Blitter`]s supported by the terminal,
    /// from the ones provided.
    pub fn from_term_all(nc: &Notcurses, blitters: &[Blitter]) -> Vec<Self> {
        let caps = nc.capabilities();

        let mut all = vec![];
        for b in blitters {
            if caps.can_blitter(*b) {
                all.push(Self::from_term(nc, *b));
            }
        }

        all
    }
}

/// # methods
impl PlaneGeometry {
    /// The current blitter.
    #[inline]
    pub const fn blitter(&self) -> Blitter {
        self.blitter
    }

    /// Total size, in `Cell`s.
    #[inline]
    pub fn cells(&self) -> Size {
        self.pixels / self.pixels_per_cell
    }

    /// Total size, in `blitter` blits.
    #[inline]
    pub fn blits(&self) -> Size {
        self.cells() * self.blits_per_cell()
    }

    /// Total size, in pixels.
    #[inline]
    pub const fn pixels(&self) -> Size {
        self.pixels
    }

    /// A `Cell`s size, in `blitter` *blits*.
    #[inline]
    pub fn blits_per_cell(&self) -> Size {
        if self.blitter == Blitter::Pixel {
            self.pixels_per_cell()
        } else {
            Size::from((
                self.blitter.cell_height().unwrap_or(0),
                self.blitter.cell_width().unwrap_or(0),
            ))
        }
    }

    /// A `Cell`'s size, in pixels.
    #[inline]
    pub const fn pixels_per_cell(&self) -> Size {
        self.pixels_per_cell
    }

    /// Returns the maximum supported bitmap size, in pixels,
    /// or none if bitmaps are not supported.
    #[inline]
    pub const fn max_bitmap_pixels(&self) -> Option<Size> {
        self.max_bitmap_pixels
    }

    /// Returns the maximum supported bitmap size, in *blits*,
    /// or none if bitmaps are not supported.
    pub fn max_bitmap_blits(&self) -> Option<Size> {
        self.max_bitmap_blitter(self.blitter)
    }

    /// Returns the maximum supported bitmap size, in *blits*, using the provided `blitter`,
    /// or none if bitmaps are not supported.
    #[inline]
    pub fn max_bitmap_blitter(&self, blitter: Blitter) -> Option<Size> {
        if let Some(max) = self.max_bitmap_cells() {
            match blitter {
                Blitter::Pixel => self.max_bitmap_pixels,
                Blitter::Default => None, // ←FIX
                _ => blitter.cell_size().map(|cs| max * Size::from(cs)),
            }
        } else {
            None
        }
    }

    /// Returns the maximum supported bitmap size, in `Cell`s,
    /// or none if bitmaps are not supported.
    #[inline]
    pub fn max_bitmap_cells(&self) -> Option<Size> {
        self.max_bitmap_pixels
            .map(|size| size / self.pixels_per_cell)
    }
}
