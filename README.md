# notcurses

[![Crate](https://img.shields.io/crates/v/notcurses.svg)](https://crates.io/crates/notcurses)
<!-- [![API](https://docs.rs/notcurses/badge.svg)](https://docs.rs/notcurses/) -->
[![MSRV: 1.58.1](https://flat.badgen.net/badge/MSRV/1.58.1/purple)](https://blog.rust-lang.org/2022/01/20/Rust-1.58.1.html)

A high-level Rust wrapper over [libnotcurses-sys][0] and the [notcurses C library][1].


## Status of the library
*Current major version `3` is considered a development version*.

It is not yet in a usable state.

<!--
Main API differences with `libnotcurses-sys`:
- All types have the `Drop` trait implemented.
- All coordinate pairs (`X`,`Y`), (`cols`,`rows`) are in alphabetic order.
-->

<!--
- Instead of using option structures, you now use the builder pattern
  to construct `Plane` and `Visual` objects.
- The concept of the standard plane disappears, you just use `Plane`s.
-->

[0]:https://github.com/dankamongmen/notcurses
[1]:https://github.com/dankamongmen/libnotcurses-sys
