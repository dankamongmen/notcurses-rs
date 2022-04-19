// notcurses::plane
//
//!
//

use crate::{sys::NcPlane, Align, Notcurses, Result};

mod builder;
pub use builder::PlaneBuilder;

/// A drawable text surface, composed of *cells*.
#[derive(Debug)]
pub struct Plane {
    nc: *mut NcPlane,
}

impl Drop for Plane {
    fn drop(&mut self) {
        let _ = self.into_ref_mut().destroy();
    }
}

/// # `Plane` constructors and deconstructors.
impl Plane {
    /// Returns a new [`PlaneBuilder`].
    pub fn builder() -> PlaneBuilder {
        PlaneBuilder::new()
    }

    /// Returns a new standalone `Plane` with default options.
    pub fn new(nc: &mut Notcurses) -> Result<Self> {
        Self::builder().build(nc)
    }

    /// Returns a new standalone `Plane` with specific position and size.
    pub fn new_sized(nc: &mut Notcurses, y: i32, x: i32, rows: u32, cols: u32) -> Result<Self> {
        Self::builder().yx(y, x).rows_cols(rows, cols).build(nc)
    }

    /// Returns a new child `Plane` of the current plane, with default options.
    pub fn new_child(&mut self) -> Result<Self> {
        Self::builder().build_child(self)
    }

    /// Returns a new child `Plane` of the current plane,
    /// with specific position and size.
    pub fn new_child_sized(&mut self, y: i32, x: i32, rows: u32, cols: u32) -> Result<Self> {
        Self::builder()
            .yx(y, x)
            .rows_cols(rows, cols)
            .build_child(self)
    }

    /// Returns a new standalone `Plane` with the size of the terminal.
    pub fn with_termsize(nc: &mut Notcurses) -> Result<Self> {
        let rc = nc.rows_cols();
        Self::builder().rows_cols(rc.0, rc.1).build(nc)
    }

    //

    /// Returns a shared reference to the inner [`NcPlane`].
    pub fn into_ref(&self) -> &NcPlane {
        unsafe { &*self.nc }
    }

    /// Returns an exclusive reference to the inner [`NcPlane`].
    pub fn into_ref_mut(&mut self) -> &mut NcPlane {
        unsafe { &mut *self.nc }
    }
}

/// # `Plane` methods for position, size & alignment.
impl Plane {
    /// Returns the `(y, x)` position coordinates of this plane.
    pub fn yx(&self) -> (i32, i32) {
        self.into_ref().yx()
    }

    /// Returns the vertical `y` coordinate of this plane.
    #[inline]
    pub fn y(&self) -> i32 {
        self.into_ref().y()
    }

    /// Returns the horizontal `x` coordinate of this plane.
    #[inline]
    pub fn x(&self) -> i32 {
        self.into_ref().x()
    }

    /// Returns the number of `(columns, rows)` of this plane.
    pub fn cols_rows(&self) -> (u32, u32) {
        self.into_ref().dim_yx()
    }

    /// Returns the number of `columns` of this plane.
    #[inline]
    pub fn cols(&self) -> u32 {
        self.into_ref().dim_y()
    }

    /// Returns the number of `rows` of this plane.
    #[inline]
    pub fn rows(&self) -> u32 {
        self.into_ref().dim_x()
    }

    //

    // TODO: resize/resize_simple
    // TODO: rotate_cw rotate_ccw
    // TODO: translate, translate_abs

    //

    /// Returns the column at which `cols` columns ought start
    /// in order to be aligned according to `h` alignment within this plane.
    ///
    /// Returns [u32::MAX] if [`Align::Unaligned`].
    #[inline]
    pub fn halign(&self, h: Align, cols: u32) -> Result<u32> {
        Ok(self.into_ref().halign(h, cols)?)
    }

    /// Returns the row at which `rows` rows ought start
    /// in order to be aligned according to `v` alignment within this plane.
    #[inline]
    pub fn valign(&self, v: Align, rows: u32) -> Result<u32> {
        Ok(self.into_ref().valign(v, rows)?)
    }

    /// Finds the center coordinate of a plane.
    ///
    /// In the case of an even number of rows/columns the top/left is preferred
    /// (in such a case, there will be one more cell to the bottom/right
    /// of the center than the top/left).
    /// The center is then modified relative to the plane's origin.
    #[inline]
    pub fn center_abs(&self) -> (u32, u32) {
        self.into_ref().center_abs()
    }

    /// Returns `true` if this plane has *autogrow* enabled, or `false` otherwise.
    #[inline]
    pub fn is_autogrow(&self) -> bool {
        self.into_ref().autogrow_p()
    }

    /// (Un)Sets the *autogrow* behaviour of this plane.
    ///
    /// Returns true if scrolling was previously enabled or false otherwise.
    ///
    /// By default, planes are created with autogrow disabled.
    ///
    /// Normally, once output reaches the right boundary of a plane, it is
    /// impossible to place more output unless the cursor is first moved.
    ///
    /// If scrolling is enabled, the cursor will automatically move down and to
    /// the left in this case, but upon reaching the bottom right corner of the
    /// plane, it is impossible to place more output without a scrolling event.
    ///
    /// If autogrow is in play, the plane will automatically be enlarged to
    /// accommodate output. If scrolling is disabled, growth takes place to the
    /// right; it otherwise takes place at the bottom.
    ///
    /// The plane only grows in one dimension.
    #[inline]
    pub fn set_autogrow(&mut self, autogrow: bool) -> bool {
        self.into_ref_mut().set_autogrow(autogrow)
    }

    /// Returns true if this plane has scrolling enabled or false otherwise.
    #[inline]
    pub fn is_scrolling(&self) -> bool {
        self.into_ref().scrolling_p()
    }

    /// Sets the scrolling behaviour of this plane.
    /// Returns true if scrolling was previously enabled or false otherwise.
    #[inline]
    pub fn set_scrolling(&mut self, scrolling: bool) -> bool {
        self.into_ref_mut().set_scrolling(scrolling)
    }

    /// Sends `n` scroll events to the current plane.
    ///
    /// Returns an error if the current plane is not a scrolling plane,
    /// and otherwise returns the number of lines scrolled.
    #[inline]
    pub fn scroll(&mut self, n: u32) -> Result<u32> {
        Ok(self.into_ref_mut().scrollup(n)?)
    }

    /// Scrolls the current plane until `child` is no longer hidden beneath it.
    ///
    /// Returns an error if `child` is not a child of this plane, or if this
    /// plane is not scrolling, or `child` is fixed.
    ///
    /// Returns the number of scrolling events otherwise (might be 0).
    #[inline]
    pub fn scroll_child(&mut self, child: &Plane) -> Result<u32> {
        Ok(self.into_ref_mut().scrollup_child(child.into_ref())?)
    }
}
