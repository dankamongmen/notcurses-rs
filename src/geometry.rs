// notcurses::geometry
//
//!
//

/// The pixel/cell geometry of a [`Plane`][crate::Plane] or the terminal.
#[derive(Clone, Copy, Debug)]
pub struct PixelGeometry {
    /// The total height in rows of `Cell`s.
    // y / cy
    pub(crate) rows: u32,
    /// The total width in columns of `Cell`s.
    // x / cx
    pub(crate) cols: u32,

    /// The total height in pixels.
    pub(crate) y: u32,
    /// The total width in pixels.
    pub(crate) x: u32,

    /// A **b**itmap maximum height in pixels.
    pub(crate) by: u32,
    /// A **b**itmap maximum width in pixels.
    pub(crate) bx: u32,

    /// A **b**itmap maximum height in rows of `Cell`s.
    // bx / cx
    pub(crate) brows: u32,
    /// A **b**itmap maximum width in columns of `Cell`s.
    // bx / cx
    pub(crate) bcols: u32,

    /// A `Cell` height in pixels.
    pub(crate) cy: u32,
    /// A `Cell` width in pixels.
    pub(crate) cx: u32,
}

impl PixelGeometry {
    /// The total height in pixels.
    pub fn y(&self) -> u32 {
        self.y
    }
    /// The total width in pixels.
    pub fn x(&self) -> u32 {
        self.x
    }
    /// The total height in rows of `Cell`s.
    pub fn rows(&self) -> u32 {
        self.rows
    }
    /// The total width in columns of `Cell`s.
    pub fn cols(&self) -> u32 {
        self.cols
    }

    /// A bitmap maximum height pixels.
    pub fn bitmap_y_max(&self) -> u32 {
        self.by
    }
    /// A bitmap maximum width in pixels.
    pub fn bitmap_x_max(&self) -> u32 {
        self.bx
    }
    /// A bitmap maximum height in rows of `Cell`s.
    pub fn bitmap_rows_max(&self) -> u32 {
        self.brows
    }
    /// A bitmap maximum width in columns of `Cell`s.
    pub fn bitmap_cols_max(&self) -> u32 {
        self.bcols
    }

    /// A `Cell` height in pixels.
    pub fn cell_y(&self) -> u32 {
        self.cy
    }
    /// A `Cell` width in pixels.
    pub fn cell_x(&self) -> u32 {
        self.cx
    }
}

mod std_impls {
    use super::PixelGeometry;
    use crate::sys::NcPixelGeometry;

    impl From<NcPixelGeometry> for PixelGeometry {
        fn from(g: NcPixelGeometry) -> PixelGeometry {
            PixelGeometry {
                y: g.term_y,
                x: g.term_x,
                rows: g.term_y / g.cell_y,
                cols: g.term_x / g.cell_x,
                by: g.max_bitmap_y,
                bx: g.max_bitmap_x,
                brows: g.max_bitmap_y / g.cell_y,
                bcols: g.max_bitmap_x / g.cell_x,
                cy: g.cell_y,
                cx: g.cell_x,
            }
        }
    }

    impl From<PixelGeometry> for NcPixelGeometry {
        fn from(g: PixelGeometry) -> NcPixelGeometry {
            NcPixelGeometry {
                term_y: g.y,
                term_x: g.x,
                cell_y: g.cy,
                cell_x: g.cx,
                max_bitmap_y: g.by,
                max_bitmap_x: g.bx,
            }
        }
    }
}
