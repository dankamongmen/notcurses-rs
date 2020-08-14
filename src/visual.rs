// total functions: 16
// ------------------------------------------ (done / wont / remaining)
// - implemented: 9 / … / 16
// - +unit tests: 0 / … / 16
// ------------------------- ↓ from bindgen: 16
// ncvisual_at_yx
// ncvisual_decode
// ncvisual_destroy
// ncvisual_from_bgra
// ncvisual_from_file
// ncvisual_from_plane
// ncvisual_from_rgba
// ncvisual_geom
// ncvisual_polyfill_yx
// ncvisual_render
// ncvisual_resize
// ncvisual_rotate
// ncvisual_set_yx
// ncvisual_simple_streamer
// ncvisual_stream
// ncvisual_subtitle

use libnotcurses_sys as nc;

#[repr(u32)] // = ncscale_e
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Scale {
    None = nc::ncscale_e_NCSCALE_NONE as nc::ncscale_e,
    Scale = nc::ncscale_e_NCSCALE_SCALE as nc::ncscale_e,
    Stretch = nc::ncscale_e_NCSCALE_STRETCH as nc::ncscale_e,
}

#[repr(u32)] // = ncalign_e
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Align {
    Left = nc::ncalign_e_NCALIGN_LEFT as nc::ncalign_e,
    Center = nc::ncalign_e_NCALIGN_CENTER as nc::ncalign_e,
    Right = nc::ncalign_e_NCALIGN_RIGHT as nc::ncalign_e,
}

// each has the empty cell in addition to the product of its dimensions. i.e.
// NCBLIT_1x1 has two states: empty and full block. NCBLIT_1x1x4 has five
// states: empty, the three shaded blocks, and the full block.

///
/// NOTE: Blitter::_1x1x4 & Blitter::_4x1 are still unimplemented,
/// they both ought be falling back to 1x1 with a top half.
#[repr(u32)] // = ncblitter_e
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Blitter {
    /// full block                █
    _1x1 = nc::ncblitter_e_NCBLIT_1x1 as nc::ncblitter_e,

    /// shaded full blocks        ▓▒░█
    _1x1x4 = nc::ncblitter_e_NCBLIT_1x1x4 as nc::ncblitter_e,

    /// upper half + 1x1          ▀█
    _2x1 = nc::ncblitter_e_NCBLIT_2x1 as nc::ncblitter_e,

    /// quadrants + 2x1           ▗▐ ▖▀▟▌▙█
    _2x2 = nc::ncblitter_e_NCBLIT_2x2 as nc::ncblitter_e,

    /// four vertical levels      █▆▄▂
    _4x1 = nc::ncblitter_e_NCBLIT_4x1 as nc::ncblitter_e,

    /// eight vertical levels     █▇▆▅▄▃▂▁
    _8x1 = nc::ncblitter_e_NCBLIT_8x1 as nc::ncblitter_e,

    /// 4 rows, 2 cols (braille)  ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    Braille = nc::ncblitter_e_NCBLIT_BRAILLE as nc::ncblitter_e,

    /// let the ncvisual pick
    Default = nc::ncblitter_e_NCBLIT_DEFAULT as nc::ncblitter_e,

    /// 6 rows, 1 col (RGB), spotty support among terminals
    Sixel = nc::ncblitter_e_NCBLIT_SIXEL as nc::ncblitter_e,
}

pub struct VisualOptions {}

// ncvisual_at_yx⚠
// ncvisual_decode⚠
// ncvisual_destroy⚠
// ncvisual_from_bgra⚠
// ncvisual_from_file⚠
// ncvisual_from_plane⚠
// ncvisual_from_rgba⚠
// ncvisual_geom⚠
// ncvisual_polyfill_yx⚠
// ncvisual_render⚠
// ncvisual_resize⚠
// ncvisual_rotate⚠
// ncvisual_set_yx⚠
// ncvisual_simple_streamer⚠
// ncvisual_stream⚠
// ncvisual_subtitle⚠

pub struct Visual {}

impl Visual {
    // pub fn ncvisual_at_yx() {}

    // pub fn ncvisual_decode() {}

    // pub fn ncvisual_destroy() {}

    // pub fn ncvisual_from_bgra() {}

    // pub fn ncvisual_from_file() {}

    // pub fn ncvisual_from_plane() {}

    // pub fn ncvisual_from_rgba() {}

    // pub fn ncvisual_geom() {}

    // pub fn ncvisual_polyfill_yx() {}

    // pub fn ncvisual_render() {}

    // pub fn ncvisual_resize() {}

    // pub fn ncvisual_rotate() {}

    // pub fn ncvisual_set_yx() {}

    // pub fn ncvisual_simple_streamer() {}

    // pub fn ncvisual_stream() {}

    // pub fn ncvisual_subtitle() {}
}
