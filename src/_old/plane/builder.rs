use crate::{
    sys::{NcPlane, NcPlaneOptions},
    NResult, Notcurses, Plane,
};

/// A [`Plane`] builder.
pub struct PlaneBuilder {
    // plane offset
    offset_x: i32,
    offset_y: i32,

    // plane size
    cols: u32,
    rows: u32,

    // resizecb: Option<NcResizeCb>, // FUTURE
    flags: u64,

    margin_b: i32,
    margin_r: i32,
    // /// A flag to indicate if the plane is horizontally aligned
    // is_horizontally_aligned: bool, // TBD
    // /// A flag to indicate whether the plane is bounded to another plane,
    // /// or will be the pile of it's
    // is_bounded: bool, // TBD
}

impl Default for PlaneBuilder {
    fn default() -> Self {
        Self {
            cols: 1,
            rows: 1,
            offset_x: 0,
            offset_y: 0,
            flags: 0,
            margin_b: 0,
            margin_r: 0,
        }
    }
}

impl PlaneBuilder {
    /// Sets the number of rows (>= 1).
    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = rows;
        self
    }

    /// Sets the number of columns (>= 1).
    pub fn cols(mut self, cols: u32) -> Self {
        self.cols = cols;
        self
    }

    /// Sets the number of columns and rows (>= 1).
    pub fn cols_rows(mut self, cols: u32, rows: u32) -> Self {
        self.cols = cols;
        self.rows = rows;
        self
    }

    /// Sets the vertical placement relative to the parent plane.
    pub fn y(mut self, y: i32) -> Self {
        self.offset_y = y;
        self
    }

    /// Sets the horizontal positioning of the Plane being built.
    pub fn x(mut self, x: i32) -> Self {
        self.offset_x = x;
        self
    }

    /// Sets the horizontal and vertical positioning of the Plane being built.
    pub fn xy(mut self, x: i32, y: i32) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }

    /// Sets the bottom margin.
    pub fn margin_b(mut self, margin_b: i32) -> Self {
        self.margin_b = margin_b;
        self
    }
    /// Sets the right margin.
    pub fn margin_r(mut self, margin_r: i32) -> Self {
        self.margin_r = margin_r;
        self
    }

    /// Sets the columns and rows to match the terminal size.
    pub fn term_size(mut self, nc: &Notcurses) -> Self {
        let (cols, rows) = nc.cols_rows();
        self.cols = cols;
        self.rows = rows;
        self
    }

    // BUILD FINISHERS

    /// Finishes and returns the new [`Plane`] over the provided [`Notcurses`]
    /// context.
    //
    // TODO: horizontal alignment
    pub fn new_pile<'nc, 'ncplane>(self, nc: &mut Notcurses<'nc>) -> NResult<Plane<'ncplane>> {
        let options = NcPlaneOptions::with_flags(
            self.offset_y,
            self.offset_x,
            self.rows,
            self.cols,
            None, // TODO resizecb
            self.flags,
            self.margin_b,
            self.margin_r,
        );
        let p = NcPlane::with_options(&mut nc.nc, options)?;
        Ok(Plane { ncplane: p })
    }

    /// Finishes and returns the new [`Plane`] bounded to the same pile of the
    /// provided plane.
    //
    // TODO: horizontal alignment
    pub fn into_pile<'ncplane1, 'ncplane2>(
        self,
        plane: &mut Plane<'ncplane1>,
    ) -> NResult<Plane<'ncplane2>> {
        let options = NcPlaneOptions::with_flags(
            self.offset_y,
            self.offset_x,
            self.rows,
            self.cols,
            None, // TODO resizecb
            self.flags,
            self.margin_b,
            self.margin_r,
        );
        let ncplane = NcPlane::with_options_bound(&mut plane.ncplane, options)?;
        Ok(Plane { ncplane })
    }
}
