[![Crate](https://img.shields.io/crates/v/notcurses.svg)](https://crates.io/crates/notcurses)
[![API](https://docs.rs/notcurses/badge.svg)](https://docs.rs/notcurses/)
[![MSRV: 1.65.0](https://flat.badgen.net/badge/MSRV/1.65.0/purple)](https://releases.rs/docs/1.65.0/)

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

The current version is compatible with notcurses [`3.0.9`][tag].

*Current major version `3` is considered a development version*.

**Main differences with `libnotcurses-sys`:**
- Fully safe public API.
- Allocating types have the `Drop` trait implemented.
- Coordinates are used in the most common order: *x, y*.
- There is no *direct* mode, just use the *CLI* mode.
- The *standard* plane is now known as the *CLI* plane.
- The `*Options` structs are replaced by `*Builder`s.

[tag]: https://github.com/dankamongmen/notcurses/releases/tag/v3.0.9
