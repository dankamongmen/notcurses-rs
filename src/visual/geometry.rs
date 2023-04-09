// notcurses::visual::geometry
//
//!
//

use crate::{sys::NcVisualGeometry, visual::Blitter, Position, Size};

/// The geometry of a [`Visual`][super::Visual].
///
/// Inner values are calculated at the time of the call. A font change,
/// for example, could make all the fields invalid, except for `pixels`.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct VisualGeometry {
    /// The selected blitter.
    // blitter
    pub blitter: Blitter,

    /// True internal size in pixels, following any resizing.
    // pix_yx
    pub pixels: Size,

    /// Rendered size in pixels, as handed to the blitter, following any scaling.
    // rpix_yx
    pub rendered_pixels: Size,

    /// Rendered size in cells, following any padding.
    ///
    /// There's padding whenever `rendered_pixels` is not evenly divided by `scale`.
    // rcell_yx
    pub rendered_cells: Size,

    // /// The number of input pixels drawn to a single cell.
    // // scale_yx
    // pub scale: Size,
    /// A `Cell`s size, in pixels.
    // cdim_yx
    pub pixels_per_cell: Size,

    /// The origin of the region to be rendered (top-left corner).
    // beg_yx
    pub region_position: Position,

    /// The size of the region to be rendered.
    // len_yx
    pub region_size: Size,

    /// The largest bitmap size that the terminal is willing to accept.
    ///
    /// Or none if bitmaps are not supported.
    // maxpixel_yx
    pub max_bitmap_pixels: Option<Size>,
}

mod core_impls {
    use super::{NcVisualGeometry, Position, Size, VisualGeometry};
    // use core::fmt;

    // TODO
    // #[rustfmt::skip]
    // impl fmt::Debug for VisualGeometry {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         write!(f, "VisualGeometry {{\n    {:?}\n    {:?}\n}}", self.nc, self.pgeom)
    //     }
    // }

    impl From<NcVisualGeometry> for VisualGeometry {
        fn from(nc: NcVisualGeometry) -> VisualGeometry {
            Self {
                blitter: nc.blitter.into(),
                pixels: Size::from(nc.pix_yx.unwrap_or((0, 0))).swapped(),
                rendered_pixels: Size::from(nc.rpix_yx.unwrap_or((0, 0))).swapped(),
                rendered_cells: Size::from(nc.rcell_yx.unwrap_or((0, 0))).swapped(),
                pixels_per_cell: Size::from(nc.cdim_yx.unwrap_or((0, 0))).swapped(),
                region_position: Position::from(nc.beg_yx.unwrap_or((0, 0))).swapped(),
                region_size: Size::from(nc.len_yx.unwrap_or((0, 0))).swapped(),
                max_bitmap_pixels: nc.maxpixel_yx.map(|s| Size::from(s).swapped()),
            }
        }
    }
}

/// # methods
impl VisualGeometry {
    /// The selected blitter.
    #[inline]
    pub const fn blitter(&self) -> Blitter {
        self.blitter
    }

    //

    /// The true internal size in pixels, following any resizing.
    #[inline]
    pub const fn pixels(&self) -> Size {
        self.pixels
    }

    /// The true internal size in cells, following any resizing.
    #[inline]
    pub fn cells(&self) -> Size {
        self.pixels * self.pixels_per_cell
    }

    /// The true internal size in cells, following any resizing.
    #[inline]
    pub fn blits(&self) -> Size {
        self.pixels * self.blits_per_cell()
    }

    //

    /// The rendered size in pixels, following any scaling.
    #[inline]
    pub const fn rendered_pixels(&self) -> Size {
        self.rendered_pixels
    }

    /// The rendered size in cells, following any padding.
    ///
    /// There's padding whenever `rendered_pixels` is not evenly divided by `blits_per_cell`.
    #[inline]
    pub const fn rendered_cells(&self) -> Size {
        self.rendered_cells
    }

    /// The rendered size in *blits*, following any padding.
    pub fn rendered_blits(&self) -> Size {
        self.rendered_cells * self.blits_per_cell()
    }

    //

    /// A `Cell`'s size, in pixels.
    #[inline]
    pub const fn pixels_per_cell(&self) -> Size {
        self.pixels_per_cell
    }

    /// A `Cell`s size, in `blitter` *blits*.
    #[inline]
    pub fn blits_per_cell(&self) -> Size {
        if self.blitter == Blitter::Pixel {
            self.pixels_per_cell
        } else {
            Size::new(
                self.blitter.cell_width().unwrap_or(0) as i32,
                self.blitter.cell_height().unwrap_or(0) as i32,
            )
        }
    }

    //

    /// Returns the maximum supported bitmap size, in pixels,
    /// or none if bitmaps are not supported.
    #[inline]
    pub const fn max_bitmap_pixels(&self) -> Option<Size> {
        self.max_bitmap_pixels
    }

    /// Returns the maximum supported bitmap size, in `Cell`s,
    /// or none if bitmaps are not supported.
    #[inline]
    pub fn max_bitmap_cells(&self) -> Option<Size> {
        self.max_bitmap_pixels
            .map(|size| size / self.pixels_per_cell)
    }

    /// Returns the maximum supported bitmap size, in *blits*,
    /// using the current blitter, or none if bitmaps are not supported.
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
                _ => blitter
                    .cell_size()
                    .map(|(h, w)| max * Size::new(w as i32, h as i32)),
            }
        } else {
            None
        }
    }
}
