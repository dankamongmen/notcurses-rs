//!

use crate::{
    ncresult, // Channels,
    sys::{self, NcChannels, NcPlane},
    NResult,
    Notcurses,
    Style,
};

use core::ptr::null_mut;

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
    pub fn with_term_size(nc: &mut Notcurses) -> NResult<Self> {
        Self::build().term_size(nc).new_pile(nc)
    }

    /// Duplicates this `Plane`.
    ///
    /// The new `Plane` will have the same geometry, the same rendering state,
    /// and all the same duplicated content.
    ///
    /// The new `Plane` will be immediately above the old one on the z axis,
    /// and will be bound to the same parent.
    ///
    /// Bound `Plane`s are not duplicated; the new plane is bound to the current
    /// parent, but has no bound planes.
    pub fn dup(&mut self) -> Self {
        // replicates the innards of sys::ncplane_dup because of lifetime issues:
        // - https://github.com/rust-lang/rust/issues/42868
        // - rustc --explain E0495
        // self.ncplane.dup::<'ncplane>().into()
        unsafe { &mut *sys::ncplane_dup(self.ncplane, null_mut()) }.into()
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

/// # Plane methods for rendering
impl<'ncplane> Plane<'ncplane> {
    /// Renders the pile the current `Plane` is part of.
    pub fn render(&mut self) -> NResult<()> {
        ncresult![self.ncplane.render()]
    }

    /// Rasterizes the pile the current `Plane` is part of.
    pub fn raster(&mut self) -> NResult<()> {
        ncresult![self.ncplane.rasterize()]
    }

    /// Renders and rasterizes the pile the current `Plane` is part of.
    pub fn display(&mut self) -> NResult<()> {
        self.render()?;
        self.raster()?;
        Ok(())
    }
}

/// # Plane methods for translation and resizing
impl<'ncplane> Plane<'ncplane> {
    /// Moves the plane relatively the provided `cols` & `rows`.
    pub fn move_rel(&mut self, cols: i32, rows: i32) -> NResult<()> {
        ncresult![self.ncplane.move_rel(rows, cols)]
    }

    /// Moves the plane to the absolute coordinates `x`, `y`.
    pub fn move_abs(&mut self, x: i32, y: i32) -> NResult<()> {
        ncresult![self.ncplane.move_yx(y, x)]
    }

    // TODO
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

/// # Plane methods for stack reordering (z-axis)
///
/// Planes are ordered along an imaginary z-axis, and can be reordered.
/// New Planes are placed on the top of the stack.
impl<'ncplane> Plane<'ncplane> {
    /// Relocates this Plane at the top of the stack.
    pub fn move_top(&mut self) {
        self.ncplane.move_top();
    }

    /// Relocates this Plane at the bottom of the stack.
    pub fn move_bottom(&mut self) {
        self.ncplane.move_bottom();
    }

    // NOTE: these methods can cause having multiple mutable burrows:
    //
    // pub fn top(&mut self) -> Plane<'ncplane> {
    //     Plane { ncplane: ncresult![self.ncplane.top()] }
    // }
    //
    //// pub fn bottom(&mut self) -> Plane<'ncplane> {
    //     Plane { ncplane: ncresult![self.ncplane.bottom()] }
    // }
}

/// # Plane methods for displaying text
impl<'ncplane> Plane<'ncplane> {
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

    /// Write a string to the current location, using the current `Style`.
    ///
    /// Advances the cursor by some positive number of columns (though not
    /// beyond the end of the plane); this number is returned on success.
    ///
    /// On error, a non-positive number is returned, indicating the number of
    /// columns which were written before the error.
    ///
    pub fn putstr(&mut self, string: &str) -> NResult<u32> {
        ncresult![self.ncplane.putstr(string)]
    }

    /// Write a string to the provided location, using the current `Style`.
    ///
    /// Advances the cursor by some positive number of columns (though not
    /// beyond the end of the plane); this number is returned on success.
    ///
    /// On error, a non-positive number is returned, indicating the number of
    /// columns which were written before the error.
    ///
    pub fn putstr_xy(&mut self, x: u32, y: u32, string: &str) -> NResult<u32> {
        ncresult![self.ncplane.putstr_yx(y, x, string)]
    }
}
