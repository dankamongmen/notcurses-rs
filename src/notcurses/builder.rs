// notcurses::notcurses::builder
//
//!
//

use crate::{
    error::Result,
    notcurses::{LogLevel, Notcurses},
    sys::{Nc, NcOptionsBuilder},
};

/// A [`Notcurses`] builder.
#[derive(Clone, Copy, Debug)]
pub struct NotcursesBuilder {
    options: NcOptionsBuilder,
}

mod core_impls {
    use super::{NcOptionsBuilder, NotcursesBuilder};

    impl Default for NotcursesBuilder {
        fn default() -> Self {
            Self {
                options: NcOptionsBuilder::new().suppress_banners(true),
            }
        }
    }
}

/// # constructors
impl NotcursesBuilder {
    /// Returns a new default `NotcursesBuilder`, without banners.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a `Notcurses` instance.
    pub fn build(self) -> Result<Notcurses> {
        Notcurses::lock_notcurses()?;
        let nc = unsafe { Nc::with_options(self.options.build())? };
        Ok(Notcurses {
            nc,
            options: self.options,
        })
    }
}

/// # methods (chainable)
impl NotcursesBuilder {
    /// Sets the log level.
    pub fn log_level(mut self, log_level: LogLevel) -> Self {
        self.options.set_log_level(log_level.into());
        self
    }

    /// Sets the margins.
    pub fn margins(mut self, top: u32, right: u32, bottom: u32, left: u32) -> Self {
        self.options.set_margins(top, right, bottom, left);
        self
    }

    /// Sets the top margin.
    pub fn margin_top(mut self, top: u32) -> Self {
        self.options.set_margin_top(top);
        self
    }

    /// Sets the right margin.
    pub fn margin_right(mut self, right: u32) -> Self {
        self.options.set_margin_right(right);
        self
    }

    /// Sets the bottom margin.
    pub fn margin_bottom(mut self, bottom: u32) -> Self {
        self.options.set_margin_bottom(bottom);
        self
    }

    /// Sets the left margin.
    pub fn margin_left(mut self, left: u32) -> Self {
        self.options.set_margin_left(left);
        self
    }

    // flags

    /// If `true`, Input may be freely dropped.
    ///
    /// This ought be provided when the program does not intend to handle input.
    /// Otherwise, input can accumulate in internal buffers, eventually preventing
    /// Notcurses from processing terminal messages.
    pub fn drain_input(mut self, drain: bool) -> Self {
        self.options.set_drain_input(drain);
        self
    }

    /// If `true`, wont call setlocale().
    pub fn inhibit_set_locale(mut self, inhibit: bool) -> Self {
        self.options.set_inhibit_set_locale(inhibit);
        self
    }

    /// If `true`, wont enter alternate mode.
    pub fn no_alternate_screen(mut self, no_alternate: bool) -> Self {
        self.options.set_no_alternate_screen(no_alternate);
        self
    }

    /// If `true`, wont try to clear any preexisting bitmaps.
    pub fn no_clear_bitmaps(mut self, no_clear: bool) -> Self {
        self.options.set_no_clear_bitmaps(no_clear);
        self
    }

    /// If `true`, wont modify the font.
    pub fn no_font_changes(mut self, no_font_changes: bool) -> Self {
        self.options.set_no_font_changes(no_font_changes);
        self
    }

    /// If `true`, wont handle `SIGINT`, `SIGSEGV`, `SIGABRT` nor `SIGQUIT`.
    pub fn no_quit_sig_handlers(mut self, no_quit: bool) -> Self {
        self.options.set_no_quit_sig_handlers(no_quit);
        self
    }

    /// If `true`, wont handle `SIGWINCH`.
    pub fn no_winch_sig_handler(mut self, no_winch: bool) -> Self {
        self.options.set_no_winch_sig_handler(no_winch);
        self
    }

    /// If `true`, will initializes the CLI plane’s virtual cursor to match
    /// the physical cursor at context creation time.
    pub fn preserve_cursor(mut self, preserve: bool) -> Self {
        self.options.set_preserve_cursor(preserve);
        self
    }

    /// If `true`, will prepare the CLI plane in scrolling mode.
    pub fn scrolling(mut self, scrolling: bool) -> Self {
        self.options.set_scrolling(scrolling);
        self
    }

    /// A shortcut for setting the following options together:
    /// `no_alternate_screen`, `no_clear_bitmaps`, `preserve_cursor` & `scrolling`.
    pub fn cli_mode(mut self, cli_mode: bool) -> Self {
        self.options.set_cli_mode(cli_mode);
        self
    }

    /// If `true`, wont print banners.
    pub fn suppress_banners(mut self, suppress_banners: bool) -> Self {
        self.options.set_suppress_banners(suppress_banners);
        self
    }
}
