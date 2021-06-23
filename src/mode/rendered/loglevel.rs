use crate::sys::{self, NcLogLevel};

/// A `u32` of logging levels for [`Notcurses`][crate::Notcurses].
///
/// These log levels consciously map cleanly to those of libav; notcurses itself
/// does not use this full granularity. The log level does not affect the opening
/// and closing banners, which can be disabled via the `NcOptions`
/// `NCOPTION_SUPPRESS_BANNERS`.
/// Note that if stderr is connected to the same terminal on which we're
/// rendering, any kind of logging will disrupt the output.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum LogLevel {
    /// Default. print nothing once fullscreen service begins.
    Silent = sys::NCLOGLEVEL_SILENT,

    /// Print diagnostics immediately related to crashing.
    Panic = sys::NCLOGLEVEL_PANIC,

    /// We're hanging around, but we've had a horrible fault.
    Fatal = sys::NCLOGLEVEL_FATAL,

    /// We can't keep doing this, but we can do other things.
    Error = sys::NCLOGLEVEL_ERROR,

    /// You probably don't want what's happening to happen.
    Warning = sys::NCLOGLEVEL_WARNING,

    /// "Standard information".
    Info = sys::NCLOGLEVEL_INFO,

    /// "Detailed information".
    Verbose = sys::NCLOGLEVEL_VERBOSE,

    /// This is honestly a bit much.
    Debug = sys::NCLOGLEVEL_DEBUG,

    /// There's probably a better way to do what you want.
    Trace = sys::NCLOGLEVEL_TRACE,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Silent
    }
}

impl From<LogLevel> for NcLogLevel {
    fn from(log_level: LogLevel) -> NcLogLevel {
        log_level as NcLogLevel
    }
}
