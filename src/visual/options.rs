// notcurses::visual::options
//
//!
//

use crate::{sys::NcVisualFlag, Align, Blitter, Rgba, Scale};

/// The inner options of a [`Visual`].
///
/// The main difference with [`NcVisualOptions`][crate::sys::NcVisualOptions]
/// is the absence of a reference to a [`Plane`].
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct VisualOptions {
    pub(crate) y: i32,
    pub(crate) x: i32,
    pub(crate) scale: Scale,
    pub(crate) blitter: Blitter,
    pub(crate) transcolor: Option<Rgba>,
    pub(crate) cell_offset_yx: Option<(u32, u32)>,
    pub(crate) region_yx_lenyx: Option<(u32, u32, u32, u32)>,
    pub(crate) flags: NcVisualFlag,
}

mod std_impls {
    use super::VisualOptions;
    use crate::sys::{NcVisualOptions, NcVisualOptionsBuilder};

    impl<'n> From<VisualOptions> for NcVisualOptionsBuilder<'n> {
        fn from(vo: VisualOptions) -> NcVisualOptionsBuilder<'n> {
            let mut builder = NcVisualOptionsBuilder::new();

            if vo.is_veraligned() {
                builder = builder.valign(vo.y);
            } else {
                builder = builder.y(vo.y);
            }
            if vo.is_horaligned() {
                builder = builder.halign(vo.x);
            } else {
                builder = builder.x(vo.x);
            }

            builder = builder.scale(vo.scale);
            builder = builder.blitter(vo.blitter);

            if let Some(color) = vo.transcolor {
                builder = builder.transcolor(Some(color));
            }

            if let Some((y, x)) = vo.cell_offset_yx {
                builder = builder.cell_offset(y, x);
            }
            if let Some((y, x, leny, lenx)) = vo.region_yx_lenyx {
                builder = builder.region(y, x, leny, lenx);
            }

            builder = builder.blend(vo.does_blend());
            builder = builder.degrade(vo.does_degrade());
            builder = builder.interpolate(vo.does_interpolate());

            builder
        }
    }

    impl From<VisualOptions> for NcVisualOptions {
        fn from(vo: VisualOptions) -> NcVisualOptions {
            let builder: NcVisualOptionsBuilder = vo.into();
            builder.build()
        }
    }

    impl From<NcVisualOptions> for VisualOptions {
        fn from(ncvo: NcVisualOptions) -> VisualOptions {
            let mut vo = VisualOptions::default();

            vo.set_blend(ncvo.does_blend());
            vo.set_degrade(ncvo.does_degrade());
            vo.set_interpolate(ncvo.does_interpolate());
            vo.set_blitter(ncvo.blitter.into());
            vo.set_scale(ncvo.scaling.into());

            if ncvo.does_alpha() {
                vo.set_transparency(Some(ncvo.transcolor.into()));
            }

            if ncvo.is_veraligned() {
                vo.set_valign(ncvo.y.into());
            }
            if ncvo.is_horaligned() {
                vo.set_halign(ncvo.x.into());
            }

            let cell_offset = (ncvo.pxoffy, ncvo.pxoffx);
            if cell_offset != (0, 0) {
                vo.set_cell_offset(Some(cell_offset));
            }

            let region = (ncvo.begy, ncvo.begx, ncvo.leny, ncvo.lenx);
            if region != (0, 0, 0, 0) {
                vo.set_region(Some(region));
            }

            vo
        }
    }

    impl<'n> From<NcVisualOptionsBuilder<'n>> for VisualOptions {
        fn from(ob: NcVisualOptionsBuilder<'n>) -> VisualOptions {
            ob.build().into()
        }
    }
}

/// # Methods for setting the options.
impl VisualOptions {
    /// Sets the vertical placement.
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
        self.flags &= !NcVisualFlag::VerAligned;
    }
    /// Sets the horizontal placement.
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
        self.flags &= !NcVisualFlag::HorAligned;
    }

    /// Sets the vertical alignment.
    pub fn set_valign(&mut self, vertical: Align) {
        self.y = vertical.into();
        self.flags |= NcVisualFlag::VerAligned;
    }
    /// Sets the horizontal alignment.
    pub fn set_halign(&mut self, horizontal: Align) {
        self.x = horizontal.into();
        self.flags |= NcVisualFlag::HorAligned;
    }

    /// Sets the region of the visual to be rendered.
    pub fn set_region(&mut self, region: Option<(u32, u32, u32, u32)>) {
        self.region_yx_lenyx = region;
    }

    /// Sets the pixel offset within the cell.
    pub fn set_cell_offset(&mut self, cell_offset: Option<(u32, u32)>) {
        self.cell_offset_yx = cell_offset;
    }

    /// Sets the blitter.
    pub fn set_blitter(&mut self, blitter: Blitter) {
        self.blitter = blitter;
    }

    /// Sets the scale.
    pub fn set_scale(&mut self, scale: Scale) {
        self.scale = scale;
    }

    /// (Un)Sets the transparent color.
    pub fn set_transparency(&mut self, color: Option<Rgba>) {
        if let Some(color) = color {
            self.flags |= NcVisualFlag::AddAlpha;
            self.transcolor = Some(color);
        } else {
            self.flags &= !NcVisualFlag::AddAlpha;
            self.transcolor = None;
        }
    }

    /// (Un)Sets blending.
    pub fn set_blend(&mut self, blend: bool) {
        if blend {
            self.flags |= NcVisualFlag::Blend;
        } else {
            self.flags &= !NcVisualFlag::Blend;
        }
    }

    /// (Un)Sets degradation.
    pub fn set_degrade(&mut self, degrade: bool) {
        if degrade {
            self.flags &= !NcVisualFlag::NoDegrade;
        } else {
            self.flags |= NcVisualFlag::NoDegrade;
        }
    }

    /// (Un)Sets interpolation.
    pub fn set_interpolate(&mut self, interpolate: bool) {
        if interpolate {
            self.flags &= !NcVisualFlag::NoInterpolate;
        } else {
            self.flags |= NcVisualFlag::NoInterpolate;
        }
    }
}

/// # Methods for checking the flags.
impl VisualOptions {
    /// Returns `true` if it has the `VerAligned` flag set.
    pub fn is_veraligned(&self) -> bool {
        self.flags & NcVisualFlag::VerAligned != NcVisualFlag::None
    }

    /// Returns `true` if it has the `HorAligned` flag set.
    pub fn is_horaligned(&self) -> bool {
        self.flags & NcVisualFlag::HorAligned != NcVisualFlag::None
    }

    /// Returns `true` if it has the `Blend` flag set.
    pub fn does_blend(&self) -> bool {
        self.flags & NcVisualFlag::Blend != NcVisualFlag::None
    }

    /// Returns `false` if it has the `NoDegrade` flag set.
    pub fn does_degrade(&self) -> bool {
        self.flags & NcVisualFlag::NoDegrade == NcVisualFlag::None
    }

    /// Returns `false` if it has the `NoInterpolate` flag set.
    pub fn does_interpolate(&self) -> bool {
        self.flags & NcVisualFlag::NoInterpolate == NcVisualFlag::None
    }
}
