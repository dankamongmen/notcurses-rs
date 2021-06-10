pub use crate::sys::sleep as s;

/// Translates a libnotcurses_sys::NcError to a notcurses::Error.
#[macro_export]
macro_rules! ncresult {
    ($res:expr) => {{
        let res = $res;
        match res {
            Ok(t) => {
                return Ok(t);
            }
            Err(e) => {
                return Err(crate::Error::NcError {
                    int: e.int,
                    msg: e.msg,
                });
            }
        }
    }};
}

/// Renders and rasterizes the pile of the Plane `$p` and then sleeps.
///
/// [`Plane.render`][crate::Plane#method.render]\(`$p`\)? plus
/// [`Plane.raster`][crate::Plane#method.raster]\(`$p`\)? plus
/// [`s!`]`[$sleep_args]`.
///
/// Returns [Result].
#[macro_export]
macro_rules! rs {
    ($p:expr, $( $sleep_args:expr),+ ) => {
        crate::Plane::render_raster($p)?;
        s![$( $sleep_args ),+];
    };
    ($nc:expr, $( $sleep_args:expr),+ ,) => {
        rs![$nc, $( $sleep_args ),* ]
    };
}
