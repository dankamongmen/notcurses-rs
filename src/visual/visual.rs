// notcurses::visual::visual
//
//!
//

use super::{Blitter, Scale, VisualBuilder, VisualGeometry, VisualOptions};
use crate::{
    color::Rgba,
    error::{NotcursesError as Error, NotcursesResult as Result},
    plane::{Align, Plane},
    sys::{self, NcRgba, NcVisual},
    Notcurses, Position, Size,
};

/// A visual bit of multimedia.
pub struct Visual {
    pub(super) nc: *mut NcVisual,
    pub(super) options: VisualOptions,
}

mod core_impls {
    use super::Visual;
    use core::fmt;

    impl Drop for Visual {
        #[inline]
        fn drop(&mut self) {
            if crate::Notcurses::is_initialized() {
                self.into_ref_mut().destroy()
            }
        }
    }

    impl fmt::Display for Visual {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write![f, "{}", self.options]
        }
    }

    impl fmt::Debug for Visual {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Visual {{ {:?} }}", self.options)
        }
    }
}

/// # `Visual` constructors and deconstructors.
impl Visual {
    /// Returns a new `Visual` builder.
    #[inline]
    pub fn builder() -> VisualBuilder {
        VisualBuilder::new()
    }

    /// Returns a new `Visual` from a byte buffer with RGBA content.
    #[inline]
    pub fn from_rgba(rgba: &[u8], size: impl Into<Size>) -> Result<Visual> {
        Visual::builder().build_from_rgba(rgba, size.into())
    }

    /// Builds a new `Visual` from a byte buffer with RGB content, providing
    /// the alpha to assign to all the pixels.
    #[inline]
    pub fn from_rgb(rgb: &[u8], size: impl Into<Size>, alpha: u8) -> Result<Visual> {
        Visual::builder().build_from_rgb(rgb, size.into(), alpha)
    }

    /// Builds a new `Visual` from a byte buffer with RGBX content, overriding
    /// the alpha byte *X* for all the pixels.
    #[inline]
    pub fn from_rgbx(rgbx: &[u8], size: impl Into<Size>, alpha: u8) -> Result<Visual> {
        Visual::builder().build_from_rgbx(rgbx, size.into(), alpha)
    }

    /// Builds a new `Visual` from a byte buffer with BGRA content.
    ///
    /// This is slower than [`build_from_rgba`][VisualBuilder#method.build_fromrgba],
    /// since it has to convert the pixels to the rgba format used internally.
    #[inline]
    pub fn from_bgra(bgra: &[u8], size: impl Into<Size>) -> Result<Visual> {
        Visual::builder().build_from_bgra(bgra, size.into())
    }

    /// Builds a new `Visual` from a `file`, extracts the codec and parameters
    /// and decodes the first image to memory.
    ///
    /// It needs notcurses to be compiled with multimedia capabilities.
    #[inline]
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
    #[inline]
    pub fn from_plane(
        plane: &Plane,
        blitter: Blitter,
        beg_x: Option<u32>,
        beg_y: Option<u32>,
        len_x: Option<u32>,
        len_y: Option<u32>,
    ) -> Result<Visual> {
        Visual::builder().build_from_plane(plane, blitter, beg_x, beg_y, len_x, len_y)
    }

    /// Returns a shared reference to the inner [`NcVisual`].
    #[inline]
    pub fn into_ref(&self) -> &NcVisual {
        unsafe { &*self.nc }
    }

    /// Returns an exclusive reference to the inner [`NcVisual`].
    #[inline]
    pub fn into_ref_mut(&mut self) -> &mut NcVisual {
        unsafe { &mut *self.nc }
    }

    /// Returns the visual options.
    #[inline]
    pub fn options(&self) -> VisualOptions {
        self.options
    }

    /// Sets the visual `options`.
    #[inline]
    pub fn set_options(&mut self, options: VisualOptions) {
        self.options = options;
    }
}

/// # `Visual` methods.
impl Visual {
    /// Renders the `Visual` to a new [`Plane`], which is returned.
    #[inline]
    pub fn blit(&mut self, nc: &mut Notcurses) -> Result<Plane> {
        let vo: sys::NcVisualOptions = self.options.into();
        let ncplane = unsafe { self.into_ref_mut().blit(nc.into_ref_mut(), Some(&vo))? };
        Ok(ncplane.into())
    }

    /// Renders the `Visual` to an existing `target` [`Plane`].
    #[inline]
    pub fn blit_plane(&mut self, nc: &mut Notcurses, target: &mut Plane) -> Result<()> {
        let mut vo: sys::NcVisualOptions = self.options.into();
        vo.n = target.into_ref_mut();
        let _ = unsafe { self.into_ref_mut().blit(nc.into_ref_mut(), Some(&vo))? };
        Ok(())
    }

    /// Renders the `Visual` to a new child [`Plane`] of a `parent` plane, which is returned.
    #[inline]
    pub fn blit_child(&mut self, nc: &mut Notcurses, parent: &mut Plane) -> Result<Plane> {
        let mut vo: sys::NcVisualOptions = self.options.into();
        vo.n = parent.into_ref_mut();
        vo.flags |= sys::NcVisualFlag::ChildPlane;

        let ncplane_child = unsafe { self.into_ref_mut().blit(nc.into_ref_mut(), Some(&vo))? };
        Ok(ncplane_child.into())
    }

    //

    /// Returns the visual geometry.
    #[inline]
    pub fn geometry(&self, notcurses: &Notcurses) -> Result<VisualGeometry> {
        Ok(self
            .into_ref()
            .geom(Some(notcurses.into_ref()), Some(&self.options().into()))?
            .into())
    }

    /// Returns the internal size of the visual, in pixels.
    #[inline]
    pub fn size(&self) -> Result<Size> {
        self.into_ref()
            .geom(None, Some(&self.options().into()))?
            .pix_yx
            .map(|s| Size::from(s).swapped())
            .ok_or_else(|| Error::Message("visual size error".to_string()))
    }

    // ­--

    /// Resizes the visual to the new `size` using bilinear interpolation.
    ///
    /// This is a lossy transformation, unless the size is unchanged.
    #[inline]
    pub fn resize(&mut self, size: Size) -> Result<()> {
        let (w, h) = size.into();
        Ok(self.into_ref_mut().resize(h, w)?)
    }

    /// Resizes the visual to the new `size` using nearest neighbor interpolation.
    ///
    /// This is a lossy transformation, unless the size is unchanged.
    #[inline]
    pub fn resize_nearest(&mut self, size: Size) -> Result<()> {
        let (w, h) = size.into();
        Ok(self.into_ref_mut().resize_noninterpolative(h, w)?)
    }

    //

    /// Rotates the visual a number of `radians`.
    ///
    /// Only M_PI/2 and -M_PI/2 are supported at the moment.
    #[inline]
    pub fn rotate(&mut self, radians: f64) -> Result<()> {
        Ok(self.into_ref_mut().rotate(radians)?)
    }

    //

    /// Sets the horizontal placement, overriding horizontal alignment.
    ///
    /// Default: *`0`*.
    #[inline]
    pub fn set_x(&mut self, x: i32) {
        self.options.set_x(x);
    }

    /// Sets the vertical placement, overriding vertical alignment.
    ///
    /// Default: *`0`*.
    #[inline]
    pub fn set_y(&mut self, y: i32) {
        self.options.set_y(y);
    }

    /// Sets both the horizontal & vertical placement,
    /// overriding both horizontal & vertical alignment.
    ///
    /// Default: *`(0, 0)`*.
    #[inline]
    pub fn set_xy(&mut self, x: i32, y: i32) {
        self.options.set_x(x);
        self.options.set_y(y);
    }

    /// Convenience wrapper around [`set_yx`][Visual#method.yx].
    #[inline]
    pub fn set_position(&mut self, position: Position) {
        let (x, y) = position.into();
        self.set_xy(x, y);
    }

    /// Sets the horizontal alignment.
    ///
    /// Default: *[`Align::Left`]*.
    #[inline]
    pub fn set_halign(&mut self, horizontal: Align) {
        self.options.set_halign(horizontal);
    }

    /// Sets the vertical alignment.
    ///
    /// Default: *[`Align::Top`]*.
    #[inline]
    pub fn set_valign(&mut self, vertical: Align) {
        self.options.set_valign(vertical);
    }

    /// Sets both the vertical & horizontal alignment.
    ///
    /// Default: *`(`[`Align::Top`]*`, `*[`Align::Left`]`)`*.
    #[inline]
    pub fn set_align(&mut self, vertical: Align, horizontal: Align) {
        self.options.set_halign(horizontal);
        self.options.set_valign(vertical);
    }

    /// Sets the [`Scale`].
    ///
    /// Default: `Scale::None`.
    #[inline]
    pub fn set_scale(&mut self, scale: Scale) {
        self.options.set_scale(scale);
    }

    /// Sets the [`Blitter`].
    ///
    /// Default: `Blitter::Default`.
    #[inline]
    pub fn set_blitter(&mut self, blitter: Blitter) {
        self.options.set_blitter(blitter);
    }

    /// Sets the [`Pixel`][Blitter::Pixel] blitter.
    #[inline]
    pub fn set_blitter_pixel(&mut self) {
        self.options.set_blitter(Blitter::Pixel);
    }

    /// Gets the Rgba pixel at the provided coordinates.
    ///
    /// *Corresponds to [`NcVisual::at_yx`].*
    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<Rgba> {
        let ncrgba: NcRgba = self.into_ref().at_yx(y, x)?.into();
        Ok(ncrgba.into())
    }

    /// Sets the Rgba pixel at the provided coordinates.
    ///
    /// *Corresponds to [`NcVisual::set_yx`].*
    #[inline]
    pub fn set_pixel(&mut self, x: u32, y: u32, rgba: impl Into<Rgba>) -> Result<()> {
        let ncrgba: NcRgba = rgba.into().into();
        self.into_ref_mut().set_yx(y, x, ncrgba)?;
        Ok(())
    }

    /// (Un)Sets graful degradation.
    ///
    /// Choose between gracefully degrading the blitter, or fail if the choosen
    /// `Blitter` is not supported by the terminal.
    ///
    /// Default: true (degrade).
    #[inline]
    pub fn set_degrade(&mut self, degrade: bool) {
        self.options.set_degrade(degrade);
    }

    /// (Un)Sets this color as transparent.
    ///
    /// Default: `None`.
    #[inline]
    pub fn set_transparency(&mut self, color: Option<Rgba>) {
        self.options.set_transparency(color);
    }

    /// (Un)Sets alpha blending.
    ///
    /// Choose whether to use [`Alpha::Blend`] with the [`Visual`], so that
    /// the foreground or background colors can be a composite between
    /// a color and the corresponding colors underneath it.
    ///
    /// Default: *false* (no blend).
    ///
    /// [`Alpha::Blend`]: crate::color::Alpha#associatedconstant.Blend
    #[inline]
    pub fn set_blend(&mut self, blend: bool) {
        self.options.set_blend(blend);
    }

    /// (Un)Sets scaling interpolation.
    ///
    /// Default: true (interpolate).
    #[inline]
    pub fn set_interpolate(&mut self, interpolate: bool) {
        self.options.set_interpolate(interpolate);
    }

    /// Sets the region to be rendered.
    ///
    /// - `y`, `x`: origin of the rendered region in pixels.
    /// - `len_y`, `len_x`: size of the rendered region in pixels.
    #[inline]
    pub fn set_region(&mut self, x: u32, y: u32, len_x: u32, len_y: u32) {
        self.options.set_region(Some((x, y, len_x, len_y)));
    }

    /// Sets the pixel offset within the [`Cell`][crate::plane::Cell].
    #[inline]
    pub fn set_cell_offset(&mut self, x: u32, y: u32) {
        self.options.set_cell_offset(Some((x, y)));
    }
}
