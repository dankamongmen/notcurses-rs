//!

use crate::sys::{self, NcVisual, NcVisualOptions};
use crate::{Align, Blitter, NotcursesError, NotcursesResult as Result, Plane, Scale, Visual};

/// A [`Visual`] builder.
#[derive(Default)]
pub struct VisualBuilder<'ncvisual, 'ncplane, 'plane> {
    ncvisual: Option<&'ncvisual mut NcVisual>,

    plane: Option<&'plane mut Plane<'ncplane>>,
    scale: Scale,

    x: u32,
    y: u32,

    halign: Option<Align>,
    valign: Option<Align>,

    begx: u32,
    begy: u32,
    lenx: u32,
    leny: u32,

    blitter: Blitter,

    flags: u32,
    transcolor: u32,
}

impl<'ncvisual, 'ncplane, 'plane> VisualBuilder<'ncvisual, 'ncplane, 'plane> {
    /// Prepares a `Visual` based off RGBA content in memory at `rgba`.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_rgba(mut self, rgba: &[u8], cols: u32, rows: u32) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_rgba(rgba, rows, cols * 4, cols)?);
        Ok(self)
    }

    /// Prepares a `Visual` based off RGB content in memory at `rgb`, providing
    /// the alpha to assign to all the pixels.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_rgb(mut self, rgb: &[u8], cols: u32, rows: u32, alpha: u8) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_rgb_packed(rgb, rows, cols * 3, cols, alpha)?);
        Ok(self)
    }

    /// Prepares a `Visual` based off RGBX content in memory at `rgbx`,
    /// overriding the *alpha* byte *X* for all the pixels.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_rgbx(mut self, rgbx: &[u8], cols: u32, rows: u32, alpha: u8) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_rgb_loose(rgbx, rows, cols * 4, cols, alpha)?);
        Ok(self)
    }

    /// Prepares a `Visual` based off BGRA content in memory at `bgra`.
    ///
    /// This is slower than [`from_rgba`][VisualBuilder#method.rgba], since it
    /// has to convert the pixels to the rgba format used internally.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_bgra(mut self, bgra: &[u8], cols: u32, rows: u32) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_bgra(bgra, rows, cols * 4, cols)?);
        Ok(self)
    }

    /// Prepares a `Visual` from a `file`, extracts the codec and paramenters
    /// and decodes the first image to memory.
    ///
    /// It needs notcurses to be compiled with multimedia capabilities.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_file(mut self, file: &str) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_file(file)?);
        Ok(self)
    }

    /// Prepares a `Visual` from a [`Plane`].
    ///
    /// The plane may contain only spaces, half blocks, and full blocks.
    /// This will be checked, and any other glyph will result in an error.
    ///
    /// This function exists so that planes can be subjected to NcVisual transformations.
    /// If possible, it's better to create the ncvisual from memory using
    /// [from_rgba][Visual#method.from_rgba].
    ///
    // RETHINK whether to name it y1,x1 or leny, lenx
    #[allow(clippy::wrong_self_convention)]
    pub fn from_plane(
        mut self,
        plane: &Plane<'ncplane>,
        blitter: Blitter,
        x0: u32,
        y0: u32,
        x1: u32,
        y1: u32,
    ) -> Result<Self> {
        self.ncvisual = Some(NcVisual::from_plane(
            plane.raw,
            blitter.into(),
            y0,
            x0,
            y1,
            x1,
        )?);
        Ok(self)
    }

    // MAYBE
    // /// Creates a `VisualBuilder` from an already configured [`Visual`].
    // pub fn from_visual(visual: Visual<'ncvisual>) -> Self {
    //     // if let visua
    //     // let plane =
    //     Self {
    //         //ncvisual: Some(visual.raw),
    //         ..Default::default()
    //     }
    // }

    /// Sets the [`Plane`] used by the rendering functions. Default: Not set.
    pub fn plane(mut self, plane: &'plane mut Plane<'ncplane>) -> Self {
        self.plane = Some(plane);
        self
    }

    /// Sets the `x,y` coordinates.
    ///
    // TODO: make clearer:
    /// - If you don't provide a pre-existing `Plane`, they will be relative to
    ///   the terminal size.
    /// - If you do provide a pre-existing `Plane` via the
    ///   [`plane`][VisualBuilder#method.plane] method,
    ///   they indicate where in that `Plane` to start drawing the `Visual`.
    /// - If you provide a [`parent_plane`][VisualBuilder#method.parent_plane]
    ///   they are interpreted relative to that `Plane`.
    ///
    /// This will override any relative [`halign`][VisualBuilder#method.halign]
    /// and [`valign`][VisualBuilder#method.halign] positioning.
    pub fn xy(mut self, x: u32, y: u32) -> Self {
        self.x = x;
        self.y = y;
        self.flags &= !sys::NCVISUAL_OPTION_HORALIGNED;
        self.flags &= !sys::NCVISUAL_OPTION_VERALIGNED;
        self
    }

    /// Sets the `x` cooordinate.
    ///
    /// This will override any relative [`halign`][VisualBuilder#method.halign]
    /// vertical positioning.
    pub fn x(mut self, x: u32) -> Self {
        self.x = x;
        self.halign = None;
        self.flags &= !sys::NCVISUAL_OPTION_HORALIGNED;
        self
    }

    /// Sets the `y` cooordinate.
    ///
    /// This will override any relative [`valign`][VisualBuilder#method.valign]
    /// vertical positioning.
    pub fn y(mut self, y: u32) -> Self {
        self.y = y;
        self.valign = None;
        self.flags &= !sys::NCVISUAL_OPTION_VERALIGNED;
        self
    }

    /// The `Visual` will be horizontally aligned.
    ///
    /// This will override any absolute [`y`][VisualBuilder#method.y] positioning.
    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self.flags |= sys::NCVISUAL_OPTION_HORALIGNED;
        self
    }

    /// The `Visual` will be horizontally aligned.
    ///
    /// This will override any absolute [`y`][VisualBuilder#method.y] positioning.
    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self.flags |= sys::NCVISUAL_OPTION_VERALIGNED;
        self
    }

    /// TODO: description
    ///
    /// - `x0`,`y0` are the origin coordinates of the rendering section
    /// - `x1`,`y1` are the size of the rendering section
    pub fn section(mut self, x0: u32, y0: u32, x1: u32, y1: u32) -> Self {
        self.begx = x0;
        self.begy = y0;
        self.lenx = x1;
        self.leny = y1;
        self
    }

    /// Sets the [`Blitter`]. Default: `Blitter::Default`.
    pub fn blitter(mut self, blitter: Blitter) -> Self {
        self.blitter = blitter;
        self
    }

    /// Sets the [`Scale`]. Default: `Scale::None`.
    pub fn scale(mut self, scale: Scale) -> Self {
        self.scale = scale;
        self
    }

    /// Will treat this RGB color as transparent. Default: `None`.
    pub fn transparent_color(mut self, color: Option<u32>) -> Self {
        if let Some(color) = color {
            self.flags |= sys::NCVISUAL_OPTION_ADDALPHA;
            self.transcolor = color;
        } else {
            self.flags &= !sys::NCVISUAL_OPTION_ADDALPHA;
            self.transcolor = 0;
        }
        self
    }

    /// Sets whether the scaling should be done with interpolation or not.
    /// Default: do interpolate.
    pub fn interpolate(mut self, interpolate: bool) -> Self {
        if interpolate {
            self.flags &= !sys::NCVISUAL_OPTION_NOINTERPOLATE;
        } else {
            self.flags |= sys::NCVISUAL_OPTION_NOINTERPOLATE;
        }
        self
    }

    // BUILD FINISHERS

    /// Finishes the build and returns a `Visual`.
    // TODO:IMPROVE
    // - save scale, even if not
    // - MAYBE
    //   1. separate the options from VisualBuilder into an external private structure
    //   2. use the options structure also in Visual, and move the  conversion to
    //      ncvisual options inside this method as a new private Visual method.
    pub fn finish(self) -> Result<Visual<'ncvisual>> {
        if self.ncvisual.is_some() {
            let ncvisualopt = if let Some(plane) = self.plane {
                NcVisualOptions::with_plane(
                    plane.as_ncplane_mut(),
                    self.scale as u32,
                    self.y,
                    self.x,
                    self.begy,
                    self.begx,
                    self.leny,
                    self.lenx,
                    self.blitter.into(),
                    self.flags,
                    self.transcolor,
                )
            } else {
                NcVisualOptions::without_plane(
                    // scale preference should be taken into account
                    self.y,
                    self.x,
                    self.begy,
                    self.begx,
                    self.leny,
                    self.lenx,
                    self.blitter.into(),
                    self.flags,
                    self.transcolor,
                )
            };

            Ok(Visual {
                options: ncvisualopt,
                raw: self.ncvisual.unwrap(),
            })
        } else {
            Err(NotcursesError::BuildIncomplete(
                "It's necessary to prepare the Visual
                first by calling one of the `from_*` methods."
                    .into(),
            ))
        }
    }

    // MAYBE
    // /// Fills the relevant `VisualBuilder` fields from an `NcVisualOptions`.
    // fn disassemble_options(&mut self, &NcVisualOptions) {
    // }
}
