// TEMP INFO
//
// C API functions & structs:
// - fn notcurses_term_dim_yx: the terminal size in cols,rows
// - fn ncplane_pixelgeom: geometry relevant to the plane
// - struct ncvgeom: geometry relevant to the visual (includes plane)
//
// see also: https://github.com/dankamongmen/notcurses/issues/1844

// Retrieve pixel geometry for the display region ('pxy', 'pxx'), each cell
// ('celldimy', 'celldimx'), and the maximum displayable bitmap ('maxbmapy',
// 'maxbmapx'). If bitmaps are not supported, 'maxbmapy' and 'maxbmapx' will
// be 0. Any of the geometry arguments may be NULL.
// API void ncplane_pixelgeom(const struct ncplane* n, int* pxy, int* pxx,
//                            int* celldimy, int* celldimx,
//                            int* maxbmapy, int* maxbmapx)
// NcPixelGeometry {
//     term_y: pxy as NcDim,
//     term_x: pxx as NcDim,
//     cell_y: celldimy as NcDim,
//     cell_x: celldimx as NcDim,
//     max_bitmap_y: maxbmapy as NcDim,
//     max_bitmap_x: maxbmapx as NcDim,

// FIXME this ought be used in the rendered mode API as well; it's currently
// only used by direct mode. describes all geometries of an ncvisual--both those
// which are inherent, and those in a given rendering regime.
//
// 1. pixy and pixx are the true internal pixel geometry, taken directly from
//    the load (and updated by ncvisual_resize()).
// 2. cdimy/cdimx are the cell pixel geometry *at the time of this call* (it can
//    change with a font change, in which case all values other than pixy/pixx
//    are invalidated).
// 3. rpixy/rpixx are the pixel geometry as handed to the blitter, following any
//    scaling. scaley/scalex are the number of input pixels drawn to full cell;
//    when using NCBLIT_PIXEL, they are equivalent to cdimy/cdimx.
// 4. rcelly/rcellx are the cell geometry as written by the blitter, following
//    any padding (there is padding whenever rpix{y, x} is not evenly divided by
//    scale{y, x}, and also sometimes for Sixel).
// 5. maxpixely/maxpixelx are defined only when NCBLIT_PIXEL is used, and
//    specify the largest bitmap that the terminal is willing to accept.
//
// typedef struct ncvgeom {
//   int pixy, pixx;     // true pixel geometry of ncvisual data
//   int cdimy, cdimx;   // terminal cell geometry when this was calculated
//   int rpixy, rpixx;   // rendered pixel geometry
//   int rcelly, rcellx; // rendered cell geometry
//   int scaley, scalex; // pixels per filled cell
//   // only defined for NCBLIT_PIXEL
//   int maxpixely, maxpixelx;
//   ncblitter_e blitter;// blitter that will be used

// TODO:
// impl From<NcPixelGeom> for Geometry {
// }

/// The geometry of a plane, the terminal, or plane. (WIP)
#[derive(Clone, Debug, Copy)]
pub struct Geometry {
    /// The total width in pixels.
    pub x: u32,
    /// The total height in pixels.
    pub y: u32,

    /// The total width in columns of `Cell`s.
    // x / cx
    pub cols: u32,
    /// The total height in rows of `Cell`s.
    // y / cy
    pub rows: u32,

    /// A **b**itmap maximum width in pixels.
    pub bx: u32,
    /// A **b**itmap maximum height in pixels.
    pub by: u32,

    /// A **b**itmap maximum width in columns of `Cell`s.
    // bx / cx
    pub bcols: u32,
    /// A **b**itmap maximum height in rows of `Cell`s.
    // bx / cx
    pub brows: u32,

    /// A `Cell` width in pixels.
    pub cx: u32,
    /// A `Cell` height in pixels.
    pub cy: u32,
}

impl Geometry {
    /// The total width in pixels.
    pub fn x(&self) -> u32 {
        self.x
    }
    /// The total height in pixels.
    pub fn y(&self) -> u32 {
        self.y
    }
    /// The total width in columns of `Cell`s.
    pub fn cols(&self) -> u32 {
        self.cols
    }
    /// The total height in rows of `Cell`s.
    pub fn rows(&self) -> u32 {
        self.rows
    }

    /// A bitmap maximum width in pixels.
    pub fn bitmap_x_max(&self) -> u32 {
        self.bx
    }
    /// A bitmap maximum height pixels.
    pub fn bitmap_y_max(&self) -> u32 {
        self.by
    }
    /// A bitmap maximum width in columns of `Cell`s.
    pub fn bitmap_cols_max(&self) -> u32 {
        self.bcols
    }
    /// A bitmap maximum height in rows of `Cell`s.
    pub fn bitmap_rows_max(&self) -> u32 {
        self.brows
    }

    /// A `Cell` width in pixels.
    pub fn cell_x(&self) -> u32 {
        self.cx
    }
    /// A `Cell` height in pixels.
    pub fn cell_y(&self) -> u32 {
        self.cy
    }
}
