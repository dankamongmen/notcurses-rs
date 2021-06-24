use crate::{
    sys::{self, Nc, NcOptions},
    LogLevel, NResult, Notcurses,
};

/// A [`Notcurses`] builder.
#[derive(Debug)]
pub struct NotcursesBuilder {
    /// top, right, bottom & left margins.
    margins: (u32, u32, u32, u32),
    flags: u64,
    loglevel: LogLevel,
}

impl Default for NotcursesBuilder {
    fn default() -> Self {
        Self {
            margins: (0, 0, 0, 0),
            loglevel: LogLevel::default(),
            flags: sys::NCOPTION_SUPPRESS_BANNERS,
        }
    }
}

impl<'nc> NotcursesBuilder {
    ///   Desirable margins.
    ///
    ///   If all are 0 (default), we will render to the entirety of the screen.
    ///   If the screen is too small, we do what we can.
    //
    //   Absolute coordinates are relative to the rendering area
    //   ((0, 0) is always the origin of the rendering area).
    pub fn margins(mut self, top: u32, right: u32, bottom: u32, left: u32) -> Self {
        self.margins = (top, right, bottom, left);
        self
    }

    /// The [`LogLevel`] to use. Default: `LogLevel::Silent`.
    pub fn loglevel(mut self, loglevel: LogLevel) -> Self {
        self.loglevel = loglevel;
        self
    }

    /// Use alternate screen? Default: true.
    ///
    /// The previoius terminal contents will be restored after notcurses stops.
    pub fn altscreen(mut self, altscreen: bool) -> Self {
        if altscreen {
            self.flags &= !sys::NCOPTION_NO_ALTERNATE_SCREEN;
        } else {
            self.flags |= sys::NCOPTION_NO_ALTERNATE_SCREEN;
        }
        self
    }

    /// Show banners? Default: false.
    ///
    /// Shows version information in initialization and performance information
    /// on shutdown.
    pub fn banners(mut self, banners: bool) -> Self {
        if banners {
            self.flags &= !sys::NCOPTION_SUPPRESS_BANNERS;
        } else {
            self.flags |= sys::NCOPTION_SUPPRESS_BANNERS;
        }
        self
    }

    /// Handle SIGQUIT, SIGINT, SIGSEGV, SIGABRT? Default: true.
    pub fn sigquit(mut self, sighandlers: bool) -> Self {
        if sighandlers {
            self.flags &= !sys::NCOPTION_NO_QUIT_SIGHANDLERS;
        } else {
            self.flags |= sys::NCOPTION_NO_QUIT_SIGHANDLERS;
        }
        self
    }

    /// Handle SIGWINCH? Default: true.
    ///
    /// NCKEY_RESIZE events being generated on input.
    /// With this flag, the handler will not be installed.
    pub fn sigwinch(mut self, sighandlers: bool) -> Self {
        if sighandlers {
            self.flags &= !sys::NCOPTION_NO_WINCH_SIGHANDLER;
        } else {
            self.flags |= sys::NCOPTION_NO_WINCH_SIGHANDLER;
        }
        self
    }

    /// call setlocale()? Default: true.
    ///
    pub fn set_locale(mut self, setlocale: bool) -> Self {
        if setlocale {
            self.flags &= !sys::NCOPTION_INHIBIT_SETLOCALE;
        } else {
            self.flags |= sys::NCOPTION_INHIBIT_SETLOCALE;
        }
        self
    }

    /// try to clear any preexisting bitmaps? Default: true.
    pub fn clear_bitmaps(mut self, clear_bitmaps: bool) -> Self {
        if clear_bitmaps {
            self.flags &= !sys::NCOPTION_NO_CLEAR_BITMAPS;
        } else {
            self.flags |= sys::NCOPTION_NO_CLEAR_BITMAPS;
        }
        self
    }

    /// modify the font? Default: true.
    ///
    /// Notcurses might attempt to change the font slightly, to support certain
    /// glyphs (especially on the Linux console). If this is set, no such
    /// modifications will be made. Note that font changes will not affect
    /// anything but the virtual console/terminal in which notcurses is running.
    pub fn modify_font(mut self, modify_font: bool) -> Self {
        if modify_font {
            self.flags &= !sys::NCOPTION_NO_FONT_CHANGES;
        } else {
            self.flags |= sys::NCOPTION_NO_FONT_CHANGES;
        }
        self
    }

    /// Finishes the build returning the newly configured [`Notcurses`] context.
    pub fn finish(self) -> NResult<Notcurses<'nc>> {
        let ncoptions = NcOptions::with_all_options(
            self.loglevel.into(),
            self.margins.0,
            self.margins.1,
            self.margins.2,
            self.margins.3,
            self.flags,
        );
        let nc = Nc::with_options(ncoptions)?;
        Ok(Notcurses { raw: nc })
    }
}
