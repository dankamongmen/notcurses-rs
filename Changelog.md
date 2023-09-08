# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

## [3.5.0] - 2023-09-08

### Added
- new feature `nightly_docs`.

### Changed
- bump `libnotcurses-sys` to `v3.10.0`.
- bump MSRV to `1.65.0`.

### Fixed
- avoid destructuring enums.
- update CI.

## [3.4.1] - 2023-04-23

### Changed
- update libnotcurses-sys to `v3.9.1`.

### Fixes
- fix compilation on MacOs.

## [3.4.0] - 2023-04-09

### Added
- add `cuadra` dependency to use their `Position` and `Size` replacing the old types.

### Changed
- update `Blitter` methods related to cell size to use the expected order of width first.

### Removed
- remove `az` dependency.

## [3.3.0] - 2023-04-08

### Added
- add missing `Plane` methods: `erase`, `erase_region`, `contents`, `contents_region`.
- add multiple missing inline attributes.

### Changed
- update libnotcurses to `3.9.0`.
- make `VisualOptions` public.
- rename `Error` and `Result` to `NotcursesError` and `NotcursesResult`, respectively (**breaking change**).

### Fixed
- fix `Plane`'s `set_base_*` methods.
- update impl `Debug` for `Style`.

## [3.2.4] - 2023-03-22

### Fixed
- update CIs.
- update docs.
- update lints.
- minor refactor.

## [3.2.3] - 2023-03-22

### Changed
- update `libnotcurses-sys` to `v3.7.5`.

### Fixed
- fix `Nc`'s `osversion` method.

## [3.2.2] - 2023-02-10

### Changed
- update libnotcurses-sys to `v3.7.4`.
- remove `vendored` as a default feature.

## [3.2.1] - 2023-02-09

### Added
- impl from `KeyMod` for `u32`.
- add `rgb` interoperability dependency.

### Changed
- update libnotcurses sys to `v3.7.2`.
- depend on `core` when possible instead of `std`.
- make `vendored` a default feature.

## [3.2.0] - 2023-01-19

- update dependencies.
  - update libnotcurses-sys to `v3.7.1`.
- refactor methods for `Input:`
  - rename `is_received` to `received`.
  - `has_key`, `has_char` for `some_key`, `some_char`.
  - add `is_press`, `is_release`, `is_repeat`.
- new methods for `Received`: `is_key`, `is_char`, `key`, `char`.
- new methods for `InputType`: `is_press`, `is_repeat`, `is_release`.
- add missing inlines.

## [3.1.0] - 2022-09-26

- rename methods `Visual::set_pixel` to `set_blitter_pixel` and `VisualBuilder::pixel` to `blitter_pixel`.
- add `Visual` methods `set_pixel` & `get_pixel`.
- update `Plane`:
  - fix method `duplicate` to accept a shared self reference.
  - impl `Clone`.
  - fix `Plane::resize_simple` order of dimensions.
  - accept impl `Into<Size|Position>` for `resize` && `resize_simple`.
  - add methods: `styles`, `on_styles`, `off_styles`, `set_styles`, `set_fg_palindex`, `set_bg_palindex`.
  - update `Plane` methods: `set_base_bg`, `set_base_fg` & `set_base_channels`, now accepting `impl Into<Channel*>` instead of `Channel`.
- fix double free when dropping order is not ideal.
- add `new` `const` constructor for `Rgb` & `Rgba`.
- add conversions for `Rgb` and `Rgba`.
- update `Style`:
  - fix method `to_vec` to only include `None` if there are no other styles present.
  - improve `Display` & `Debug` impls.
  - fix `From`/`Into` `NcStyle` impls.
  - fix supported styles (WIP→ sys?)
  - rename method `add` to `set`.
  - add new method `unset`.
  - improve example `info`.
- Update `Channels`:
  - new constructors: `with_default`, `from_rgb`, `from_rgb_both`, `from_rgb_alpha`, `from_rgb_alpha_both`, `combine`.
  - new methods: `reverse`, `set_fg`, `set_bg`, `set_channels`.
  - new `From` impls from tuples and arrays of primitives, representing different bg & fg channels: `(r,g,b,r,g,b)`, `((r,g,b),(r,g,b))`, `[r,g,b,r,g,b]`, `[r,g,b,r,g,b]`, `[[r,g,b],[r,g,b]]`, same bg & fg channels: `(r,g,b)`, `[r,g,b]`, and a combination of bg & fg impl Into<Channel> `(fg, bg)`, `[fg, bg]`.
- impl `Display` for `Capabilities`.

## [3.0.3] - 2022-07-25

### Added
- update `Channels`:
  - new constructors: `with_default`, `from_rgb`, `from_rgb_both`, `from_rgb_alpha`, `from_rgb_alpha_both`, `combine`.
  - new methods: `reverse`, `set_fg`, `set_bg`, `set_channels`.
  - new `From` impls from tuples and arrays of primitives, representing different bg & fg channels: `(r,g,b,r,g,b)`, `((r,g,b),(r,g,b))`, `[r,g,b,r,g,b]`, `[r,g,b,r,g,b]`, `[[r,g,b],[r,g,b]]`, same bg & fg channels: `(r,g,b)`, `[r,g,b]`, and a combination of bg & fg impl Into<Channel> `(fg, bg)`, `[fg, bg]`.

### Changed
- update `Plane` methods: `set_base_bg`, `set_base_fg` & `set_base_channels`, now accepting `impl Into<Channel*>` instead of `Channel`.
- impl `Display` for `Capabilities`.
- bump libnotcurses-sys to `v3.6.2`.

### Fixed
- fix clippy lints.
- minor refactors.

## [3.0.2] - 2022-06-17

### Fixed
- fix compilation on apple M1.

## [3.0.1] - 2022-06-12

### Fixed
- fix docs.rs build.

## [3.0.0] - 2022-06-12

Too many changes.

## [2.0.0] - 2021-04-20

A clean slate.


[unreleased]: https://github.com/dankamongmen/notcurses-rs/compare/v3.5.0...HEAD
[3.5.0]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.5.0
[3.4.1]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.4.1
[3.4.0]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.4.0
[3.3.0]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.3.0
[3.2.3]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.2.3
[3.2.2]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.2.2
[3.2.1]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.2.1
[3.2.0]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.2.0
[3.1.0]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.1.0
[3.0.3]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.0.3
[3.0.2]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.0.2
[3.0.1]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v3.0.1
[2.0.0]: https://github.com/dankamongmen/notcurses-rs/releases/tag/v2.0.0

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
