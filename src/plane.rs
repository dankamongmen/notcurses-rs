//! # Planes and Piles
//!
//!
//!
#![allow(dead_code)]

use crate::{
    ncresult,
    sys::{NcChannelPair, NcPlane, NcPlaneOptions},
    Dimension, Error, Nc, Offset, Result, Style,
};

// TODO:DESIGN:
// - create new plane, root of a new pile
// - create a new plane attach to an existing pile
//
// CONSIDER:
// - what happens with Visual? can a pile consist of both?
//   - see: NcVisual::from_plane
// - use options? use builder pattern?
//

/// The fundamental drawing surface.
pub struct Plane<'a> {
    pub(crate) raw: &'a mut NcPlane,
}

impl<'a> AsMut<NcPlane> for Plane<'a> {
    fn as_mut(&mut self) -> &mut NcPlane {
        //self.raw.get_mut()
        self.raw
    }
}

impl<'a> Drop for Plane<'a> {
    /// Destroys this Plane.
    ///
    /// None of its contents will be visible after the next render call.
    fn drop(&mut self) {
        // let _ = self.raw.get_mut().destroy();
        let _ = self.raw.destroy();
    }
}

/// # Constructors and translators
impl<'a> Plane<'a> {
    // # Constructors

    /// Returns a [`PlaneBuilder`] used to customize a new `Plane`.
    pub fn build() -> PlaneBuilder {
        PlaneBuilder {
            rows: 1,
            cols: 1,
            ..Default::default()
        }
    }

    /// Creates a `Plane` from an existing [`NcPlane`].
    pub fn from_ncplane(plane: &'a mut NcPlane) -> Plane<'a> {
        Self { raw: plane }
    }

    /// Returns a mutable reference to the inner [`NcPlane`].
    pub fn as_ncplane(&'a mut self) -> &'a mut NcPlane {
        self.raw
    }

    // /// Creates a new Plane, in a new pile, having the same dimensions of the
    // /// terminal.
    // pub fn with_termsize() -> Self {}
}

/// # Methods
impl<'a> Plane<'a> {
    pub fn move_rel(&mut self, rows: Offset, cols: Offset) -> Result<()> {
        ncresult![self.raw.move_rel(rows, cols)]
    }

    /// Sets the base cell from its components.
    ///
    /// Returns the number of bytes copied out of 'gcluster'
    pub fn set_base(&mut self, egc: &str, style: Style, channels: NcChannelPair) -> Result<u32> {
        // TODO: create macro that wraps this
        match self.raw.set_base(egc, style.bits(), channels) {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(Error::NcError {
                int: e.int,
                msg: e.msg,
            }),
        }
    }

    /// Renders the pile the current `Plane` is part of.
    pub fn render(&mut self) -> Result<()> {
        ncresult![self.raw.render()]
    }

    /// Rasterizes the pile the current `Plane` is part of.
    pub fn raster(&mut self) -> Result<()> {
        ncresult![self.raw.rasterize()]
    }

    /// Renders and rasterizes the pile the current `Plane` is part of.
    pub fn render_raster(&mut self) -> Result<()> {
        self.render()?;
        self.raster()?;
        Ok(())
    }
}

/// A [`Plane`] builder.
#[derive(Default)]
pub struct PlaneBuilder {
    y: Offset,
    x: Offset,
    rows: Dimension,
    cols: Dimension,
    // resizecb: Option<NcResizeCb>, // FUTURE
    flags: u64,
    margin_b: Offset,
    margin_r: Offset,
    /// A flag to indicate if the plane is horizontally aligned
    is_horizontally_aligned: bool, // TBD
    /// A flag to indicate whether the plane is bounded to another plane,
    /// or will be the pile of it's
    is_bounded: bool, // TBD
}

impl PlaneBuilder {
    /// Sets the number of rows (>= 1).
    pub fn rows(mut self, rows: Dimension) -> Self {
        self.rows = rows;
        self
    }

    /// Sets the number of columns (>= 1).
    pub fn cols(mut self, cols: Dimension) -> Self {
        self.cols = cols;
        self
    }

    /// Sets the vertical placement relative to the parent plane.
    pub fn y(mut self, y: Offset) -> Self {
        self.y = y;
        self
    }

    /// Sets the horizontal positioning of the Plane being built.
    pub fn x(mut self, x: Offset) -> Self {
        self.x = x;
        self
    }

    /// Sets the flags of the Plane being built.
    // TODO: make an enum
    pub fn flags(mut self, flags: u64) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the bottom margin.
    pub fn margin_b(mut self, margin_b: Offset) -> Self {
        self.margin_b = margin_b;
        self
    }
    /// Sets the right margin.
    pub fn margin_r(mut self, margin_r: Offset) -> Self {
        self.margin_r = margin_r;
        self
    }

    /// Sets the rows and columns to match the terminal size.
    pub fn termsize(mut self, nc: &Nc) -> Self {
        let (rows, cols) = nc.termsize();
        self.rows = rows;
        self.cols = cols;
        self
    }

    // BUILD FINISHERS

    /// Finishes and returns the new [`Plane`] over the provided [`Nc`] context.
    //
    // TODO: horizontal alignment
    pub fn new_pile<'a>(self, nc: &mut Nc<'a>) -> Result<Plane<'a>> {
        let options = NcPlaneOptions::with_flags(
            self.y,
            self.x,
            self.rows,
            self.cols,
            None,       // TODO resizecb
            self.flags, // TODO: use enum
            self.margin_b,
            self.margin_r,
        );
        let p = NcPlane::with_options(&mut nc.raw, options)?;
        Ok(Plane { raw: p })
    }

    /// Finishes and returns the new [`Plane`] bounded to the same pile of the
    /// provided plane.
    //
    // TODO: horizontal alignment
    pub fn in_pile<'a>(self, plane: &mut Plane<'a>) -> Result<Plane<'a>> {
        let options = NcPlaneOptions::with_flags(
            self.y,
            self.x,
            self.rows,
            self.cols,
            None,       // TODO resizecb
            self.flags, // TODO: use enum
            self.margin_b,
            self.margin_r,
        );
        let p = NcPlane::with_options_bound(&mut plane.raw, options)?;
        Ok(Plane { raw: p })
    }
}
