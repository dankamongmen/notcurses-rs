//!

use crate::{
    ncresult, // Channels,
    sys::{self, NcChannels, NcPlane},
    Align,
    NResult,
    Notcurses,
    Style,
};

#[allow(unused_imports)]
use crate::{Cell, Channel};

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
/// - its current *[`Style`]*, *foreground [`Channel`]*, and *background [`Channel`]*.
/// - its framebuffer, which is a rectilinear matrix of [`Cell`]s.
/// - its *base [`Cell`]*, used for any cell on the plane without a glyph.
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
        unsafe { &mut *sys::c_api::ncplane_dup(self.ncplane, null_mut()) }.into()
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

    // TODO: Make resize_simple just resize, and the other a longer name.
    //
    // Planes can be freely resized, though they must be at least 1 cell high or wide.
    //
    // This function allows resizing a Plane, retaining all or a portion of the
    // plane's existing content, and translating the plane in one step.
    //
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

    // Resizes this `Plane`, retaining what data we can (everything, unless we're
    // shrinking in some dimension). Keeps the origin where it is.
    //
    // The helper function ncplane_resize_simple() allows resizing an ncplane without movement, retaining all possible data. To move the plane without resizing it or changing its content, use ncplane_move_yx(). It is an error to invoke these functions on the standard plane.
    // pub fn resize_simple(&mut self, x: u32, y: u32) -> NResult<()> {
    // }

    // MAYBE
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
    pub fn set_base<C: Into<NcChannels>>(
        &mut self,
        egc: &str,
        style: Style,
        channels: C,
    ) -> NResult<u32> {
        ncresult![self.ncplane.set_base(egc, style.bits(), channels.into())]
    }

    /// Write a string to the current cursor location, using the current `Style`.
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

    /// Same as [`putstr_xy()`][Plane#method.putstr_xy] but [`Align`]ed on x.
    ///
    pub fn putstr_aligned(&mut self, x_align: Align, y: u32, string: &str) -> NResult<u32> {
        ncresult![self.ncplane.putstr_aligned(y, x_align.into(), string)]
    }

    /// Replace a string's worth of glyphs at the current cursor location.
    /// Same as [`putstr()`][Plane#method.putstr] but retain the styling.
    /// The current styling of the plane will not be changed.
    ///
    pub fn putstr_stained(&mut self, string: &str) -> NResult<u32> {
        ncresult![self.ncplane.putstr_stained(string)]
    }

    /// Sets the scrolling setting. (`true` to enable, `false` to disable).
    ///
    /// All planes are created with scrolling disabled.
    ///
    /// Returns true if it was previously enabled, or false if it was disabled.
    ///
    /// ## More info
    ///
    /// While scrolling is disabled, attempting to print past the end of a line
    /// will stop at the plane boundary, and indicate an error.
    ///
    /// On a plane 10 columns wide and two rows high, printing "0123456789" at the
    /// origin should succeed, but printing "01234567890" will by default fail at
    /// the eleventh character. In either case, the cursor will be left at location
    /// 0x10; it must be moved before further printing can take place.
    ///
    /// If scrolling is enabled, the first row will be filled with 01234546789,
    /// the second row will have 0 written to its first column, and the cursor
    /// will end up at 1x1. Note that it is still an error to manually attempt
    /// to move the cursor off-plane, or to specify off-plane output.
    ///
    /// Boxes do not scroll; attempting to draw a 2x11 box on our 2x10 plane will
    /// result in an error and no output.
    ///
    /// When scrolling is enabled, and output takes place while the cursor is
    /// past the end of the last row, the first row is discarded, all other rows
    /// are moved up, the last row is cleared, and output begins at the beginning
    /// of the last row. This does not take place until output is generated
    /// (i.e. it is possible to fill a plane when scrolling is enabled).
    pub fn scrolling(&mut self, scrolling: bool) -> bool {
        self.ncplane.set_scrolling(scrolling)
    }

    /// Is this `Plane` set to scroll?
    ///
    /// See also [`scrolling`][Plane#method.scrolling].
    pub fn is_scrolling(&mut self) -> bool {
        self.ncplane.scrolling_p()
    }

    // TODO
    // Set the given channels throughout the specified region, keeping content and
    // attributes unchanged. Returns the number of cells set, or -1 on failure.
    // pub fn stain() {}
}
