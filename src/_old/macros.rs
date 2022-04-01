pub use crate::sys::sleep;

/// Translates a `libnotcurses_sys::NcResult` to a `notcurses::NResult`.
#[macro_export]
macro_rules! ncresult {
    ($res:expr) => {{
        let res = $res;
        match res {
            Ok(t) => {
                return Ok(t);
            }
            Err(e) => {
                return Err(crate::NError::NcError {
                    int: e.int,
                    msg: e.msg,
                });
            }
        }
    }};
}

// MAYBE this is not necessary
//
// /// Renders and rasterizes the pile of the Plane `$p` and then sleeps.
// ///
// /// [`Plane.render`][crate::Plane#method.render]\(`$p`\)? plus
// /// [`Plane.raster`][crate::Plane#method.raster]\(`$p`\)? plus
// /// [`sleep!`]`[$sleep_args]`.
// ///
// /// Returns [`NotcursesResult`].
// #[macro_export]
// macro_rules! rs {
//     ($p:expr, $( $sleep_args:expr),+ ) => {
//         crate::Plane::render_raster($p)?;
//         sleep![$( $sleep_args ),+];
//     };
//     ($nc:expr, $( $sleep_args:expr),+ ,) => {
//         rs![$nc, $( $sleep_args ),* ]
//     };
// }
