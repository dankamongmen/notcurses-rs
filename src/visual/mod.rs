// notcurses::visual
//
//!
//

use crate::{
    sys::{self, NcVisual},
    Align, Blitter, Notcurses, Plane, Result, Rgba, Scale, Size,
};

mod builder;
pub use builder::VisualBuilder;

mod options;
use options::VisualOptions;

/// A visual bit of multimedia.
#[derive(Debug)]
pub struct Visual {
    nc: *mut NcVisual,
    options: VisualOptions,
}

impl Drop for Visual {
    fn drop(&mut self) {
        self.into_ref_mut().destroy()
    }
}

/// # `Visual` constructors and deconstructors.
impl Visual {
    /// Returns a new `Visual` builder.
    pub fn builder() -> VisualBuilder {
        VisualBuilder::new()
    }

    /// Returns a new `Visual` from a byte buffer with RGBA content.
    pub fn from_rgba(rgba: &[u8], size: impl Into<Size>) -> Result<Visual> {
        Visual::builder().build_from_rgba(rgba, size.into())
    }

    /// Builds a new `Visual` from a byte buffer with RGB content, providing
    /// the alpha to assign to all the pixels.
    pub fn from_rgb(rgb: &[u8], size: impl Into<Size>, alpha: u8) -> Result<Visual> {
        Visual::builder().build_from_rgb(rgb, size.into(), alpha)
    }

    /// Builds a new `Visual` from a byte buffer with RGBX content, overriding
    /// the alpha byte *X* for all the pixels.
    pub fn from_rgbx(rgbx: &[u8], size: impl Into<Size>, alpha: u8) -> Result<Visual> {
        Visual::builder().build_from_rgbx(rgbx, size.into(), alpha)
    }

    /// Builds a new `Visual` from a byte buffer with BGRA content.
    ///
    /// This is slower than [`build_from_rgba`][VisualBuilder#method.build_fromrgba],
    /// since it has to convert the pixels to the rgba format used internally.
    pub fn from_bgra(bgra: &[u8], size: impl Into<Size>) -> Result<Visual> {
        Visual::builder().build_from_bgra(bgra, size.into())
    }

    /// Builds a new `Visual` from a `file`, extracts the codec and parameters
    /// and decodes the first image to memory.
    ///
    /// It needs notcurses to be compiled with multimedia capabilities.
    pub fn from_file(self, file: &str) -> Result<Visual> {
        Visual::builder().build_from_file(file)
    }

    /// Builds a new `Visual` from a [`Plane`].
    ///
    /// The plane may contain only spaces, half blocks, and full blocks.
    /// This will be checked, and any other glyph will result in an error.
    ///
    /// This function exists so that planes can be subjected to `Visual` transformations.
    /// If possible, it's better to build the visual from memory using
    /// [`build_from_rgba`][Visual#method.build_from_rgba].
    ///
    /// Use `None` for either or both of `beg_y` and `beg_x` in order to
    /// use the current cursor position along that axis.
    ///
    /// Use `None` for either or both of `len_y` and `len_x` in order to
    /// go through the boundary of the plane in that axis (same as `0`).
    ///
    pub fn from_plane(
        plane: &Plane,
        blitter: Blitter,
        beg_y: Option<u32>,
        beg_x: Option<u32>,
        len_y: Option<u32>,
        len_x: Option<u32>,
    ) -> Result<Visual> {
        Visual::builder().build_from_plane(plane, blitter, beg_y, beg_x, len_y, len_x)
    }

    /// Returns a shared reference to the inner [`NcVisual`].
    pub fn into_ref(&self) -> &NcVisual {
        unsafe { &*self.nc }
    }

    /// Returns an exclusive reference to the inner [`NcVisual`].
    pub fn into_ref_mut(&mut self) -> &mut NcVisual {
        unsafe { &mut *self.nc }
    }
}

/// # `Visual` methods.
impl Visual {
    /// Renders the `Visual` to a new [`Plane`], which is returned.
    pub fn blit(&mut self, nc: &mut Notcurses) -> Result<Plane> {
        let vo: sys::NcVisualOptions = self.options.into();
        let ncplane = unsafe { self.into_ref_mut().blit(nc.into_ref_mut(), Some(&vo))? };
        Ok(ncplane.into())
    }

    /// Renders the `Visual` to an existing `target` [`Plane`].
    pub fn blit_plane(&mut self, nc: &mut Notcurses, target: &mut Plane) -> Result<()> {
        let mut vo: sys::NcVisualOptions = self.options.into();
        vo.n = target.into_ref_mut();
        let _ = unsafe { self.into_ref_mut().blit(nc.into_ref_mut(), Some(&vo))? };
        Ok(())
    }

    /// Renders the `Visual` to a new child [`Plane`] of a `parent` plane, which is returned.
    pub fn blit_child(&mut self, nc: &mut Notcurses, parent: &mut Plane) -> Result<Plane> {
        let mut vo: sys::NcVisualOptions = self.options.into();
        vo.n = parent.into_ref_mut();
        vo.flags |= sys::NcVisualFlag::ChildPlane;

        let ncplane_child = unsafe { self.into_ref_mut().blit(nc.into_ref_mut(), Some(&vo))? };
        Ok(ncplane_child.into())
    }

    //

    /// Sets the vertical placement, overriding vertical alignment.
    ///
    /// Default: *`0`*.
    pub fn set_y(&mut self, y: i32) {
        self.options.set_y(y);
    }

    /// Sets the horizontal placement, overriding horizontal alignment.
    ///
    /// Default: *`0`*.
    pub fn set_x(&mut self, x: i32) {
        self.options.set_x(x);
    }

    /// Sets both the vertical & horizontal placement,
    /// overriding both vertical & horizontal alignment.
    ///
    /// Default: *`(0, 0)`*.
    pub fn set_yx(&mut self, y: i32, x: i32) {
        self.options.set_y(y);
        self.options.set_x(x);
    }

    /// Sets the vertical alignment.
    ///
    /// Default: *[`Align::Top`]*.
    pub fn set_valign(&mut self, vertical: Align) {
        self.options.set_valign(vertical);
    }

    /// Sets the horizontal alignment.
    ///
    /// Default: *[`Align::Left`]*.
    pub fn set_halign(&mut self, horizontal: Align) {
        self.options.set_halign(horizontal);
    }

    /// Sets both the vertical & horizontal alignment.
    ///
    /// Default: *`(`[`Align::Top`]*`, `*[`Align::Left`]`)`*.
    pub fn set_align(&mut self, vertical: Align, horizontal: Align) {
        self.options.set_valign(vertical);
        self.options.set_halign(horizontal);
    }

    /// Sets the [`Scale`].
    ///
    /// Default: `Scale::None`.
    pub fn set_scale(&mut self, scale: Scale) {
        self.options.set_scale(scale);
    }

    /// Sets the [`Blitter`].
    ///
    /// Default: `Blitter::Default`.
    pub fn set_blitter(&mut self, blitter: Blitter) {
        self.options.set_blitter(blitter);
    }

    /// (Un)Sets this color as transparent.
    ///
    /// Default: `None`.
    pub fn set_transparency(&mut self, color: Option<Rgba>) {
        self.options.set_transparency(color);
    }

    /// Choose whether to use [`Alpha::Blend`] with the [`Visual`], so that
    /// the foreground or background colors can be a composite between
    /// a color and the corresponding colors underneath it.
    ///
    /// Default: *false* (no blend).
    ///
    /// [`Alpha::Blend`]: crate::Alpha#associatedconstant.Blend
    pub fn set_blend(&mut self, blend: bool) {
        self.options.set_blend(blend);
    }

    /// Choose between gracefully degrading the blitter, or fail if the choosen
    /// `Blitter` is not supported by the terminal.
    ///
    /// Default: true (degrade).
    pub fn set_degrade(&mut self, degrade: bool) {
        self.options.set_degrade(degrade);
    }

    /// Sets whether the scaling should be done with interpolation or not.
    ///
    /// Default: true (interpolate).
    pub fn set_interpolate(&mut self, interpolate: bool) {
        self.options.set_interpolate(interpolate);
    }

    /// Sets the region to be rendered.
    ///
    /// - `y`, `x`: origin of rendered region in pixels.
    /// - `len_y`, `len_x`: size of rendered region in pixels.
    pub fn set_region(&mut self, y: u32, x: u32, len_y: u32, len_x: u32) {
        self.options.set_region(Some((y, x, len_y, len_x)));
    }

    /// Sets the pixel offset within the [`Cell`][crate::Cell].
    pub fn set_cell_offset(&mut self, y: u32, x: u32) {
        self.options.set_cell_offset(Some((y, x)));
    }
}
