//!
//! - <https://notcurses.com/notcurses_visual.3.html>

#![allow(dead_code)]

// TODO
// - NcRgba, NcVGeom...
// - allow changing the inner options after, with a safe interface
// - add alpha_color NCVISUAL_OPTION_ADDALPHA
// - add halign & valign
// - add blend NCVISUAL_OPTION_BLEND
// - add nodegrade NCVISUAL_OPTION_NODEGRADE
//
// MAYBE
// - offer the alternative of using a VisualOptions structure. (old: visual3)

use crate::{
    ncresult,
    sys::{self, NcVisual, NcVisualOptions},
    NResult, Notcurses, Plane,
};

mod blitter;
mod builder;
mod pixelgeometry;
mod rgba;
mod scale;

pub use blitter::Blitter;
pub use builder::VisualBuilder;
pub use pixelgeometry::PixelGeometry;
pub use rgba::Rgba;
pub use scale::Scale;

/// A virtual [`Rgba`] pixel framebuffer.
#[derive(Debug)]
pub struct Visual<'ncvisual> {
    pub(crate) ncvisual: &'ncvisual mut NcVisual,
    pub(crate) options: NcVisualOptions,
}

impl<'ncvisual> Drop for Visual<'ncvisual> {
    /// Destroys the Visual.
    ///
    /// Rendered elements will not be disrupted, but the visual can be neither
    /// decoded nor rendered any further.
    fn drop(&mut self) {
        let _ = self.ncvisual.destroy();
    }
}

/// # Methods
impl<'ncvisual, 'ncplane, 'plane> Visual<'ncvisual> {
    /// Returns a default [`VisualBuilder`] used to customize a new `Visual`.
    pub fn build() -> VisualBuilder<'ncvisual, 'ncplane, 'plane> {
        VisualBuilder::default()
    }

    // /// Creates a `Visual` from an existing [`NcVisual`] and [`NcVisualOptions`].
    // pub fn from_ncvisual(visual: &'ncvisual mut NcVisual) -> Visual<'ncvisual> {
    //     Self {
    //         ncvisual: visual,
    //         // options: None,
    //     }
    // }

    pub fn as_ncvisual(&self) -> &NcVisual {
        self.ncvisual
    }

    /// Returns a mutable reference to the inner `NcVisual`.
    pub fn as_ncvisual_mut(&mut self) -> &mut NcVisual {
        self.ncvisual
    }

    /// Resizes the visual to `x`,`y` pixels, using interpolation.
    pub fn resize(&mut self, x: u32, y: u32) -> NResult<()> {
        ncresult![NcVisual::resize(self.ncvisual, y, x)]
    }

    /// Resizes the visual to `x`,`y` pixels, without using interpolation.
    pub fn resize_ni(&mut self, x: u32, y: u32) -> NResult<()> {
        ncresult![NcVisual::resize_noninterpolative(self.ncvisual, y, x)]
    }

    /// Renders the decoded frame to the configured [`Plane`][crate::Plane].
    pub fn render_plane(&mut self, nc: &mut Notcurses) -> NResult<()> {
        assert![!self.options.n.is_null()];
        self.options.flags &= !sys::NCVISUAL_OPTION_CHILDPLANE as u64;
        let _ = NcVisual::render(self.ncvisual, nc.nc, &self.options)?;
        Ok(())
    }

    /// Renders the decoded frame as a new plane, that is a child of the configured
    /// [`Plane`][crate::Plane], and returns it.
    pub fn render_child_plane(
        &'ncvisual mut self,
        nc: &mut Notcurses,
    ) -> NResult<Plane<'ncvisual>> {
        assert![!self.options.n.is_null()];
        self.options.flags |= sys::NCVISUAL_OPTION_CHILDPLANE as u64;
        let child_plane = NcVisual::render(self.ncvisual, nc.nc, &self.options)?;
        Ok(Plane::<'ncvisual> { ncplane: child_plane })
    }

    /// Renders the decoded frame as a new [`Plane`][crate::Plane], and returns it.
    ///
    /// Doesn't need to have a plane configured.
    pub fn render_new_plane(&'ncvisual mut self, nc: &mut Notcurses) -> NResult<Plane<'ncvisual>> {
        self.options.flags |= sys::NCVISUAL_OPTION_CHILDPLANE as u64;
        let child_ncplane = NcVisual::render(self.ncvisual, nc.nc, &self.options)?;
        Ok(child_ncplane.into())
    }
}

/// # Post-Builder Configuration Methods
///
/// These methods allows to re-configure a `Visual` after it has been built
/// via [`VisualBuilder`].
impl<'ncvisual, 'ncplane> Visual<'ncvisual> {
    /// (re)Sets the `Visual` based off RGBA content in memory at `rgba`.
    pub fn set_from_rgba(&mut self, rgba: &[u8], cols: u32, rows: u32) -> NResult<()> {
        self.ncvisual = NcVisual::from_rgba(rgba, rows, cols * 4, cols)?;
        Ok(())
    }

    /// (re)Sets the `Visual` based off RGB content in memory at `rgb`.
    pub fn set_from_rgb(&mut self, rgb: &[u8], cols: u32, rows: u32, alpha: u8) -> NResult<()> {
        self.ncvisual = NcVisual::from_rgb_packed(rgb, rows, cols * 4, cols, alpha)?;
        Ok(())
    }

    /// (re)Sets the `Visual` based off RGBX content in memory at `rgbx`.
    pub fn set_from_rgbx(&mut self, rgbx: &[u8], cols: u32, rows: u32, alpha: u8) -> NResult<()> {
        self.ncvisual = NcVisual::from_rgb_loose(rgbx, rows, cols * 4, cols, alpha)?;
        Ok(())
    }

    /// (re)Sets the `Visual` based off BGRA content in memory at `bgra`.
    pub fn set_from_bgra(&mut self, bgra: &[u8], cols: u32, rows: u32) -> NResult<()> {
        self.ncvisual = NcVisual::from_bgra(bgra, rows, cols * 4, cols)?;
        Ok(())
    }

    /// (re)Sets the `Visual` from a `file`, extracts the codec and paramenters
    /// and decodes the first image to memory.
    pub fn set_from_file(&mut self, file: &str) -> NResult<()> {
        self.ncvisual = NcVisual::from_file(file)?;
        Ok(())
    }

    /// (re)Sets the [`Blitter`]. Default: `Blitter::Default`.
    pub fn set_blitter(&mut self, blitter: Blitter) {
        self.options.blitter = blitter.into();
    }

    /// (re)Sets the [`Scale`]. Default: `Blitter::Default`.
    pub fn set_scale(&mut self, scale: Scale) {
        self.options.scaling = scale.into();
    }

    /// (re)Sets the [`Plane`] used by the rendering functions. Default: unset.
    pub fn set_plane(&mut self, plane: &mut Plane<'ncplane>) {
        self.options.n = plane.as_ncplane_mut();
    }

    /// Unsets the [`Plane`]. The plane is unset by default.
    pub fn unset_plane(&mut self) {
        self.options.n = core::ptr::null_mut();
    }

    /// Sets whether the scaling should be done with interpolation or not.
    ///
    /// The default is to interpolate.
    pub fn set_interpolate(&mut self, interpolate: bool) {
        if interpolate {
            self.options.flags &= !sys::NCVISUAL_OPTION_NOINTERPOLATE as u64;
        } else {
            self.options.flags |= sys::NCVISUAL_OPTION_NOINTERPOLATE as u64;
        }
    }

    /// Sets the RGB color to be treated as transparent. Default: `None`.
    pub fn set_transparent_color(mut self, color: Option<u32>) -> Self {
        if let Some(color) = color {
            self.options.flags |= sys::NCVISUAL_OPTION_ADDALPHA as u64;
            self.options.transcolor = color;
        } else {
            self.options.flags &= !sys::NCVISUAL_OPTION_ADDALPHA as u64;
            self.options.transcolor = 0;
        }
        self
    }
}
