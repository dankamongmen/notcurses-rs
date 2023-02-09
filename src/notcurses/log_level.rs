// notcurses::notcurses::log_level
//
//!
//

/// Stderr log level.
///
/// Note that if stderr is connected to the same terminal on which we're
/// rendering, any kind of logging will disrupt the output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogLevel {
    /// Default. print nothing once fullscreen service begins.
    Silent,

    /// Print diagnostics immediately related to crashing.
    Panic,

    /// We're hanging around, but we've had a horrible fault.
    Fatal,

    /// We can't keep doin' this, but we can do other things.
    Error,

    /// You probably don't want what's happening to happen.
    Warning,

    /// "Standard information".
    Info,

    /// "Detailed information".
    Verbose,

    /// This is honestly a bit much.
    Debug,

    /// There's probably a better way to do what you want.
    Trace,
}

mod core_impls {
    use super::LogLevel;
    use crate::sys::{c_api::NcLogLevel_i32, NcLogLevel};
    use core::fmt;

    impl Default for LogLevel {
        fn default() -> Self {
            Self::Silent
        }
    }

    impl fmt::Display for LogLevel {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use LogLevel::*;
            write!(
                f,
                "{}",
                match self {
                    Silent => "Silent",
                    Panic => "Panic",
                    Fatal => "Fatal",
                    Error => "Error",
                    Warning => "Warning",
                    Info => "Info",
                    Verbose => "Verbose",
                    Debug => "Debug",
                    Trace => "Trace",
                }
            )
        }
    }

    //

    impl From<NcLogLevel> for LogLevel {
        fn from(nc: NcLogLevel) -> LogLevel {
            match nc {
                NcLogLevel::Silent => LogLevel::Silent,
                NcLogLevel::Panic => LogLevel::Panic,
                NcLogLevel::Fatal => LogLevel::Fatal,
                NcLogLevel::Error => LogLevel::Error,
                NcLogLevel::Warning => LogLevel::Warning,
                NcLogLevel::Info => LogLevel::Info,
                NcLogLevel::Verbose => LogLevel::Verbose,
                NcLogLevel::Debug => LogLevel::Debug,
                NcLogLevel::Trace => LogLevel::Trace,
            }
        }
    }
    impl From<LogLevel> for NcLogLevel {
        fn from(ll: LogLevel) -> NcLogLevel {
            match ll {
                LogLevel::Silent => NcLogLevel::Silent,
                LogLevel::Panic => NcLogLevel::Panic,
                LogLevel::Fatal => NcLogLevel::Fatal,
                LogLevel::Error => NcLogLevel::Error,
                LogLevel::Warning => NcLogLevel::Warning,
                LogLevel::Info => NcLogLevel::Info,
                LogLevel::Verbose => NcLogLevel::Verbose,
                LogLevel::Debug => NcLogLevel::Debug,
                LogLevel::Trace => NcLogLevel::Trace,
            }
        }
    }

    impl From<NcLogLevel_i32> for LogLevel {
        fn from(nci: NcLogLevel_i32) -> LogLevel {
            NcLogLevel::from(nci).into()
        }
    }
    impl From<LogLevel> for NcLogLevel_i32 {
        fn from(pi: LogLevel) -> NcLogLevel_i32 {
            NcLogLevel::from(pi).into()
        }
    }
}
