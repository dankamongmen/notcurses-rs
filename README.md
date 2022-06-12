# notcurses

[![Crate](https://img.shields.io/crates/v/notcurses.svg)](https://crates.io/crates/notcurses)
[![API](https://docs.rs/notcurses/badge.svg)](https://docs.rs/notcurses/)
[![MSRV: 1.58.1](https://flat.badgen.net/badge/MSRV/1.58.1/purple)](https://blog.rust-lang.org/2022/01/20/Rust-1.58.1.html)
[![Lines Of Code](https://tokei.rs/b1/github/dankamongmen/notcurses-rs?category=code)](https://github.com/dankamongmen/notcurses-rs)

A rusty wrapper over [notcurses][0], the most blingful TUI library.

## Example

```rust
use notcurses::*;

fn main() -> Result<()> {
    let mut nc = Notcurses::new_cli()?;
    let mut cli = nc.cli_plane()?;
    cli.putstrln("hello world!")?;
    cli.render()?;
    Ok(())
}
```

## Status of the library
*Current major version `3` is considered a development version*.

The API is currently undergoing heavy work.

**Main differences with `libnotcurses-sys`:**
- Coordinates are used in the most common order: *x, y*.
- Allocating types have the `Drop` trait implemented.
- There is no *direct* mode, just use the *CLI* mode.
- The *standard* plane is now known as the *CLI* plane.
- The `*Options` structs are replaced by `*Builder`s.
- Fully safe public API.

[0]:https://github.com/dankamongmen/notcurses
