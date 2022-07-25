// norcurses::macros
//
//!
//

/// Prints to a plane, similarly as [`print!`].
///
/// `Plane`.[`putstr`] using the [`format!`] syntax.
///
/// [`putstr`]: crate::plane::Plane#method.putstr
///
/// Optionally renders with `+render` as first argument.
///
/// # Example
/// ```
/// # use notcurses::*;
/// # fn main() -> Result<()> {
/// # let mut nc = Notcurses::new_cli()?;
/// # let mut plane = Plane::new(&mut nc)?;
/// # plane.set_scrolling(true);
/// assert_eq![12, putstr!(plane, "hello\nworld\n")?];
/// putstr!(plane, "formatted text: {:?}\n", (0, 1.0, "two") )?;
/// putstr!(+render plane, "renders afterwards = {}", true)?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! putstr {
    ($plane:expr, $($args:tt)*) => {
        ({
            let res = $plane.putstr(&format![$($args)*])?;
            Ok(res)
        }) as $crate::Result<u32>
    };
    (+render $plane:expr, $($args:tt)*) => {
        ({
            let res = $plane.putstr(&format![$($args)*])?;
            $plane.render()?;
            Ok(res)
        }) as $crate::Result<u32>
    };

}

/// Prints to a plane, with a new line, similarly as [`println!`].
///
/// `Plane`.[`putstrln`] using the [`format!`] syntax.
///
/// [`putstrln`]: crate::plane::Plane#method.putstrln
///
/// Optionally renders with `+render` as first argument.
///
/// # Example
/// ```
/// # use notcurses::*;
/// # fn main() -> Result<()> {
/// # let mut nc = Notcurses::new_cli()?;
/// # let mut plane = Plane::new(&mut nc)?;
/// # plane.set_scrolling(true);
/// assert_eq![12, putstrln!(plane, "hello world")?];
/// putstrln!(plane, "formatted text: {:?}", (0, 1.0, "two") )?;
/// putstrln!(+render plane)?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! putstrln {
    ($plane:expr) => {
        ({
            let res = $plane.putln()?;
            Ok(res)
        }) as $crate::Result<u32>
    };
    ($plane:expr, $($args:tt)*) => {
        ({
            let res = $plane.putstrln(&format![$($args)*])?;
            Ok(res)
        }) as $crate::Result<u32>
    };
    (+render $plane:expr) => {
        ({
            let res = $plane.putln()?;
            $plane.render()?;
            Ok(res)
        }) as $crate::Result<u32>
    };
    (+render $plane:expr, $($args:tt)*) => {
        ({
            let res = $plane.putstrln(&format![$($args)*])?;
            $plane.render()?;
            Ok(res)
        }) as $crate::Result<u32>
    };
}
