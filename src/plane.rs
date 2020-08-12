use libnotcurses_sys as nc;
use std::ptr::{null, null_mut};

use crate::notcurses::NotCurses;
use crate::error::NcError;

/// Alignment within a plane or terminal. Left/right-justified, or centered.
///
/// [C sourcecode](https://nick-black.com/notcurses/html/notcurses_8h_source.html#l00063)
#[repr(u32)] // = nc::ncalign_e
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NcAlign {
    Left = nc::ncalign_e_NCALIGN_LEFT as nc::ncalign_e,
    Center = nc::ncalign_e_NCALIGN_CENTER as nc::ncalign_e,
    Right = nc::ncalign_e_NCALIGN_RIGHT as nc::ncalign_e,
}

/// NCPLANE
///
/// New planes can be created with `new()`, `bound()`, and `aligned()`.
/// dup() duplicates a plane.
///
// NOTE: the previous 4 functions internally can fail and return null
// https://nick-black.com/notcurses/notcurses_plane.3.html#return-values
///
// SACAR:
//
// - Following initialization, a single ncplane exists, the “standard plane”. This plane cannot be destroyed
//   nor manually resized, and is always exactly as large as the screen.
// - Further `NcPlane`s can be created with `NcPlane::new()`. A total z-ordering always exists on the set of
//   `NcPlane`s, and new `NcPlane`s are placed at the top of the z-buffer.
// - `NcPlane`s can be larger, smaller, or the same size as the physical screen, and can be placed anywhere
//   relative to it (including entirely off-screen).
// - TODO: `NcPlane`s are made up of `NcCell`s cells.
// - TODO: All user-created planes can be destroyed in one call with notcurses_drop_planes()
// - It is generally more performant to “hide” planes at the bottom of the stack, ideally behind a large
//   opaque plane, rather than moving them beyond the boundaries of the visible window.
// - Planes ought be no larger than necessary, so that they intersect with the minimum number of cells.
//
// Ncplanes are the fundamental drawing object of notcurses.
//
// TODO: (METHODS?) All output functions take a struct `NcPlane` as an argument.
//
// They can be any size, and placed anywhere. In addition to its framebuffer an NcPlane is defined by:
//  - a base cell, used for any cell on the plane without a glyph
//  - the egcpool backing its cells
//  - a current cursor location
//  - a current style, foreground channel, and background channel
//  - its geometry
//  - a configured user pointer,
//  - its position relative to the visible plane
//  - its z-index
///
/// ## Links
/// - [man notcurses_stdplane](https://nick-black.com/notcurses/notcurses_stdplane.3.html)
/// - [doxygen ncplane struct reference](https://nick-black.com/notcurses/html/structncplane.html)
///
pub struct NcPlane {
    data: *mut nc::ncplane,
}

// TODO: implement remaining methods:
//
// + = implemented
// » = implemented and unit tested
//
//+ncplane_aligned⚠
// ncplane_at_cursor⚠
// ncplane_at_yx⚠
// ncplane_attr⚠
// ncplane_base⚠
// ncplane_below⚠
//+ncplane_bound⚠
// ncplane_box⚠
// ncplane_center_abs⚠
// ncplane_channels⚠
// ncplane_contents⚠
// ncplane_cursor_move_yx⚠
// ncplane_cursor_yx⚠
// ncplane_destroy⚠
//»ncplane_dim_x
//»ncplane_dim_y
//»ncplane_dim_yx⚠
// ncplane_dup⚠
//+ncplane_erase⚠
// ncplane_fadein⚠
// ncplane_fadein_iteration⚠
// ncplane_fadeout⚠
// ncplane_fadeout_iteration⚠
// ncplane_format⚠
// ncplane_gradient⚠
// ncplane_greyscale⚠
// ncplane_highgradient⚠
// ncplane_hline_interp⚠
// ncplane_home⚠
// ncplane_mergedown⚠
// ncplane_move_above⚠
// ncplane_move_below⚠
// ncplane_move_bottom⚠
// ncplane_move_top⚠
// ncplane_move_yx⚠
//+ncplane_new⚠
// ncplane_notcurses⚠
// ncplane_notcurses_const⚠
// ncplane_perimeter
// ncplane_polyfill_yx⚠
// ncplane_pulse⚠
// ncplane_putc_yx⚠
// ncplane_putegc_stainable⚠
// ncplane_putegc_yx⚠
// ncplane_putnstr_aligned⚠
// ncplane_putnstr_yx⚠
// ncplane_putsimple_stainable⚠
// ncplane_putstr
// ncplane_putstr_aligned⚠
// ncplane_putstr_stainable⚠
// ncplane_putstr_yx⚠
// ncplane_puttext⚠
// ncplane_putwegc_stainable⚠
// ncplane_qrcode⚠
//+ncplane_reparent⚠
//+ncplane_resize⚠
// ncplane_rgba⚠
// ncplane_rotate_ccw⚠
// ncplane_rotate_cw⚠
// ncplane_set_attr⚠
// ncplane_set_base⚠
// ncplane_set_base_cell⚠
// ncplane_set_bg⚠
// ncplane_set_bg_alpha⚠
// ncplane_set_bg_default⚠
// ncplane_set_bg_palindex⚠
// ncplane_set_bg_rgb⚠
// ncplane_set_bg_rgb_clipped⚠
// ncplane_set_channels⚠
// ncplane_set_fg⚠
// ncplane_set_fg_alpha⚠
// ncplane_set_fg_default⚠
// ncplane_set_fg_palindex⚠
// ncplane_set_fg_rgb⚠
// ncplane_set_fg_rgb_clipped⚠
// ncplane_set_scrolling⚠
// ncplane_set_userptr⚠
// ncplane_stain⚠
// ncplane_styles⚠
// ncplane_styles_off⚠
// ncplane_styles_on⚠
// ncplane_styles_set⚠
// ncplane_translate⚠
// ncplane_translate_abs⚠
// ncplane_userptr⚠
// ncplane_vline_interp⚠
// ncplane_vprintf_aligned⚠
// ncplane_vprintf_stainable⚠
// ncplane_vprintf_yx⚠
//
//
// REFERENCE OF THE ORIGINAL FUNCTIONS:
// -------------------------------------
// Functions returning int return 0 on success, and non-zero on error.
// All other functions cannot fail (and return void)
// -------------------------------------
//
//+struct ncplane* ncplane_new(struct notcurses* nc, int rows, int cols, int yoff, int xoff, void* opaque);
//+struct ncplane* ncplane_bound(struct ncplane* n, int rows, int cols, int yoff, int xoff, void* opaque);
// struct ncplane* ncplane_reparent(struct ncplane* n, struct ncplane* newparent);
// struct ncplane* ncplane_aligned(struct ncplane* n, int rows, int cols, int yoff, ncalign_e align, void* opaque);
//+struct ncplane* ncplane_dup(struct ncplane* n, void* opaque);
// int ncplane_resize(struct ncplane* n, int keepy, int keepx, int keepleny, int keeplenx, int yoff, int xoff, int ylen, int xlen);
// int ncplane_move_yx(struct ncplane* n, int y, int x);
// void ncplane_yx(const struct ncplane* n, int* restrict y, int* restrict x);
// int ncplane_set_base_cell(struct ncplane* ncp, const cell* c);
// int ncplane_set_base(struct ncplane* ncp, const char* egc, uint32_t attrword, uint64_t channels);
// int ncplane_base(struct ncplane* ncp, cell* c);
// int ncplane_move_top(struct ncplane* n);
// int ncplane_move_bottom(struct ncplane* n);
// int ncplane_move_above(struct ncplane* restrict n, struct ncplane* restrict above);
// int ncplane_move_below(struct ncplane* restrict n, struct ncplane* restrict below);
// struct ncplane* ncplane_below(struct ncplane* n);
// char* ncplane_at_cursor(struct ncplane* n, uint32_t* attrword, uint64_t* channels);
// int ncplane_at_cursor_cell(struct ncplane* n, cell* c);
// char* ncplane_at_yx(const struct ncplane* n, int y, int x, uint32_t* attrword, uint64_t* channels);
// int ncplane_at_yx_cell(struct ncplane* n, int y, int x, cell* c);
// uint32_t* ncplane_rgba(const struct ncplane* nc, int begy, int begx, int leny, int lenx);
// char* ncplane_contents(const struct ncplane* nc, int begy, int begx, int leny, int lenx);
// void* ncplane_set_userptr(struct ncplane* n, void* opaque);
// void* ncplane_userptr(struct ncplane* n);
//»void ncplane_dim_yx(const struct ncplane* n, int* restrict rows, int* restrict cols);
//»static inline int ncplane_dim_y(const struct ncplane* n);
//»static inline int ncplane_dim_x(const struct ncplane* n);
// void ncplane_cursor_yx(const struct ncplane* n, int* restrict y, int* restrict x);
// void ncplane_translate(const struct ncplane* src, const struct ncplane* dst, int* restrict y, int* restrict x);
// bool ncplane_translate_abs(const struct ncplane* n, int* restrict y, int* restrict x);
// uint64_t ncplane_channels(const struct ncplane* n);
// uint32_t ncplane_attr(const struct ncplane* n);
// void ncplane_set_channels(struct ncplane* nc, uint64_t channels);
// void ncplane_set_attr(struct ncplane* nc, uint32_t attrword);
// static inline unsigned ncplane_bchannel(struct ncplane* nc);
// static inline unsigned ncplane_fchannel(struct ncplane* nc);
// static inline unsigned ncplane_fg(struct ncplane* nc);
// static inline unsigned ncplane_bg(struct ncplane* nc);
// static inline unsigned ncplane_fg_alpha(struct ncplane* nc);
// static inline unsigned ncplane_bg_alpha(struct ncplane* nc);
// static inline unsigned ncplane_fg_rgb(struct ncplane* n, unsigned* r, unsigned* g, unsigned* b);
// static inline unsigned ncplane_bg_rgb(struct ncplane* n, unsigned* r, unsigned* g, unsigned* b);
// int ncplane_set_fg_rgb(struct ncplane* n, int r, int g, int b);
// int ncplane_set_bg_rgb(struct ncplane* n, int r, int g, int b);
// void ncplane_set_fg_rgb_clipped(struct ncplane* n, int r, int g, int b);
// void ncplane_set_bg_rgb_clipped(struct ncplane* n, int r, int g, int b);
// int ncplane_set_fg(struct ncplane* n, unsigned channel);
// int ncplane_set_bg(struct ncplane* n, unsigned channel);
// void ncplane_set_fg_default(struct ncplane* n);
// void ncplane_set_bg_default(struct ncplane* n);
// int ncplane_set_fg_alpha(struct ncplane* n, unsigned alpha);
// int ncplane_set_bg_alpha(struct ncplane* n, unsigned alpha);
// int ncplane_set_fg_palindex(struct ncplane* n, int idx);
// int ncplane_set_bg_palindex(struct ncplane* n, int idx);
// void ncplane_styles_set(struct ncplane* n, unsigned stylebits);
// void ncplane_styles_on(struct ncplane* n, unsigned stylebits);
// void ncplane_styles_off(struct ncplane* n, unsigned stylebits);
// unsigned ncplane_styles(const struct ncplane* n);
// void ncplane_greyscale(struct ncplane* n);
// int ncplane_blit_bgrx(struct ncplane* nc, int placey, int placex, int linesize, ncblitter_e blitter, const unsigned char* data, int begy, int begx, int leny, int lenx);
// int ncplane_blit_rgba(struct ncplane* nc, int placey, int placex, int linesize, ncblitter_e blitter, const unsigned char* data, int begy, int begx, int leny, int lenx);
// int ncplane_destroy(struct ncplane* ncp);
// void notcurses_drop_planes(struct notcurses* nc);
// int ncplane_mergedown(struct ncplane* restrict src, struct ncplane* restrict dst);
//+void ncplane_erase(struct ncplane* n);
// bool ncplane_set_scrolling(struct ncplane* n, bool scrollp);
// int ncplane_rotate_cw(struct ncplane* n);
// int ncplane_rotate_ccw(struct ncplane* n);
//
// NOTE: it is an error to call ncplane_destroy, ncplane_resize, or ncplane_move on the standard plane.
//
impl NcPlane {
    // CONSTRUCTORS: aligned(), bound(), dup(), new()

    pub fn aligned(plane: NcPlane, rows: i32, cols: i32, yoff: i32, align: NcAlign) -> Result<Self, NcError> {
        Ok(NcPlane {
            data: unsafe {
                nc::ncplane_aligned(
                    plane.data,
                    rows,
                    cols,
                    yoff,
                    align as nc::ncalign_e,
                    null_mut(),
                )
            },
        })
    }

    /// Creates a new plane bound to another plane
    ///
    /// If a plane is bound to another, x and y coordinates are relative to the plane to which it is bound,
    /// and if that plane moves, all its bound planes move along with it.
    /// When a plane is destroyed, all planes bound to it (directly or transitively) are destroyed.
    pub fn bound(plane: NcPlane, rows: i32, cols: i32, yoff: i32, xoff: i32) -> Result<Self, NcError> {
        Ok(NcPlane {
            data: unsafe { nc::ncplane_bound(plane.data, rows, cols, yoff, xoff, null_mut()) },
        })
    }

    /// Duplicates a plane
    pub fn dup(&self) -> Result<Self, NcError> {
        Ok(NcPlane {
            data: unsafe { nc::ncplane_dup(self.data, null_mut()) },
        })
    }

    /// Creates a new plane
    pub fn new(notcurses: &mut NotCurses, rows: i32, cols: i32, yoff: i32, xoff: i32) -> Result<Self, NcError> {
        Ok(NcPlane {
            // https://nick-black.com/notcurses/html/notcurses_8c.html#ae53e76e41aa6f82e1f1130093df1b007
            data: unsafe { nc::ncplane_new(notcurses.data, rows, cols, yoff, xoff, null_mut()) },
        })
    }

    /// Returns a new NcPlane struct from an existing notcurses_ncplane struct
    pub(crate) fn new_from(ncplane: *mut nc::ncplane) -> Self {
        NcPlane { data: ncplane }
    }

    // ---

    // TODO
    ///
    /// NOTE: Errors when called on the standard plane
    pub fn destroy(&mut self) {}

    /// Returns the plane dimension x
    pub fn dim_x(&self) -> i32 {
        nc::ncplane_dim_x(self.data)
    }

    /// Returns the plane dimension y
    pub fn dim_y(&self) -> i32 {
        nc::ncplane_dim_y(self.data)
    }

    /// Returns a tuple with the plane dimensions (y,x)
    pub fn dim_yx(&self) -> (i32, i32) {
        let mut y = 0;
        let mut x = 0;
        unsafe { nc::ncplane_dim_yx(self.data, &mut y, &mut x) }
        (y, x)
    }

    /// Zeroes out every cell of the plane, dumps the egcpool, and homes the cursor. The base cell is preserved.
    pub fn erase(&mut self) {
        unsafe {
            nc::ncplane_erase(self.data);
        }
    }

    // TODO
    ///
    /// NOTE: Errors when called on the standard plane
    pub fn r#move(&mut self) {}

    /// Detaches the plane from any plane to which it is bound, and binds it to newparent if newparent is not NULL
    pub fn reparent(&mut self, newparent: &mut NcPlane) {
        unsafe { nc::ncplane_reparent(self.data, newparent.data) };
    }

    /// Resizing is a very general and powerful operation
    ///
    /// It is possible to implement `move_yx()` in terms of `resize()`.
    /// The four parameters `keepy`, `keepx`, `keepleny`, and `keeplenx` define a subset of the plane to retain.
    /// The retained rectangle has its origin at `keepy`, `keepx`, and a `keepleny`-row, `keeplenx`-column geometry.
    /// If either of the dimensions are zero, no material is retained. In this case, keepx and keepy are immaterial,
    /// save that in no case may any of these four parameters be negative.
    /// `keepx` and `keepy` are both relative to the plane’s origins, not the rendering area.
    /// Attempting to “retain” material beyond the boundaries of the plane is an error.
    /// `yoff` and `xoff` are likewise relative to the plane’s origin, and define the geometry of the plane following the resize. Both of these
    /// arguments must be positive. Attempting to retain more material than there is room in the reshaped plane is an error.
    ///
    /// NOTE: Errors when called on the standard plane
    // [C source](https://nick-black.com/notcurses/html/notcurses_8c.html#a0f66685d25b59f0e9ab3726076041f24)
    pub fn resize(
        &mut self,
        keepy: i32,
        keepx: i32,
        keepleny: i32,
        keeplenx: i32,
        yoff: i32,
        xoff: i32,
        ylen: i32,
        xlen: i32,
    ) {
        unsafe {
            nc::ncplane_resize(
                self.data, keepy, keepx, keepleny, keeplenx, yoff, xoff, ylen, xlen,
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::notcurses::NcOptions;

    /*
    #[test]
    fn () -> Result<(), NcError> {
        let mut nc = NotCurses::new_default_test()?;
        let plane = NcPlane::new(&mut nc, 50, 100, 0, 0)?;
        assert_eq!(, );
        Ok()
    }
    */

    #[test]
    fn new() -> Result<(), NcError> {
        let mut nc = NotCurses::new_default_test()?;
        NcPlane::new(&mut nc, 50, 100, 0, 0)?;
        Ok(())
    }

    #[test]
    fn dim_x() -> Result<(), NcError> {
        let mut nc = NotCurses::new_default_test()?;
        let plane = NcPlane::new(&mut nc, 50, 100, 0, 0)?;
        assert_eq!(100, plane.dim_x());
        Ok(())
    }

    #[test]
    fn dim_y() -> Result<(), NcError> {
        let mut nc = NotCurses::new_default_test()?;
        let plane = NcPlane::new(&mut nc, 50, 100, 0, 0)?;
        assert_eq!(50, plane.dim_y());
        Ok(())
    }

    #[test]
    fn dim_yx() -> Result<(), NcError> {
        let mut nc = NotCurses::new_default_test()?;
        let plane = NcPlane::new(&mut nc, 50, 100, 0, 0)?;
        assert_eq!((50, 100), plane.dim_yx());
        Ok(())
    }
}
