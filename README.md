[![Crate](https://img.shields.io/crates/v/notcurses.svg)](https://crates.io/crates/notcurses)
[![API](https://docs.rs/notcurses/badge.svg)](https://docs.rs/notcurses/)
[![MSRV: 1.58.1](https://flat.badgen.net/badge/MSRV/1.58.1/purple)](https://blog.rust-lang.org/2022/01/20/Rust-1.58.1.html)
[![Lines Of Code](https://tokei.rs/b1/github/dankamongmen/notcurses-rs?category=code)](https://github.com/dankamongmen/notcurses-rs)

A rusty wrapper over [notcurses][0], the most blingful TUI library.

[0]:https://github.com/dankamongmen/notcurses

## Example

```rust
use notcurses::*;

fn main() -> Result<()> {
    let mut nc = Notcurses::new_cli()?;
    let mut cli = nc.cli_plane()?;
    cli.putstrln("\nhello world!")?;
    cli.render()?;
    Ok(())
}
```

## Status of the library
Current version `3.0.2` is compatible with notcurses `3.0.8`.

*Current major version `3` is considered a development version*.

**Main differences with `libnotcurses-sys`:**
- Fully safe public API.
- Allocating types have the `Drop` trait implemented.
- Coordinates are used in the most common order: *x, y*.
- There is no *direct* mode, just use the *CLI* mode.
- The *standard* plane is now known as the *CLI* plane.
- The `*Options` structs are replaced by `*Builder`s.
