use crate::sys::NcLogLevel;

/// A `i32` of logging levels for [`Notcurses`][crate::Notcurses].
///
/// These log levels consciously map cleanly to those of libav; notcurses itself
/// does not use this full granularity. The log level does not affect the opening
/// and closing banners, which can be disabled via the `NcOptions`
/// `NCOPTION_SUPPRESS_BANNERS`.
///
/// Note that if stderr is connected to the same terminal on which we're
/// which we're rendering, any kind of logging will disrupt the output (which is
/// undesirable). The "default" value is `NcLogLevel::PANIC`.
#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum LogLevel {
    /// Default. print nothing once fullscreen service begins.
    Silent = NcLogLevel::Silent as i32,

    /// Print diagnostics immediately related to crashing.
    Panic = NcLogLevel::Panic as i32,

    /// We're hanging around, but we've had a horrible fault.
    Fatal = NcLogLevel::Fatal as i32,

    /// We can't keep doing this, but we can do other things.
    Error = NcLogLevel::Error as i32,

    /// You probably don't want what's happening to happen.
    Warning = NcLogLevel::Warning as i32,

    /// "Standard information".
    Info = NcLogLevel::Info as i32,

    /// "Detailed information".
    Verbose = NcLogLevel::Verbose as i32,

    /// This is honestly a bit much.
    Debug = NcLogLevel::Debug as i32,

    /// There's probably a better way to do what you want.
    Trace = NcLogLevel::Trace as i32,
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
