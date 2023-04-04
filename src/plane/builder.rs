// notcurses::plane::builder
//
//!
//

use crate::{
    error::NotcursesResult as Result,
    plane::{Align, Plane, Position, Size},
    sys::{NcPlane, NcPlaneOptionsBuilder},
    Notcurses,
};

/// A [`Plane`] builder.
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
    /// Sets the vertical position relative to the parent plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *`y`* coordinate and unsets vertical alignment.
    pub fn y(mut self, y: i32) -> Self {
        self.options.set_y(y);
        self
    }

    /// Sets the horizontal position relative to the parent plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *`x`* coordinate and unsets vertical alignment.
    pub fn x(mut self, x: i32) -> Self {
        self.options.set_x(x);
        self
    }

    /// Sets the position relative to parent plane.
    ///
    /// Default: *`0`*, *`0`*.
    ///
    /// Effect: Sets both *`x`* & *`y`* coordinates and unsets both horizontal and
    /// vertical alignment.
    pub fn position(mut self, position: impl Into<Position>) -> Self {
        let (x, y) = position.into().into();
        self.options.set_yx(y, x);
        self
    }

    //

    /// Sets the vertical alignment.
    ///
    /// Default: *[`Align::Top`]*.
    ///
    /// Effect: Sets *`v`* alignment.
    pub fn valign(mut self, vertical: Align) -> Self {
        self.options.set_valign(vertical);
        self
    }

    /// Sets the horizontal alignment.
    ///
    /// Default: *[`Align::Left`]*.
    ///
    /// Effect: Sets *`h`* alignment.
    pub fn halign(mut self, horizontal: Align) -> Self {
        self.options.set_halign(horizontal);
        self
    }

    /// Sets the vertical & horizontal placement relative to parent plane.
    ///
    /// Default: *`0`*, *`0`*.
    ///
    /// Effect: Sets both horizontal and vertical alignment.
    pub fn align(mut self, vertical: Align, horizontal: Align) -> Self {
        self.options.set_align(vertical, horizontal);
        self
    }

    //

    /// Sets the height of the plane (rows).
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the height of the plane and *unmaximizes* it.
    pub fn height(mut self, height: u32) -> Self {
        self.options.set_rows(height);
        self
    }

    /// Sets the width for the plane (columns).
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the width of the plane and *unmaximizes* it.
    pub fn width(mut self, width: u32) -> Self {
        self.options.set_cols(width);
        self
    }

    /// Sets the size of the plane (rows, columns).
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the height and width of the plane and *unmaximizes* it.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        let (width, height) = size.into().into();
        self.options.set_rows_cols(height, width);
        self
    }

    //

    /// Maximizes the plane, with optional right & bottom margins.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// Effect: maximizes the plane relative to the parent plane, minus the
    /// provided margins.
    ///
    /// See also: [`sys::NcPlaneFlag::Marginalized`].
    ///
    /// [`sys::NcPlaneFlag::Marginalized`]: crate::sys::NcPlaneFlag#associatedconstant.Marginalized
    pub fn maximize(mut self, right_bottom: impl Into<Size>) -> Self {
        let (x_right, y_bottom) = right_bottom.into().into();
        self.options.set_margins(y_bottom, x_right);
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
    /// [`set_scrolling(true)`]: super::Plane#method.set_scrolling
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
