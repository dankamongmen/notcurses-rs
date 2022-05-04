// norcurses::macros
//
//!
//

/// Convenience wrapper around [`Plane.putstr`][crate::Plane#method.putstr].
///
/// Calls the `format!` macro.
///
/// # Example
/// ```
/// # use notcurses::*;
/// # fn main() -> Result<()> {
/// let mut nc = Notcurses::new_cli()?;
/// let mut plane = Plane::new(&mut nc)?;
/// plane.set_scrolling(true);
/// assert_eq![12, putstr!(plane, "hello\nworld\n")?];
/// putstr!(plane, "formatted text: {:?}\n", (0, 1.0, "two") )?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! putstr {
    ($plane:ident, $($args:tt)*) => {
        $plane.putstr(&format![$($args)*])
    };
}

/// Convenience wrapper around [`Plane.putstrln`][crate::Plane#method.putstrln].
///
/// Calls the `format!` macro.
///
/// # Example
/// ```
/// # use notcurses::*;
/// # fn main() -> Result<()> {
/// let mut nc = Notcurses::new_cli()?;
/// let mut plane = Plane::new(&mut nc)?;
/// plane.set_scrolling(true);
/// assert_eq![12, putstrln!(plane, "hello world")?];
/// putstr!(plane, "formatted text: {:?}", (0, 1.0, "two") )?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! putstrln {
    ($plane:ident) => {
        $plane.putln()
    };
    ($plane:ident, $($args:tt)*) => {
        $plane.putstrln(&format![$($args)*])
    };
}

/// Convenience wrapper around [`Plane.putstr`][crate::Plane#method.putstr].
///
/// Calls the `format!` macro and [`render`]s the plane afterwards.
///
/// [`render`]: crate::Plane#Render
///
/// # Example
/// ```
/// # use notcurses::*;
/// # fn main() -> Result<()> {
/// let mut nc = Notcurses::new_cli()?;
/// let mut plane = Plane::new(&mut nc)?;
/// plane.set_scrolling(true);
/// assert_eq![12, putstr!(plane, "hello\nworld\n")?];
/// putstr!(plane, "formatted text: {:?}\n", (0, 1.0, "two") )?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! printstr {
    ($plane:ident, $($args:tt)*) => {
        ({
            let res = $plane.putstr(&format![$($args)*])?;
            $plane.render()?;
            Ok(res)
        }) as crate::Result<usize>
    };
}

/// Convenience wrapper around [`Plane.putstrln`][crate::Plane#method.putstrln].
///
/// Calls the `format!` macro and [`render`]s the plane afterwards.
///
/// [`render`]: crate::Plane#Render
///
/// # Example
/// ```
/// # use notcurses::*;
/// # fn main() -> Result<()> {
/// let mut nc = Notcurses::new_cli()?;
/// let mut plane = Plane::new(&mut nc)?;
/// plane.set_scrolling(true);
/// assert_eq![12, putstrln!(plane, "hello world")?];
/// putstr!(plane, "formatted text: {:?}", (0, 1.0, "two") )?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! printstrln {
    ($plane:ident) => {
        ({
            let res = $plane.putln()?;
            $plane.render()?;
            Ok(res)
        }) as crate::Result<usize>
    };
    ($plane:ident, $($args:tt)*) => {
        ({
            let res = $plane.putstrln(&format![$($args)*])?;
            $plane.render()?;
            Ok(res)
        }) as crate::Result<usize>
    };
}
