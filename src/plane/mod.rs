// notcurses::plane
//
//!
//

use crate::{
    sys::NcPlane, Align, Blitter, Capabilities, Channel, Channels, Notcurses, Position, Result,
    Size, Style,
};

mod builder;
mod cell;
mod geometry;

pub use builder::PlaneBuilder;
pub use cell::Cell;
pub use geometry::PlaneGeometry;

/// A drawable text surface, composed of [`Cell`]s.
pub struct Plane {
    nc: *mut NcPlane,
}

mod std_impls {
    use super::{NcPlane, Plane};
    use crate::notcurses::CLI_PLANE_LOCK;
    use once_cell::sync::OnceCell;
    use std::fmt;

    impl Drop for Plane {
        fn drop(&mut self) {
            if self.is_cli() {
                // Allows instancing a new Plane referring to the *standard* Plane again.
                CLI_PLANE_LOCK.with(|refcell| {
                    refcell.replace(OnceCell::new());
                });
            }

            let _ = self.into_ref_mut().destroy();
        }
    }

    impl fmt::Debug for Plane {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut opts = String::new();

            if self.is_cli() {
                opts += "CLI+";
            }
            if self.is_scrolling() {
                opts += "scroll+";
            }
            if self.is_autogrow() {
                opts += "autogrow+";
            }
            opts.pop();

            write!(
                f,
                "Plane {{ {:?}, {:?} [{opts}] {} cursor:{} }}",
                self.size(),
                self.position(),
                self.channels(),
                self.cursor(),
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

/// # constructors
impl Plane {
    /// Returns the *cli* Plane for the provided `notcurses` instance.
    ///
    /// Returns an error if there's already one *cli* plane instantiated.
    pub fn from_cli(notcurses: &mut Notcurses) -> Result<Plane> {
        notcurses.cli_plane()
    }

    //

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
    pub fn new_child(&mut self) -> Result<Plane> {
        Self::builder().build_child(self)
    }

    /// Returns a new child plane at a specific `position`.
    ///
    /// The plane will be terminal sized.
    pub fn new_child_at(&mut self, position: impl Into<Position>) -> Result<Plane> {
        Self::builder().position(position).build_child(self)
    }

    /// Returns a new child plane with a specific `size`.
    ///
    /// - `size` must be greater than `0` in both dimensions.
    /// - The plane will be positioned at `(0, 0)`.
    pub fn new_child_sized(&mut self, size: impl Into<Size>) -> Result<Plane> {
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
    pub fn duplicate(&mut self) -> Plane {
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

/// # the CLI plane
impl Plane {
    /// Is this plane the [*CLI* plane][Plane#the-cli-plane]?
    ///
    /// > There can only be one.
    pub fn is_cli(&self) -> bool {
        let nc = unsafe { self.into_ref().notcurses_const() }.expect("notcurses_const");
        let stdplane = unsafe { nc.stdplane_const() };
        std::ptr::eq(stdplane, self.into_ref())
    }
}

/// # rendering
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

/// # size, geometry
impl Plane {
    // convenience function to get the capabilities directly from a Plane.
    fn capabilities(&self) -> Capabilities {
        let nc = unsafe { self.into_ref().notcurses_const() }.expect("notcurses_const");

        Capabilities {
            halfblock: nc.canhalfblock(),
            quadrant: nc.canquadrant(),
            sextant: nc.cansextant(),
            braille: nc.canbraille(),
            utf8: nc.canutf8(),
            images: nc.canopen_images(),
            videos: nc.canopen_videos(),
            pixel: nc.canpixel(),
            pixel_impl: nc.check_pixel_support(),
            truecolor: nc.cantruecolor(),
            fade: nc.canfade(),
            palette_change: nc.canchangecolor(),
            palette_size: nc.palette_size().unwrap_or(0),
        }
    }

    /// Returns the geometry of the plane, using the best blitter available.
    pub fn geometry_best(&self) -> PlaneGeometry {
        let blitter = self.capabilities().best_blitter();
        let ncgeom = self.into_ref().pixel_geom();
        (ncgeom, blitter).into()
    }

    /// Returns the geometry of the plane, using the provided blitter.
    pub fn geometry_with(&self, blitter: Blitter) -> PlaneGeometry {
        let ncgeom = self.into_ref().pixel_geom();
        (ncgeom, blitter).into()
    }

    /// Returns the size of the plane.
    pub fn size(&self) -> Size {
        self.into_ref().dim_yx().into()
    }

    /// Resizes the plane to a new `size`.
    ///
    /// An area of the plane to keep unchanged is defined by `keep` and `keep_len`.
    ///
    /// Note that
    /// - `keep` position is relative to the plane.
    /// - `offset` position is relative to `keep`, placing the upper-left-corner
    ///    of the resized plane.
    ///
    /// # Errors
    /// - if `keep` falls outside of the plane.
    /// - if `keep_size` is zero in just one dimension.
    /// - if `size` is smaller than `keep_size` in any dimension.
    pub fn resize(
        &mut self,
        size: Size,
        keep: Position,
        keep_size: Size,
        offset: Position,
    ) -> Result<()> {
        let (keep_y, keep_x) = keep.into();
        let (keep_len_y, keep_len_x) = keep_size.into();
        let (off_y, off_x) = offset.into();
        let (len_y, len_x) = size.into();
        Ok(self.into_ref_mut().resize(
            keep_y, keep_x, keep_len_y, keep_len_x, off_y, off_x, len_y, len_x,
        )?)
    }

    /// Resizes this `NcPlane`, retaining what data we can (everything, unless
    /// we're shrinking in some dimension). Keeps the origin where it is.
    pub fn resize_simple(&mut self, size: Size) -> Result<()> {
        Ok(self
            .into_ref_mut()
            .resize_simple(size.height(), size.width())?)
    }

    // TODO CHECK callbacks

    // /// Realigns this plane against its parent, using the alignment specified
    // /// at creation time.
    // ///
    // /// Suitable for use as a [`ResizeCallback`].
    // pub fn resize_realign(&mut self) -> Result<()> {
    //     Ok(self.into_ref_mut().resize_realign()?)
    // }

    // /// Resizes this plane against its parent, attempting to enforce
    // /// the supplied margins.
    // ///
    // /// This is suitable for use as a [`ResizeCallback`] on planes created
    // /// with [`maximize`][PlaneBuilder#method.maximize].
    // pub fn resize_maximize(&mut self) -> Result<()> {
    //     Ok(self.into_ref_mut().resize_maximize()?)
    // }

    // /// Resizes this plane to the visual area's size.
    // pub fn resize_maximize_visual(&mut self) -> Result<()> {
    //     Ok(self.into_ref_mut().resize_maximize()?)
    // }

    // /// Returns this plane's current resize callback, or `None` if not set.
    // pub fn resize_cb(&self) -> Option<ResizeCb> {
    //     self.into_ref().resizecb()
    // }

    // /// (Un)Sets this plane's resize callback.
    // pub fn set_resize_cb(&self, Option<ResizeCb>) {
    //     self.into_ref_mut().set_resizecb()
    // }
}

/// # area positioning
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

/// # z-buffer positioning
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

    // /// Returns true if the plane is a root plane (has no parents).
    //
    // WIP TRACKING ISSUE: https://github.com/dankamongmen/notcurses/issues/2657
    // pub fn is_root(&self) -> bool {
    //     let ncp = unsafe { self.into_ref().parent_const() };
    //     println!("is_root >>> {:?}", ncp);
    //     // true
    //     ncp.is_err()
    // }

    /// Unbounds this plane from its parent and makes it a child of `new_parent`.
    ///
    /// Any child planes of this plane are reparented to the previous parent.
    ///
    /// If this plane is equal to `new_parent` it becomes the root of a new pile,
    /// unless it's already the root of a pile, in which case this is a no-op.
    pub fn reparent(&mut self, new_parent: &mut Plane) {
        let _ = self.into_ref_mut().reparent(new_parent.into_ref_mut());
    }

    /// Unbounds this plane from its parent and makes it a child of `new_parent`,
    /// including its child planes, maintaining their z-order.
    ///
    /// If this plane is equal to `new_parent` it becomes the root of a new pile,
    /// unless it's already the root of a pile, in which case this is a no-op.
    pub fn reparent_family(&mut self, new_parent: &mut Plane) {
        let _ = self.into_ref_mut().reparent(new_parent.into_ref_mut());
    }
}

/// # alignment, scrolling and growing
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
        // NOTE: if this is the cli mode, it should update Notcurses's options,
        // but that's not possible from here with the current system.
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

/// # cursor related methods
impl Plane {
    /// Returns the current cursor `(row, column)` position within this plane.
    pub fn cursor(&self) -> Position {
        self.into_ref().cursor_yx().into()
    }

    //

    /// Moves the cursor to the home position `(0, 0)`.
    pub fn cursor_home(&mut self) {
        self.into_ref_mut().cursor_home()
    }

    /// Moves the cursor to the specified `position` within this plane.
    ///
    /// The cursor doesn't need to be visible.
    ///
    /// Errors if the coordinates exceed the plane's dimensions, and the cursor
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

/// # text and cells
impl<'plane> Plane {
    /// Writes a `string` to the current cursor position, using the current style.
    ///
    /// Returns the number of columns the cursor has advanced.
    ///
    /// ## Errors
    /// - if the position falls outside the plane's area.
    /// - if a glyph can't fit in the current line, unless scrolling is enabled.
    pub fn putstr(&mut self, string: &str) -> Result<u32> {
        Ok(self.into_ref_mut().putstr(string)?)
    }

    /// Writes a `string` to the current cursor position, ending in newline,
    /// and using the current style.
    ///
    /// Returns the number of columns the cursor has advanced.
    ///
    /// ## Errors
    /// - if the position falls outside the plane's area.
    /// - if a glyph can't fit in the current line, unless scrolling is enabled.
    pub fn putstrln(&mut self, string: &str) -> Result<u32> {
        Ok(self.into_ref_mut().putstrln(string)?)
    }

    /// Writes a newline to the current cursor position.
    ///
    /// A newline counts as 1 column advanced.
    pub fn putln(&mut self) -> Result<u32> {
        Ok(self.into_ref_mut().putln()?)
    }

    /// Writes a `string` to some `y`, and a `horizontal` alignment,
    /// using the current style.
    ///
    /// Returns the number of columns the cursor has advanced.
    ///
    /// ## Errors
    /// - if the position falls outside the plane's area.
    /// - if a glyph can't fit in the current line, unless scrolling is enabled.
    pub fn putstr_aligned(
        &mut self,
        y: Option<u32>,
        horizontal: Align,
        string: &str,
    ) -> Result<u32> {
        Ok(self.into_ref_mut().putstr_aligned(y, horizontal, string)?)
    }

    /// Writes a `string` to the current position, using the pre-existing style.
    ///
    /// Returns the number of columns the cursor has advanced.
    ///
    /// ## Errors
    /// - if the position falls outside the plane's area.
    /// - if a glyph can't fit in the current line, unless scrolling is enabled.
    pub fn putstr_stained(&mut self, string: &str) -> Result<u32> {
        Ok(self.into_ref_mut().putstr_stained(string)?)
    }

    /// Writes a `string` to `y`, and `horizontal` alignment,
    /// using the pre-existing style.
    ///
    /// Returns the number of columns the cursor has advanced.
    ///
    /// ## Errors
    /// - if the position falls outside the plane's area.
    /// - if a glyph can't fit in the current line, unless scrolling is enabled.
    pub fn putstr_aligned_stained(
        &mut self,
        y: Option<u32>,
        horizontal: Align,
        string: &str,
    ) -> Result<u32> {
        Ok(self
            .into_ref_mut()
            .putstr_aligned_stained(y, horizontal, string)?)
    }

    //

    /// Writes a `string` to `position`, using the current style.
    ///
    /// Returns the number of columns the cursor has advanced.
    ///
    /// ## Errors
    /// - if the position falls outside the plane's area.
    /// - if a glyph can't fit in the current line, unless scrolling is enabled.
    pub fn putstr_at(&mut self, position: impl Into<Position>, string: &str) -> Result<u32> {
        let (y, x): (u32, u32) = position.into().into();
        Ok(self.into_ref_mut().putstr_yx(y.into(), x.into(), string)?)
    }

    /// Writes a `string` to some `y`, some `x`, or both, using the current style.
    ///
    /// Returns the number of columns the cursor has advanced.
    ///
    /// It wont move over a axis that is set to `None`.
    ///
    /// ## Errors
    /// - if the position falls outside the plane's area.
    /// - if a glyph can't fit in the current line, unless scrolling is enabled.
    pub fn putstr_at_yx(&mut self, y: Option<u32>, x: Option<u32>, string: &str) -> Result<u32> {
        Ok(self.into_ref_mut().putstr_yx(y, x, string)?)
    }

    //

    /// Writes a `string` to the current cursor position, using the current style,
    /// and no more than `len` bytes will be written.
    ///
    /// Returns the number of columns the cursor has advanced.
    ///
    /// ## Errors
    /// - if the position falls outside the plane's area.
    /// - if a glyph can't fit in the current line, unless scrolling is enabled.
    pub fn putstr_len(&mut self, len: usize, string: &str) -> Result<u32> {
        Ok(self.into_ref_mut().putnstr(len, string)?)
    }

    /// Writes a `string` to some `position`, using the current style,
    /// and no more than `len` bytes will be written.
    ///
    /// Returns the number of columns the cursor has advanced.
    ///
    /// ## Errors
    /// - if the position falls outside the plane's area.
    /// - if a glyph can't fit in the current line, unless scrolling is enabled.
    pub fn putstr_len_at(
        &mut self,
        position: impl Into<Position>,
        len: usize,
        string: &str,
    ) -> Result<u32> {
        let (y, x): (u32, u32) = position.into().into();
        Ok(self
            .into_ref_mut()
            .putnstr_yx(y.into(), x.into(), len, string)?)
    }

    /// Writes a `string` to some `y`, some `x`, using the current style,
    /// and no more than `len` bytes will be written.
    ///
    /// Returns the number of columns the cursor has advanced.
    ///
    /// ## Errors
    /// - if the position falls outside the plane's area.
    /// - if a glyph can't fit in the current line, unless scrolling is enabled.
    pub fn putstr_len_at_yx(
        &mut self,
        y: Option<u32>,
        x: Option<u32>,
        len: usize,
        string: &str,
    ) -> Result<u32> {
        Ok(self.into_ref_mut().putnstr_yx(y, x, len, string)?)
    }

    /// Writes a `string` to some `y`, and a `horizontal` alignment,
    /// using the current style, and no more than `len` bytes will be written.
    ///
    /// Returns the number of columns the cursor has advanced.
    ///
    /// ## Errors
    /// - if the position falls outside the plane's area.
    /// - if a glyph can't fit in the current line, unless scrolling is enabled.
    pub fn putstr_len_aligned(
        &mut self,
        y: Option<u32>,
        horizontal: Align,
        len: usize,
        string: &str,
    ) -> Result<u32> {
        Ok(self
            .into_ref_mut()
            .putnstr_aligned(y, horizontal, len, string)?)
    }

    /// Considers the glyph at `position` as the fill target,
    /// and copies `cell` to it and to all cardinally-connected cells.
    ///
    /// Returns the number of cells polyfilled.
    ///
    /// Errors if the position falls outside the plane's area.
    pub fn polyfill_yx(&mut self, position: impl Into<Position>, cell: &Cell) -> Result<usize> {
        let (y, x): (u32, u32) = position.into().into();
        Ok(self
            .into_ref_mut()
            .polyfill_yx(y.into(), x.into(), cell.into())?)
    }

    //

    /// Returns the cell at `position`.
    pub fn cell_at(&self, position: impl Into<Position>) -> Result<Cell> {
        let (y, x) = position.into().into();
        let mut cell = crate::sys::NcCell::new();
        let _bytes = self.into_ref().at_yx_cell(y, x, &mut cell)?;
        Ok(cell.into())
    }

    // TODO:
    // style_at
    // channels_at
}

/// # colors and styles
impl Plane {
    /// Gets the channels.
    pub fn channels(&self) -> Channels {
        self.into_ref().channels().into()
    }

    /// Gets the foreground channel.
    pub fn fg(&self) -> Channel {
        self.into_ref().fchannel().into()
    }

    /// Gets the foreground channel.
    pub fn bg(&self) -> Channel {
        self.into_ref().bchannel().into()
    }

    /// Sets the channels.
    pub fn set_channels(&mut self, channels: impl Into<Channels>) {
        self.into_ref_mut().set_channels(channels.into())
    }

    /// Sets the `foreground` channel. Returns the updated channels.
    pub fn set_fg(&mut self, foreground: impl Into<Channel>) -> Channels {
        self.into_ref_mut().set_fchannel(foreground.into()).into()
    }

    /// Sets the `background` channel. Returns the updated channels.
    pub fn set_bg(&mut self, background: impl Into<Channel>) -> Channels {
        self.into_ref_mut().set_bchannel(background.into()).into()
    }

    /// Sets the background channel to the default. Returns the updated channels.
    pub fn unset_bg(&mut self) -> Channels {
        self.set_bg(Channel::with_default())
    }

    /// Sets the foreground channel to the default. Returns the updated channels.
    pub fn unset_fg(&mut self) -> Channels {
        self.set_fg(Channel::with_default())
    }
}

/// # base cell
impl Plane {
    /// Returns this plane's base `Cell`.
    pub fn base(&self) -> Result<Cell> {
        Ok(self.into_ref().base()?.into())
    }

    /// Sets the plane's base [`Cell`] from its components.
    ///
    /// Returns the number of bytes copied out of `egc`.
    ///
    /// The base cell shows anywhere in the plane with an empty `egc`.
    ///
    /// Note that erasing the plane does not reset the base cell.
    pub fn set_base(
        &mut self,
        egc: &str,
        style: Style,
        channels: impl Into<Channels>,
    ) -> Result<usize> {
        Ok(self.into_ref_mut().set_base(egc, style, channels.into())? as usize)
    }

    // /// Sets the plane's base [`Cell`].
    // ///
    // /// The base cell shows anywhere in the plane with an empty `egc`.
    // //
    // // NOTE: this doesn't CHECK the cell's egc points to this Plane's egcpool.
    // pub fn set_base_cell(&mut self, cell: &Cell) -> Result<()> {
    //     Ok(self.into_ref_mut().set_base_cell(cell.into())?)
    // }

    /// Sets the plane's base cell's `styles`,
    ///
    /// Returns the previous value.
    pub fn set_base_styles(&mut self, styles: Style) -> Result<Style> {
        let mut base = self.base()?;
        Ok(base.set_styles(styles))
    }

    /// Sets the plane's base cell's foreground & background `channels`.
    ///
    /// Returns the previous value.
    pub fn set_base_channels(&mut self, channels: Channels) -> Result<Channels> {
        let mut base = self.base()?;
        Ok(base.set_channels(channels))
    }

    /// Sets the plane's base cell's foreground `channel`.
    ///
    /// Returns the previous value.
    pub fn set_base_fg(&mut self, foreground: Channel) -> Result<Channel> {
        let mut base = self.base()?;
        Ok(base.set_fg(foreground))
    }

    /// Sets the plane's base cell's background `channel`.
    ///
    /// Returns the previous value.
    pub fn set_base_bg(&mut self, background: Channel) -> Result<Channel> {
        let mut base = self.base()?;
        Ok(base.set_bg(background))
    }
}
