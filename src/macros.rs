// norcurses::macros
//
//!
//

/// `Plane`.[`putstr`] + `format!`.
///
/// [`putstr`]: crate::Plane#method.putstr
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
    ($plane:expr, $($args:tt)*) => {
        $plane.putstr(&format![$($args)*])
    };
}

/// `Plane`.[`putstrln`] + `format!`.
///
/// [`putstrln`]: crate::Plane#method.putstrln
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
    ($plane:expr) => {
        $plane.putln()
    };
    ($plane:expr, $($args:tt)*) => {
        $plane.putstrln(&format![$($args)*])
    };
}

/// `Plane`.[`putstr`] + `format!` + [`render`].
///
/// [`putstr`]: crate::Plane#method.putstr
/// [`render`]: crate::Plane#method.render
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
    ($plane:expr, $($args:tt)*) => {
        ({
            let res = $plane.putstr(&format![$($args)*])?;
            $plane.render()?;
            Ok(res)
        }) as crate::Result<u32>
    };
}

/// `Plane`.[`putstrln`] + `format!` + [`render`].
///
/// [`putstrln`]: crate::Plane#method.putstrln
/// [`render`]: crate::Plane#method.render
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
    ($plane:expr) => {
        ({
            let res = $plane.putln()?;
            $plane.render()?;
            Ok(res)
        }) as crate::Result<u32>
    };
    ($plane:expr, $($args:tt)*) => {
        ({
            let res = $plane.putstrln(&format![$($args)*])?;
            $plane.render()?;
            Ok(res)
        }) as crate::Result<u32>
    };
}
