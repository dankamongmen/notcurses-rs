//!

use crate::{
    ncresult, // Channels,
    sys::{NcChannels, NcPlane},
    NotcursesResult as Result,
    Offset,
    Style,
};

mod builder;
pub use builder::PlaneBuilder;

/// A text drawing surface.
///
#[derive(Debug)]
pub struct Plane<'a> {
    pub(crate) raw: &'a mut NcPlane,
}

impl<'a> Drop for Plane<'a> {
    /// Destroys this Plane.
    ///
    /// None of its contents will be visible after the next render call.
    fn drop(&mut self) {
        let _ = self.raw.destroy();
    }
}

/// # Constructors and converters
impl<'a> Plane<'a> {
    /// Returns a [`PlaneBuilder`] used to customize a new `Plane`.
    pub fn build() -> PlaneBuilder {
        PlaneBuilder::default()
    }

    /// Creates a `Plane` from an existing [`NcPlane`].
    pub fn from_ncplane(plane: &'a mut NcPlane) -> Plane<'a> {
        Self { raw: plane }
    }

    /// Returns a reference to the inner [`NcPlane`].
    pub fn as_ncplane(&self) -> &NcPlane {
        self.raw
    }

    /// Returns a mutable reference to the inner [`NcPlane`].
    pub fn as_ncplane_mut(&mut self) -> &mut NcPlane {
        self.raw
    }
}

/// # Methods
impl<'a> Plane<'a> {
    /// Moves the plane relatively the provided `cols` & `rows`.
    pub fn move_rel(&mut self, cols: Offset, rows: Offset) -> Result<()> {
        ncresult![self.raw.move_rel(rows, cols)]
    }

    /// Moves the plane to the absolute coordinates `x`, `y`.
    pub fn move_abs(&mut self, x: Offset, y: Offset) -> Result<()> {
        ncresult![self.raw.move_yx(y, x)]
    }

    /// Sets the base cell from its components.
    ///
    /// Returns the number of bytes copied out of 'gcluster'
    pub fn set_base<CHANNELS: Into<NcChannels>>(
        &mut self,
        egc: &str,
        style: Style,
        channels: CHANNELS,
    ) -> Result<u32> {
        ncresult![self.raw.set_base(egc, style.bits(), channels.into())]
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
