# notcurses

[![Crate](https://img.shields.io/crates/v/notcurses.svg)](https://crates.io/crates/notcurses)
[![MSRV: 1.58.1](https://flat.badgen.net/badge/MSRV/1.58.1/purple)](https://blog.rust-lang.org/2022/01/20/Rust-1.58.1.html)
<!-- [![API](https://docs.rs/notcurses/badge.svg)](https://docs.rs/notcurses/) -->

A high-level Rust wrapper over [notcurses][0], the most blingful TUI library.

## Status of the library
*Current major version `3` is considered a development version*.

The API is currently undergoing heavy work.

## Main differences with `libnotcurses-sys`:
- All types have the `Drop` trait implemented.
- There is no *direct* mode, use *CLI* mode.
- The *standard* plane is referred to as the *CLI* plane.
- The `*Options` structs are replaced by `*Builder`s.
- Simpler and safer to use.
-->

[0]:https://github.com/dankamongmen/notcurses
