//!

use crate::{
    ncresult, // Channels,
    sys::{NcChannels, NcPlane},
    NResult,
    Notcurses,
    Style,
};

mod builder;
pub use builder::PlaneBuilder;

/// A rectilinear surface meant for text dncplaneing.
///
/// Can be larger than the physical screen, or smaller, or the same size; it can
/// be entirely contained within the physical screen, or overlap in part, or lie
/// wholly beyond the boundaries, never to be rendered.
///
/// a Plane is defined by:
///
/// - its *geometry*, its *position* relative to the visible plane and its *z-index*.
/// - its current *style*, *foreground* [`Channel`], and *background* `Channel`.
/// - its framebuffer, being a rectilinear matrix of [`Cells`].
/// - its *base `Cell`*, used for any cell on the plane without a glyph.
/// - its current *cursor location*.
// - a configured user curry (a void*),
// - an optional resize callback,
// - a name (used only for debugging).

#[derive(Debug)]
pub struct Plane<'ncplane> {
    pub(crate) ncplane: &'ncplane mut NcPlane,
}

impl<'ncplane> Drop for Plane<'ncplane> {
    /// Destroys this Plane.
    ///
    /// None of its contents will be visible after the next render call.
    fn drop(&mut self) {
        let _ = self.ncplane.destroy();
    }
}

impl<'ncplane> From<&'ncplane mut NcPlane> for Plane<'ncplane> {
    fn from(ncplane: &'ncplane mut NcPlane) -> Self {
        Self { ncplane }
    }
}

/// # Constructors and converters.
impl<'ncplane> Plane<'ncplane> {
    /// Returns a [`PlaneBuilder`] used to customize a new `Plane`.
    pub fn build() -> PlaneBuilder {
        PlaneBuilder::default()
    }

    /// New `Plane` with the size of the terminal.
    // FIXME
    pub fn with_term_size(nc: &mut Notcurses) -> NResult<Self> {
        Self::build().term_size(nc).new_pile(nc)
    }

    /// Returns a reference to the inner [`NcPlane`].
    pub fn as_ncplane(&self) -> &NcPlane {
        self.ncplane
    }

    /// Returns a mutable reference to the inner [`NcPlane`].
    pub fn as_ncplane_mut(&mut self) -> &mut NcPlane {
        self.ncplane
    }
}

/// # Methods
impl<'ncplane> Plane<'ncplane> {
    /// Moves the plane relatively the provided `cols` & `rows`.
    pub fn move_rel(&mut self, cols: i32, rows: i32) -> NResult<()> {
        ncresult![self.ncplane.move_rel(rows, cols)]
    }

    /// Moves the plane to the absolute coordinates `x`, `y`.
    pub fn move_abs(&mut self, x: i32, y: i32) -> NResult<()> {
        ncresult![self.ncplane.move_yx(y, x)]
    }

    /// Sets the base cell from its components.
    ///
    /// Returns the number of bytes copied out of `egc`.
    pub fn set_base<CHANNELS: Into<NcChannels>>(
        &mut self,
        egc: &str,
        style: Style,
        channels: CHANNELS,
    ) -> NResult<u32> {
        ncresult![self.ncplane.set_base(egc, style.bits(), channels.into())]
    }

    /// Renders the pile the current `Plane` is part of.
    pub fn render(&mut self) -> NResult<()> {
        ncresult![self.ncplane.render()]
    }

    /// Rasterizes the pile the current `Plane` is part of.
    pub fn raster(&mut self) -> NResult<()> {
        ncresult![self.ncplane.rasterize()]
    }

    /// Renders and rasterizes the pile the current `Plane` is part of.
    pub fn show(&mut self) -> NResult<()> {
        self.render()?;
        self.raster()?;
        Ok(())
    }

    // /// Resizes this `Plane`.
    // ///
    // /// The four parameters `keep_y`, `keep_x`, `keep_len_y`, and `keep_len_x`
    // /// defines a subset of this NcPlane to keep unchanged. This may be a section
    // /// of size 0.
    // ///
    // /// `keep_x` and `keep_y` are relative to this NcPlane. They must specify a
    // /// coordinate within the ncplane's totality. If either of `keep_len_y` or
    // /// `keep_len_x` is non-zero, both must be non-zero.
    // ///
    // /// `y_off` and `x_off` are relative to `keep_y` and `keep_x`, and place the
    // /// upper-left corner of the resized NcPlane.
    // ///
    // /// `y_len` and `x_len` are the dimensions of this NcPlane after resizing.
    // /// `y_len` must be greater than or equal to `keep_len_y`,
    // /// and `x_len` must be greater than or equal to `keeplenx`.
    // ///
    // /// It is an error to attempt to resize the standard plane.
    // ///
    // pub fn resize(&mut self, x: u32, y: u32) -> NResult<()> {
    // }

    // TODO: create a simplified version
    // pub fn resize_subrect(&mut self, x: u32, y: u32) -> NResult<()> {
    // }
}
