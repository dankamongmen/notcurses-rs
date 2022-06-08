// notcurses::notcurses::statistics
//
//!
//

use crate::{sys::NcStats, Notcurses};

/// Runtime statistics.
#[derive(Clone, PartialEq, Eq)]
pub struct Statistics {
    nc: *mut NcStats,
}

mod std_impls {
    use super::{NcStats, Statistics};
    use crate::sys::c_api::libc::free;
    use std::fmt;

    impl Drop for Statistics {
        fn drop(&mut self) {
            unsafe { free(self.into_ref_mut() as *mut NcStats as *mut core::ffi::c_void) }
        }
    }

    impl fmt::Debug for Statistics {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Statistics {{
  renders: ok:{} err:{} ns:[{} max:{} min:{}]
  rasters: ok:{} err:{} ns:[{} max:{} min:{}] bytes:[{} max:{} min:{}]
  cells:[{} eli:{} changes:{}] sprixel:[{} eli:{} bytes:{} changes:{}]
  fg:[{} eli:{}] bg:[{} eli:{}] default:[{} eli:{}]
  input:[{} err:{} hpa:{}]
  planes:{} fb_bytes:{} refreshes:{} app_sync:{}
}}",
                self.renders(),
                self.failed_renders(),
                self.render_ns(),
                self.render_max_ns(),
                self.render_min_ns(),
                //
                self.writeouts(),
                self.failed_writeouts(),
                self.raster_ns(),
                self.raster_max_ns(),
                self.raster_min_ns(),
                self.raster_bytes(),
                self.raster_max_bytes(),
                self.raster_min_bytes(),
                //
                self.cell_emissions(),
                self.cell_elisions(),
                self.cell_geo_changes(),
                //
                self.sprixel_emissions(),
                self.sprixel_elisions(),
                self.sprixel_bytes(),
                self.pixel_geo_changes(),
                //
                self.fg_emissions(),
                self.fg_elisions(),
                self.bg_emissions(),
                self.bg_elisions(),
                self.default_emissions(),
                self.default_elisions(),
                //
                self.input_events(),
                self.input_errors(),
                self.hpa_gratuitous(),
                //
                self.planes(),
                self.fb_bytes(),
                self.refreshes(),
                self.appsync_updates(),
            )
        }
    }

    impl From<&mut NcStats> for Statistics {
        fn from(nc: &mut NcStats) -> Statistics {
            Self { nc }
        }
    }
}

/// # constructors & deconstructors
impl Statistics {
    /// Allocates a [`Statistics`] object.
    pub fn new(nc: &mut Notcurses) -> Self {
        let mut stats = Self {
            nc: nc.into_ref_mut().stats_alloc(),
        };
        stats.update(nc);
        stats
    }

    //

    /// Returns a shared reference to the inner [`NcStats`].
    pub fn into_ref(&self) -> &NcStats {
        unsafe { &*self.nc }
    }

    /// Returns an exclusive reference to the inner [`NcStats`].
    pub fn into_ref_mut(&mut self) -> &mut NcStats {
        unsafe { &mut *self.nc }
    }
}

/// # manager methods
impl Statistics {
    /// Acquires an atomic snapshot of the notcurses object's stats.
    pub fn update(&mut self, nc: &mut Notcurses) {
        nc.into_ref_mut().stats(self.into_ref_mut())
    }

    /// Resets all cumulative stats.
    ///
    /// Immediate ones, such as fbbytes, are not reset.
    pub fn reset(&mut self, nc: &mut Notcurses) {
        nc.into_ref_mut().stats_reset(self.into_ref_mut())
    }
}

/// # query methods
impl Statistics {
    /// Successful renders.
    pub fn renders(&self) -> u64 {
        self.into_ref().renders
    }

    /// Failed renders.
    pub fn failed_renders(&self) -> u64 {
        self.into_ref().failed_renders
    }

    /// Nanoseconds spent rendering.
    pub fn render_ns(&self) -> u64 {
        self.into_ref().render_ns
    }

    /// Max ns spent in render for a frame.
    pub fn render_max_ns(&self) -> i64 {
        self.into_ref().render_max_ns
    }

    /// Min ns spent in render for a frame.
    pub fn render_min_ns(&self) -> i64 {
        self.into_ref().render_min_ns
    }

    //

    /// Successful rasterizations.
    pub fn writeouts(&self) -> u64 {
        self.into_ref().writeouts
    }

    /// Failed rasterizations.
    pub fn failed_writeouts(&self) -> u64 {
        self.into_ref().failed_writeouts
    }

    /// Bytes emitted to ttyfp.
    pub fn raster_bytes(&self) -> u64 {
        self.into_ref().raster_bytes
    }

    /// Max bytes emitted for a frame.
    pub fn raster_max_bytes(&self) -> i64 {
        self.into_ref().raster_max_bytes
    }

    /// Min bytes emitted for a frame.
    pub fn raster_min_bytes(&self) -> i64 {
        self.into_ref().raster_min_bytes
    }

    /// Nanoseconds spent rasterizing.
    pub fn raster_ns(&self) -> u64 {
        self.into_ref().raster_ns
    }

    /// Max ns spent in raster for a frame.
    pub fn raster_max_ns(&self) -> i64 {
        self.into_ref().raster_max_ns
    }

    /// Min ns spent in raster for a frame.
    pub fn raster_min_ns(&self) -> i64 {
        self.into_ref().raster_min_ns
    }

    //

    /// Cells we elided entirely thanks to damage maps.
    pub fn cell_elisions(&self) -> u64 {
        self.into_ref().cellelisions
    }

    /// Total number of cells emitted to terminal.
    pub fn cell_emissions(&self) -> u64 {
        self.into_ref().cellemissions
    }

    /// RGB fg elision count.
    pub fn fg_elisions(&self) -> u64 {
        self.into_ref().fgelisions
    }

    /// RGB fg emissions.
    pub fn fg_emissions(&self) -> u64 {
        self.into_ref().fgemissions
    }

    /// RGB bg elision count.
    pub fn bg_elisions(&self) -> u64 {
        self.into_ref().bgelisions
    }

    /// RGB bg emissions.
    pub fn bg_emissions(&self) -> u64 {
        self.into_ref().bgemissions
    }

    /// Default color was emitted.
    pub fn default_elisions(&self) -> u64 {
        self.into_ref().defaultelisions
    }

    /// Default color was elided.
    pub fn default_emissions(&self) -> u64 {
        self.into_ref().defaultemissions
    }

    /// Sprixel draw count.
    pub fn sprixel_emissions(&self) -> u64 {
        self.into_ref().sprixelemissions
    }

    /// Sprixel elision count.
    pub fn sprixel_elisions(&self) -> u64 {
        self.into_ref().sprixelelisions
    }

    /// Sprixel bytes emitted.
    pub fn sprixel_bytes(&self) -> u64 {
        self.into_ref().sprixelbytes
    }

    //

    /// Refresh requests (non-optimized redraw).
    pub fn refreshes(&self) -> u64 {
        self.into_ref().refreshes
    }

    //

    /// How many application-synchronized updates?
    pub fn appsync_updates(&self) -> u64 {
        self.into_ref().appsync_updates
    }

    /// Errors processing control sequences/utf8.
    pub fn input_errors(&self) -> u64 {
        self.into_ref().input_errors
    }

    /// Characters returned to userspace.
    pub fn input_events(&self) -> u64 {
        self.into_ref().input_events
    }

    /// Unnecessary hpas issued.
    ///
    /// The number of hpa (horizontal position absolute, see terminfo(5)) control
    /// sequences issued where not strictly necessary.
    ///
    /// This is done to cope with fundamental ambiguities regarding glyph width.
    /// It is not generally possible to know how wide a glyph will be rendered
    /// on a given combination of font, font rendering engine, and terminal.
    /// Indeed, it is not even generally possible to know how many glyphs will
    /// result from a sequence of EGCs. As a result, Notcurses sometimes issues
    /// "gratuitous" hpa controls.
    pub fn hpa_gratuitous(&self) -> u64 {
        self.into_ref().hpa_gratuitous
    }

    /// Cell geometry changes (resizes).
    ///
    /// The number of changes to the visible area's cell geometry.
    ///
    /// The cell geometry changes whenever the visible area is resized without a
    /// corresponding cell-pixel geometry change.
    ///
    /// Both can change at the same time if e.g. a terminal undergoes a
    /// font size change without changing its total size.
    pub fn cell_geo_changes(&self) -> u64 {
        self.into_ref().cell_geo_changes
    }

    /// Pixel geometry changes (font resize).
    ///
    /// The number of changes to cells' pixel geometry (i.e. the height and
    /// width of each cell), and changes whenever the font size changes.
    pub fn pixel_geo_changes(&self) -> u64 {
        self.into_ref().pixel_geo_changes
    }

    /// Total bytes devoted to all active framebuffers.
    pub fn fb_bytes(&self) -> u64 {
        self.into_ref().fbbytes
    }

    /// Number of planes currently in existence.
    pub fn planes(&self) -> u32 {
        self.into_ref().planes
    }
}
