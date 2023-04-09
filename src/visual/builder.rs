// notcurses::visual::builder
//
//!
//

use crate::{
    color::{Palette, Rgba},
    error::NotcursesResult as Result,
    plane::{Align, Plane},
    sys::NcVisual,
    visual::{Blitter, Scale, Visual, VisualOptions},
    Position, Size,
};

/// A [`Visual`] builder.
#[derive(Debug, Default)]
pub struct VisualBuilder {
    options: VisualOptions,
}

/// # Constructors
impl VisualBuilder {
    /// Returns a new default `VisualBuilder`.
    ///
    /// Size, position and margins are set to 0.
    /// The plane will be maximized to its parent size.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a new `Visual` from a byte buffer with RGBA content.
    pub fn build_from_rgba(self, rgba: &[u8], size: impl Into<Size>) -> Result<Visual> {
        let (w, h) = size.into().into();
        let ncvisual = NcVisual::from_rgba(rgba, h, w * 4, w)?;
        Ok(Visual {
            nc: ncvisual,
            options: self.options,
        })
    }

    /// Builds a new `Visual` from a byte buffer with RGB content, providing
    /// the alpha to assign to all the pixels.
    pub fn build_from_rgb(self, rgb: &[u8], size: impl Into<Size>, alpha: u8) -> Result<Visual> {
        let (w, h) = size.into().into();
        let ncvisual = NcVisual::from_rgb_packed(rgb, h, w * 3, w, alpha)?;
        Ok(Visual {
            nc: ncvisual,
            options: self.options,
        })
    }

    /// Builds a new `Visual` from a byte buffer with RGBX content, overriding
    /// the alpha byte *X* for all the pixels.
    pub fn build_from_rgbx(self, rgbx: &[u8], size: impl Into<Size>, alpha: u8) -> Result<Visual> {
        let (w, h) = size.into().into();
        let ncvisual = NcVisual::from_rgb_loose(rgbx, h, w * 4, w, alpha)?;
        Ok(Visual {
            nc: ncvisual,
            options: self.options,
        })
    }

    /// Builds a new `Visual` from a byte buffer with BGRA content.
    ///
    /// This is slower than [`build_from_rgba`][VisualBuilder#method.build_fromrgba],
    /// since it has to convert the pixels to the rgba format used internally.
    pub fn build_from_bgra(self, bgra: &[u8], size: impl Into<Size>) -> Result<Visual> {
        let (w, h) = size.into().into();
        let ncvisual = NcVisual::from_bgra(bgra, h, w * 4, w)?;
        Ok(Visual {
            nc: ncvisual,
            options: self.options,
        })
    }

    /// Builds a new `Visual` from a `file`, extracts the codec and parameters
    /// and decodes the first image to memory.
    ///
    /// It needs notcurses to be compiled with multimedia capabilities.
    pub fn build_from_file(self, file: &str) -> Result<Visual> {
        let ncvisual = NcVisual::from_file(file)?;
        Ok(Visual {
            nc: ncvisual,
            options: self.options,
        })
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
    pub fn build_from_plane(
        self,
        plane: &Plane,
        blitter: Blitter,
        beg_x: Option<u32>,
        beg_y: Option<u32>,
        len_x: Option<u32>,
        len_y: Option<u32>,
    ) -> Result<Visual> {
        let ncvisual = NcVisual::from_plane(plane.into_ref(), blitter, beg_y, beg_x, len_y, len_x)?;
        Ok(Visual {
            nc: ncvisual,
            options: self.options,
        })
    }

    /// Builds a new `Visual` from a nul-terminated Sixel control `sequence`.
    pub fn build_from_sixel(self, sequence: &str, len_x: u32, len_y: u32) -> Result<Visual> {
        let ncvisual = NcVisual::from_sixel(sequence, len_y, len_x)?;
        Ok(Visual {
            nc: ncvisual,
            options: self.options,
        })
    }

    /// Builds a new `Visual` from `pstride`-byte palette-indexed pixels, arranged in
    /// `y` lines of `stride` bytes each, composed of `x` pixels.
    // IMPROVE
    #[allow(clippy::too_many_arguments)]
    pub fn build_from_palidx(
        self,
        data: &[u8],
        x: u32,
        y: u32,
        stride: u32,
        palsize: u8,
        pstride: u32,
        palette: Palette,
    ) -> Result<Visual> {
        let ncvisual =
            NcVisual::from_palidx(data, y, stride, x, palsize, pstride, palette.into_ref())?;
        Ok(Visual {
            nc: ncvisual,
            options: self.options,
        })
    }
}

/// # Methods (chainable)
impl VisualBuilder {
    /// Sets the horizontal placement. Default: *`0`*.
    pub fn x(mut self, x: i32) -> Self {
        self.options.set_x(x);
        self
    }
    /// Sets the vertical placement. Default: *`0`*.
    pub fn y(mut self, y: i32) -> Self {
        self.options.set_y(y);
        self
    }
    /// Sets the vertical & horizontal placement. Default: *`(0, 0)`*.
    pub fn xy(mut self, x: i32, y: i32) -> Self {
        self.options.set_x(x);
        self.options.set_y(y);
        self
    }

    /// Convenience wrapper around [`yx`][VisualBuilder#method.yx].
    pub fn position(self, position: Position) -> Self {
        let (x, y) = position.into();
        self.xy(x, y)
    }

    /// Sets the horizontal alignment. Default: *[`Align::Left`]*.
    pub fn halign(mut self, horizontal: Align) -> Self {
        self.options.set_halign(horizontal);
        self
    }
    /// Sets the vertical alignment. Default: *[`Align::Top`]*.
    pub fn valign(mut self, vertical: Align) -> Self {
        self.options.set_valign(vertical);
        self
    }
    /// Sets both the horizontal & vertical  alignment.
    /// Default: *`(`[`Align::Top`]*`, `*[`Align::Left`]`)`*.
    pub fn align(mut self, horizontal: Align, vertical: Align) -> Self {
        self.options.set_halign(horizontal);
        self.options.set_valign(vertical);
        self
    }

    /// Sets the [`Scale`]. Default: *[`Scale::None`]*.
    pub fn scale(mut self, scale: Scale) -> Self {
        self.options.set_scale(scale);
        self
    }

    /// Sets the [`Blitter`]. Default: *[`Blitter::Default`]*.
    pub fn blitter(mut self, blitter: Blitter) -> Self {
        self.options.set_blitter(blitter);
        self
    }

    /// Sets the [`Pixel`][Blitter::Pixel] blitter.
    pub fn blitter_pixel(mut self) -> Self {
        self.options.set_blitter(Blitter::Pixel);
        self
    }

    /// Choose between gracefully degrading the blitter, or fail if the choosen
    /// `Blitter` is not supported by the terminal.
    ///
    /// Default: true (degrade).
    pub fn degrade(mut self, degrade: bool) -> Self {
        self.options.set_degrade(degrade);
        self
    }

    /// (Un)Sets some color as transparent. Default: `None`.
    pub fn transparency(mut self, color: Option<Rgba>) -> Self {
        self.options.set_transparency(color);
        self
    }

    /// Choose whether to use [`Alpha::Blend`] with the [`Visual`], so that
    /// the foreground or background colors can be a composite between
    /// a color and the corresponding colors underneath it.
    ///
    /// Default: *false* (no blend).
    ///
    /// [`Alpha::Blend`]: crate::color::Alpha#associatedconstant.Blend
    pub fn blend(mut self, blend: bool) -> Self {
        self.options.set_blend(blend);
        self
    }

    /// Sets whether the scaling should be done with interpolation or not.
    /// Default: true (interpolate).
    pub fn interpolate(mut self, interpolate: bool) -> Self {
        self.options.set_interpolate(interpolate);
        self
    }

    /// Sets the region to be rendered.
    ///
    /// - `y`, `x`: origin of rendered region in pixels.
    /// - `len_y`, `len_x`: size of rendered region in pixels.
    pub fn region(mut self, x: u32, y: u32, len_x: u32, len_y: u32) -> Self {
        self.options.set_region(Some((x, y, len_x, len_y)));
        self
    }

    /// Sets the pixel offset within the [`Cell`][crate::plane::Cell].
    pub fn cell_offset(mut self, x: u32, y: u32) -> Self {
        self.options.set_cell_offset(Some((x, y)));
        self
    }
}
