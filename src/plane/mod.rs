// notcurses::plane
//
//!
//

use crate::{sys::NcPlane, Align, Notcurses, Position, Result, Size};

mod builder;
mod cell;

pub use builder::PlaneBuilder;
pub use cell::Cell;

/// A drawable text surface, composed of [`Cell`]s.
pub struct Plane {
    nc: *mut NcPlane,
}

mod std_impls {
    use super::{NcPlane, Plane};
    use std::fmt;

    impl Drop for Plane {
        fn drop(&mut self) {
            let _ = self.into_ref_mut().destroy();
        }
    }

    impl fmt::Debug for Plane {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Plane {{ {:?}, {:?} }}",
                self.size(),
                self.position(),
            )
        }
    }

    impl From<&mut NcPlane> for Plane {
        fn from(ncplane: &mut NcPlane) -> Plane {
            Plane {
                nc: ncplane as *mut NcPlane,
            }
        }
    }
}

/// # `Plane` constructors and deconstructors.
impl Plane {
    /// Returns a new [`PlaneBuilder`].
    pub fn builder() -> PlaneBuilder {
        PlaneBuilder::new()
    }

    //

    /// Returns a new root plane with default options.
    ///
    /// The plane will be positioned at `(0, 0)` and have the size of the terminal.
    pub fn new(nc: &mut Notcurses) -> Result<Self> {
        Self::builder().build(nc)
    }

    /// Returns a new root plane at a specific `position`.
    ///
    /// The plane will have the size of the terminal.
    pub fn new_at(nc: &mut Notcurses, position: impl Into<Position>) -> Result<Self> {
        Self::builder().position(position).build(nc)
    }

    /// Returns a new root plane with a specific size.
    ///
    /// - `size` must be greater than `0` in both dimensions.
    /// - The plane will be positioned at `(0, 0)`.
    pub fn new_sized(nc: &mut Notcurses, size: impl Into<Size>) -> Result<Self> {
        Self::builder().size(size).build(nc)
    }

    /// Returns a new root plane with a specific `size` and `position`.
    ///
    /// `size` must be greater than `0` in both dimensions.
    pub fn new_sized_at(
        nc: &mut Notcurses,
        size: impl Into<Size>,
        position: impl Into<Position>,
    ) -> Result<Self> {
        Self::builder().size(size).position(position).build(nc)
    }

    //

    /// Returns a new child plane with default options.
    ///
    /// The plane will be positioned at `(0, 0)` and have the size of the terminal.
    pub fn new_child(&mut self) -> Result<Self> {
        Self::builder().build_child(self)
    }

    /// Returns a new child plane at a specific `position`.
    ///
    /// The plane will be terminal sized.
    pub fn new_child_at(&mut self, position: impl Into<Position>) -> Result<Self> {
        Self::builder().position(position).build_child(self)
    }

    /// Returns a new child plane with a specific `size`.
    ///
    /// - `size` must be greater than `0` in both dimensions.
    /// - The plane will be positioned at `(0, 0)`.
    pub fn new_child_sized(&mut self, size: impl Into<Size>) -> Result<Self> {
        Self::builder().size(size).build_child(self)
    }

    /// Returns a new child plane with a specific `position` and `size`.
    ///
    /// `size` must be greater than `0` in both dimensions.
    pub fn new_child_sized_at(
        &mut self,
        size: impl Into<Size>,
        position: impl Into<Position>,
    ) -> Result<Self> {
        Self::builder()
            .size(size)
            .position(position)
            .build_child(self)
    }

    //

    /// Duplicates this `Plane`.
    ///
    /// The new plane will have the same geometry, the same rendering state,
    /// and all the same duplicated content.
    ///
    /// The new plane will be immediately above the old one on the z-axis.
    ///
    /// The new plane will be bound to the same parent, but since child planes
    /// are not duplicated, it will not have any children planes.
    ///
    pub fn duplicate(&mut self) -> Self {
        self.into_ref_mut().dup().into()
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

/// # `Plane` rendering
impl Plane {
    /// Renders and rasterizes the pile of which this `Plane` is part.
    pub fn render(&mut self) -> Result<()> {
        Ok(self.into_ref_mut().render_raster()?)
    }

    /// Just renders the pile of which this `Plane` is part, without rasterizing.
    ///
    /// Rendering this pile again will blow away the render.
    /// To actually write out the render, call [`rasterize`] afterwards.
    ///
    /// [`rasterize`]: Plane#method.rasterize
    pub fn render_only(&mut self) -> Result<()> {
        Ok(self.into_ref_mut().render()?)
    }
    /// Makes the physical screen match the last rendered frame from the pile of
    /// which this `Plane` is part.
    ///
    /// This is a blocking call. Don't call this before the pile has been
    /// rendered (doing so will likely result in a blank screen).
    pub fn rasterize(&mut self) -> Result<()> {
        Ok(self.into_ref_mut().rasterize()?)
    }

    // TODO
    // /// Performs the rendering and rasterization portion of
    // /// [`render`][Plane#method.render]
    // /// but does not write the resulting buffer out to the terminal.
    // ///
    // /// Using this function, the user can control the writeout process.
    // /// The returned buffer must be freed by the caller.
    // ///
    // pub fn render_to_buffer(&mut self, buffer: &mut Vec<u8>) -> Result<()> {
    //     Ok(self.into_ref_mut().render_to_buffer(buffer)?)
    // }

    // TODO
    // /// Writes the last rendered frame, in its entirety, to `file`.
    // ///
    // pub fn render_to_file(&mut self, file: &mut File) -> Result<()> {
    //     Ok(self.into_ref_mut().render_to_file(file)?)
    // }
}

/// # `Plane` size
impl Plane {
    /// Returns the size of the plane.
    pub fn size(&self) -> Size {
        self.into_ref().dim_yx().into()
    }

    /// Resizes the plane.
    ///
    /// The four parameters `keep_y`, `keep_x`, `keep_len_y`, and `keep_len_x`
    /// defines a subset of this `NcPlane` to keep unchanged.
    /// This may be a section of size 0.
    ///
    /// `keep_x` and `keep_y` are relative to this plane. They must specify a
    /// coordinate within the plane's totality. If either of `keep_len_y` or
    /// `keep_len_x` is non-zero, both must be non-zero.
    ///
    /// `y_off` and `x_off` are relative to `keep_y` and `keep_x`, and place the
    /// upper-left corner of the resized Plane.
    ///
    /// `len_y` and `len_x` are the dimensions of this plane after resizing.
    /// `len_y` must be greater than or equal to `keep_len_y`,
    /// and `len_x` must be greater than or equal to `keeplenx`.
    ///
    pub fn resize(
        &mut self,
        keep_y: u32,
        keep_x: u32,
        keep_len_y: u32,
        keep_len_x: u32,
        off_y: i32,
        off_x: i32,
        len_y: u32,
        len_x: u32,
    ) -> Result<()> {
        Ok(self.into_ref_mut().resize(
            keep_y, keep_x, keep_len_y, keep_len_x, off_y, off_x, len_y, len_x,
        )?)
    }

    /// Resizes this `NcPlane`, retaining what data we can (everything, unless we're
    /// shrinking in some dimension). Keeps the origin where it is.
    pub fn resize_simple(&mut self, size: Size) -> Result<()> {
        Ok(self
            .into_ref_mut()
            .resize_simple(size.height(), size.width())?)
    }
}

/// # `Plane` position
impl Plane {
    /// Returns the current position of this plane, relative to its parent.
    ///
    /// In the case of a root (parentless) plane, it will be the same as
    /// [`root_position`][Position#method.root_position].
    #[inline]
    pub fn position(&self) -> Position {
        self.into_ref().yx().into()
    }

    /// Returns the root position of this plane,
    /// which is relative to the root of the pile this plane is part of.
    #[inline]
    pub fn root_position(&self) -> Position {
        self.into_ref().abs_yx().into()
    }

    /// Moves this plane relative to its parent (or to its pile, if it's a root plane).
    pub fn move_to(&mut self, position: impl Into<Position>) -> Result<()> {
        let (y, x) = position.into().into();
        Ok(self.into_ref_mut().move_yx(y, x)?)
    }

    /// Moves this plane relative to its current position.
    ///
    /// - Negative values move up and left, respectively.
    /// - Pass 0 to hold an axis constant.
    pub fn move_rel(&mut self, offset: impl Into<Position>) -> Result<()> {
        let (rows, cols) = offset.into().into();
        Ok(self.into_ref_mut().move_rel(rows, cols)?)
    }

    /// Moves the plane such that it is entirely within its parent, if possible.
    ///
    /// No resizing is performed.
    pub fn place_within(&mut self) -> Result<()> {
        Ok(self.into_ref_mut().resize_placewithin()?)
    }

    /// Translates a `position` relative to this plane,
    /// into a position relative to the `target` plane.
    ///
    /// # Example
    /// ```ignore
    /// # use notcurses::*;
    /// # fn main() -> Result<()> {
    /// # let nc = Notcurses::new()?;
    ///     assert_eq![
    ///         Plane::new(&mut nc)?
    ///             .translate((0, 0), &Plane::new_at(&mut nc, (1, 0))?),
    ///         Position(-1, 0),
    ///     ];
    /// # Ok(())
    /// # }
    /// ```
    pub fn translate(&self, position: impl Into<Position>, target: &Plane) -> Position {
        let (mut y, mut x) = position.into().into();
        self.into_ref().translate(target.into_ref(), &mut y, &mut x);
        Position(y, x)
    }

    /// Translates a `position` relative to the root,
    /// into a position relative to this plane, and checks if it falls inside.
    ///
    /// Fields of the returned tuple:
    /// - `.0`: The translated `position`, from root to self,
    /// - `.1`: Is *true* when `position` is inside this plane, or *false* otherwise.
    ///
    /// # Example
    /// ```ignore
    /// # use notcurses::*;
    /// # fn main() -> Result<()> {
    /// # let nc = Notcurses::new()?;
    /// assert_eq![
    ///     Plane::new_at(&mut nc, (8, 8))?.translate_root(Position(7, 7)),
    ///     (Position(-1, -1), false),
    /// ];
    /// # Ok(())
    /// # }
    /// ```
    pub fn translate_root(&self, position: impl Into<Position>) -> (Position, bool) {
        let (mut y, mut x) = position.into().into();
        let inside = self.into_ref().translate_abs(&mut y, &mut x);
        (Position(y, x), inside)
    }
}

/// # `Plane` z-buffer position in the pile
impl Plane {
    /// Returns `true` if this plane is at the top of the pile.
    pub fn is_top(&mut self) -> bool {
        self.into_ref_mut().above().is_none()
    }

    /// Relocates this plane at the top of the z-buffer of its pile.
    pub fn move_top(&mut self) {
        self.into_ref_mut().move_top()
    }

    /// Relocates this plane and its children at the top of the z-buffer.
    ///
    /// Relative order will be maintained between the reinserted planes.
    ///
    /// For a plane E bound to C, with z-ordering A B C D E, moving the C family
    /// to the top results in C E A B D.
    pub fn move_family_top(&mut self) {
        self.into_ref_mut().move_family_top()
    }

    //

    /// Returns `true` if this plane is at the bottom of the pile.
    pub fn is_bottom(&mut self) -> bool {
        self.into_ref_mut().below().is_none()
    }

    /// Relocates this plane at the bottom of the z-buffer of its pile.
    pub fn move_bottom(&mut self) {
        self.into_ref_mut().move_bottom()
    }

    /// Relocates this plane and its children at the bottom of the z-buffer.
    ///
    /// Relative order will be maintained between the reinserted planes.
    ///
    /// For a plane E bound to C, with z-ordering A B C D E, moving the C family
    /// to the bottom results in A B D C E.
    pub fn move_family_bottom(&mut self) {
        self.into_ref_mut().move_family_bottom()
    }

    //

    /// Relocates this plane above `other` plane, in the z-buffer.
    ///
    /// Errors if the current plane is already in the desired location,
    /// or if both planes are the same.
    pub fn move_above(&mut self, other: &mut Plane) -> Result<()> {
        Ok(self.into_ref_mut().move_above(other.into_ref_mut())?)
    }

    /// Relocates this plane and its children above `other` plane, in the z-buffer.
    ///
    /// Errors if the current plane is already in the desired location,
    /// or if both planes are the same.
    pub fn move_family_above(&mut self, other: &mut Plane) -> Result<()> {
        Ok(self
            .into_ref_mut()
            .move_family_above(other.into_ref_mut())?)
    }

    //

    /// Relocates this plane below `other` plane, in the z-buffer.
    ///
    /// Errors if the current plane is already in the desired location,
    /// or if both planes are the same.
    pub fn move_below(&mut self, other: &mut Plane) -> Result<()> {
        Ok(self.into_ref_mut().move_below(other.into_ref_mut())?)
    }

    /// Relocates this plane abnd its children below the `other` plane, in the z-buffer.
    ///
    /// Errors if the current plane is already in the desired location,
    /// or if both planes are the same.
    pub fn move_family_below(&mut self, other: &mut Plane) -> Result<()> {
        Ok(self
            .into_ref_mut()
            .move_family_below(other.into_ref_mut())?)
    }

    //

    /// Unbounds this plane from its parent and makes it a child of `new_parent`.
    ///
    /// Any child planes of this plane are reparented to the previous parent.
    ///
    /// If this plane is equal to `new_parent` it becomes the root of a new pile,
    /// unless it's already the root of a pile, in which case this is a no-op.
    ///
    pub fn reparent(&mut self, new_parent: &mut Plane) {
        let _ = self.into_ref_mut().reparent(new_parent.into_ref_mut());
    }

    /// Unbounds this plane from its parent and makes it a child of `new_parent`,
    /// including its child planes, maintaining their z-order.
    ///
    /// If this plane is equal to `new_parent` it becomes the root of a new pile,
    /// unless it's already the root of a pile, in which case this is a no-op.
    ///
    pub fn reparent_family(&mut self, new_parent: &mut Plane) {
        let _ = self.into_ref_mut().reparent(new_parent.into_ref_mut());
    }
}

/// # `Plane` alignment, scrolling and growing
impl Plane {
    /// Returns the column at which `width` columns ought start
    /// in order to be aligned according to `h` alignment within this plane.
    ///
    /// Returns [u32::MAX] if [`Align::Unaligned`].
    #[inline]
    pub fn halign(&self, horizontal: Align, width: u32) -> Result<u32> {
        Ok(self.into_ref().halign(horizontal, width)?)
    }

    /// Returns the row at which `rows` rows ought start
    /// in order to be aligned according to `v` alignment within this plane.
    #[inline]
    pub fn valign(&self, vertical: Align, height: u32) -> Result<u32> {
        Ok(self.into_ref().valign(vertical, height)?)
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

/// # `Plane` cursor related methods
impl Plane {
    /// Returns the current cursor `(row, column)` position within this plane.
    pub fn cursor_position(&self) -> Position {
        self.into_ref().cursor_yx().into()
    }

    //

    /// Moves the cursor to the home position `(0, 0)`.
    pub fn cursor_move_home(&mut self) {
        self.into_ref_mut().cursor_home()
    }

    /// Moves the cursor to the specified `position` within this plane.
    ///
    /// The cursor doesn't need to be visible.
    ///
    /// Errors if the parameters exceed the plane's dimensions, and the cursor
    /// will remain unchanged in that case.
    pub fn cursor_move_to(&mut self, position: impl Into<Position>) -> Result<()> {
        let (row, col) = position.into().into();
        Ok(self.into_ref_mut().cursor_move_yx(row, col)?)
    }

    /// Moves the cursor to the specified `row` within this plane.
    ///
    /// The cursor doesn't need to be visible.
    ///
    /// Errors if the row number exceed the plane's rows, and the cursor
    /// will remain unchanged in that case.
    pub fn cursor_move_to_row(&mut self, row: u32) -> Result<()> {
        Ok(self.into_ref_mut().cursor_move_y(row)?)
    }

    /// Moves the cursor to the specified `column` within this plane.
    ///
    /// The cursor doesn't need to be visible.
    ///
    /// Errors if the column number exceed the plane's columns, and the cursor
    /// will remain unchanged in that case.
    pub fn cursor_move_to_col(&mut self, column: u32) -> Result<()> {
        Ok(self.into_ref_mut().cursor_move_x(column)?)
    }
}
