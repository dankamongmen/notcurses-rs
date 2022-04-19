// notcurses::plane::builder
//
//!
//

use crate::{
    sys::{NcPlane, NcPlaneOptionsBuilder},
    Align, Notcurses, Plane, Result,
};

/// A `Plane` builder.
#[derive(Debug, Default)]
pub struct PlaneBuilder {
    options: NcPlaneOptionsBuilder,
}

/// # Constructors
impl PlaneBuilder {
    /// Returns a new default `PlaneBuilder`.
    ///
    /// Size, position and margins are set to 0.
    /// The plane will be maximized to its parent size.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new standalone `Plane`.
    pub fn build(self, nc: &mut Notcurses) -> Result<Plane> {
        let ncplane = NcPlane::new_pile(nc.into_ref_mut(), &self.options.build())?;
        Ok(Plane { nc: ncplane })
    }

    /// Returns a new child `Plane` of the provided parent.
    pub fn build_child(self, parent: &mut Plane) -> Result<Plane> {
        let ncplane = NcPlane::new_child(parent.into_ref_mut(), &self.options.build())?;
        Ok(Plane { nc: ncplane })
    }
}

/// # Methods (chainable)
impl PlaneBuilder {
    /// Sets the vertical placement relative to parent plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *`y`* coordinate and unsets vertical alignment.
    pub fn y(mut self, y: i32) -> Self {
        self.options.set_y(y);
        self
    }

    /// Sets the horizontal placement relative to parent plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *`x`* coordinate and unsets vertical alignment.
    pub fn x(mut self, x: i32) -> Self {
        self.options.set_x(x);
        self
    }

    /// Sets the vertical & horizontal placement relative to parent plane.
    ///
    /// Default: *`0`*, *`0`*.
    ///
    /// Effect: Sets both *`x`* & *`y`* coordinates and unsets both horizontal and
    /// vertical alignment.
    pub fn yx(mut self, y: i32, x: i32) -> Self {
        self.options.set_yx(y, x);
        self
    }

    //

    /// Sets the vertical alignment.
    ///
    /// Default: *[`Align::Top`]*.
    ///
    /// Effect: Sets *`v`* alignment.
    pub fn valign(mut self, v: Align) -> Self {
        self.options.set_valign(v);
        self
    }

    /// Sets the horizontal alignment.
    ///
    /// Default: *[`Align::Left`]*.
    ///
    /// Effect: Sets *`h`* alignment.
    pub fn halign(mut self, h: Align) -> Self {
        self.options.set_halign(h);
        self
    }

    /// Sets the vertical & horizontal placement relative to parent plane.
    ///
    /// Default: *`0`*, *`0`*.
    ///
    /// Effect: Sets both horizontal and vertical alignment.
    pub fn align(mut self, v: Align, h: Align) -> Self {
        self.options.set_align(v, h);
        self
    }

    //

    /// Sets the numbers of rows for the plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the height of the plane and *unmaximizes* it.
    pub fn rows(mut self, rows: u32) -> Self {
        self.options.set_rows(rows);
        self
    }

    /// Sets the numbers of columns for the plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the width of the plane and *unmaximizes* it.
    pub fn cols(mut self, cols: u32) -> Self {
        self.options.set_cols(cols);
        self
    }

    /// Sets the numbers of rows and columns for the plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the height and width of the plane and *unmaximizes* it.
    pub fn rows_cols(mut self, rows: u32, cols: u32) -> Self {
        self.options.set_rows_cols(rows, cols);
        self
    }

    //

    /// Maximizes the plane, with optional bottom & right margins.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// Effect: maximizes the plane relative to the parent plane, minus the
    /// provided margins.
    ///
    /// See also: [`sys::NcPlaneFlag::Marginalized`].
    ///
    /// [`sys::NcPlaneFlag::Marginalized`]: crate::sys::NcPlaneFlag#associatedconstant.Marginalized
    pub fn maximize(mut self, bottom: u32, right: u32) -> Self {
        self.options.set_margins(bottom, right);
        self
    }

    /// If `true`, the plane will **not** scroll with the parent.
    ///
    /// Default: *false* (scrolls with the parent).
    ///
    /// Effect: (un)fixes the plane.
    ///
    /// See also: [`sys::NcPlaneFlag::Fixed`].
    ///
    /// [`sys::NcPlaneFlag::Fixed`]: crate::sys::NcPlaneFlag#associatedconstant.Fixed
    pub fn fixed(mut self, fixed: bool) -> Self {
        self.options.set_fixed(fixed);
        self
    }

    /// If `true`, the plane will scroll vertically to accommodate output.
    ///
    /// This is equivalent to immediately calling [`set_scrolling(true)`]
    /// following `Plane` creation.
    ///
    /// Default: *false*.
    ///
    /// Effect: (un)sets vertical scrolling.
    ///
    /// [`set_scrolling(true)`]: crate::Plane#method.set_scrolling
    pub fn scroll(mut self, scroll: bool) -> Self {
        self.options.set_vscroll(scroll);
        self
    }

    /// If `true`, the plane will grow automatically.
    ///
    /// Default: *false*.
    ///
    /// Effect: (un)sets the plane to automatically grow to accomodate output.
    ///
    /// Note that just setting `autogrow` makes the plane grow to the right,
    /// and setting `autogrow` and `scroll` makes the plane grow downwards.
    pub fn autogrow(mut self, autogrow: bool) -> Self {
        self.options.set_autogrow(autogrow);
        self
    }

    // TODO: resizecb
}
