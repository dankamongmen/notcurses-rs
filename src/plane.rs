// total functions: 139 (97+42)
// ------------------------------------------ (done / remaining)
// - done: ±11 / ±117
// - test:  4 / 124
// ------------------------- ↓ from bindgen: 97
//
//  ncplane_above
//+ ncplane_aligned          <<<< FIXME (deprecated) (see _TODO.md)
//  ncplane_at_cursor
//  ncplane_at_yx
//  ncplane_base
//  ncplane_below
//+ ncplane_bound            <<<< FIXME (deprecated) (see _TODO.md)
//  ncplane_box
//  ncplane_center_abs
//  ncplane_channels
//  ncplane_contents
//  ncplane_cursor_move_yx
//  ncplane_cursor_yx
//  ncplane_destroy
//# ncplane_dim_yx
//  ncplane_dup
//+ ncplane_erase
//  ncplane_fadein
//  ncplane_fadein_iteration
//  ncplane_fadeout
//  ncplane_fadeout_iteration
//  ncplane_format
//  ncplane_gradient
//  ncplane_greyscale
//  ncplane_highgradient
//  ncplane_highgradient_sized
//  ncplane_hline_interp
//  ncplane_home
//  ncplane_mergedown
//  ncplane_mergedown_simple
//  ncplane_move_above
//  ncplane_move_below
//  ncplane_move_bottom
//  ncplane_move_top
//  ncplane_move_yx
//# ncplane_new
//  ncplane_notcurses
//  ncplane_notcurses_const
//  ncplane_off_styles
//  ncplane_on_styles
//  ncplane_polyfill_yx
//  ncplane_parent
//  ncplane_parent_const
//  ncplane_polyfill_yx
//  ncplane_pulse
//  ncplane_putchar_stained
//  ncplane_putc_yx
//  ncplane_putegc_stained
//  ncplane_putegc_yx
//  ncplane_putnstr_aligned
//  ncplane_putnstr_yx
//  ncplane_putstr_aligned
//  ncplane_putstr_stained
//  ncplane_putstr_yx
//  ncplane_puttext
//  ncplane_putwegc_stained
//  ncplane_putwstr_stained
//  ncplane_qrcode
//+ ncplane_reparent
//+ ncplane_resize
//+ ncplane_resize_realign
//  ncplane_rgba
//  ncplane_rotate_ccw
//  ncplane_rotate_cw
//  ncplane_set_base
//  ncplane_set_base_cell
//  ncplane_set_bchannel
//  ncplane_set_bg_alpha
//  ncplane_set_bg_default
//  ncplane_set_bg_palindex
//  ncplane_set_bg_rgb
//  ncplane_set_bg_rgb8
//  ncplane_set_bg_rgb8_clipped
//  ncplane_set_channels
//  ncplane_set_fchannel
//  ncplane_set_fg_alpha
//  ncplane_set_fg_default
//  ncplane_set_fg_palindex
//  ncplane_set_fg_rgb
//  ncplane_set_fg_rgb8
//  ncplane_set_fg_rgb8_clipped
//  ncplane_set_scrolling
//  ncplane_set_styles
//  ncplane_set_userptr
//  ncplane_stain
//  ncplane_styles
//  ncplane_styles_off
//  ncplane_styles_on
//  ncplane_styles_set
//  ncplane_translate
//  ncplane_translate_abs
//  ncplane_userptr
//  ncplane_vline_interp
//  ncplane_vprintf_aligned
//  ncplane_vprintf_stained
//  ncplane_vprintf_yx
//  ncplane_x
//  ncplane_y
//  ncplane_yx
// ------------------------- ↓ static inlines reimplemented: 42
// ncplane_align
// ncplane_at_cursor_cell
// ncplane_at_yx_cell
// ncplane_bchannel
// ncplane_bg_alpha
// ncplane_bg_default_p
// ncplane_bg_rgb
// ncplane_bg_rgb8
// ncplane_box_sized
//#ncplane_dim_x
//#ncplane_dim_y
// ncplane_double_box
// ncplane_double_box_sized
// ncplane_fchannel
// ncplane_fg_alpha
// ncplane_fg_default_p
// ncplane_fg_rgb
// ncplane_fg_rgb8
// ncplane_gradient_sized
// ncplane_highgradient_sized
// ncplane_hline
// ncplane_perimeter
// ncplane_perimeter_double
// ncplane_perimeter_rounded
// ncplane_putc
// ncplane_putegc
//+ncplane_putnstr
// ncplane_putsimple
// ncplane_putsimple_yx
//+ncplane_putstr
// ncplane_putwc
// ncplane_putwc_yx
// ncplane_putwegc
// ncplane_putwegc_yx
// ncplane_putwstr
// ncplane_putwstr_aligned
// ncplane_putwstr_yx
// ncplane_resize_simple
// ncplane_rounded_box
// ncplane_rounded_box_sized
// ncplane_vline
// ncplane_vprintf

use cstr_core::CString;
use std::ptr::{null, null_mut};

use crate::{sys, Align, Error, FullMode};

/// NCPLANE
///
/// New planes can be created with `new()`, `bound()`, and `aligned()`.
/// dup() duplicates a plane.
///
// NOTE: the previous 4 functions internally can fail and return null
// https://nick-black.com/notcurses/notcurses_plane.3.html#return-values
///
// TODO: EXTRACT FROM:
//
// - Following initialization, a single ncplane exists, the “standard plane”.s
//   This plane cannot be destroyed
//   nor manually resized, and is always exactly as large as the screen.
// - Further `Plane`s can be created with `Plane::new()`. A total z-ordering
//   always exists on the set of
//   `Plane`s, and new `Plane`s are placed at the top of the z-buffer.
// - `Plane`s can be larger, smaller, or the same size as the physical screen,
//   and can be placed anywhere relative to it (including entirely off-screen).
// - TODO: `Plane`s are made up of `NcCell`s cells.
// - TODO: All user-created planes can be destroyed in one call with
//   notcurses_drop_planes()
// - It is generally more performant to “hide” planes at the bottom of the stack,
//   ideally behind a large opaque plane, rather than moving them beyond the
//   boundaries of the visible window.
// - Planes ought be no larger than necessary, so that they intersect with the
//   minimum number of cells.
//
// Planes are the fundamental drawing object of notcurses.
//
// TODO: (METHODS?) All output functions take a struct `Plane` as an argument.
//
// They can be any size, and placed anywhere.
// In addition to its framebuffer, a Plane is defined by:
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
pub struct Plane {
    data: *mut sys::NcPlane,
}

pub struct PlaneOptions {
    data: *mut sys::NcPlaneOptions,
}

impl PlaneOptions {
    /// PlaneOptions constructor
    //
    // TODO: add additional constructors for:
    // -name (&str) used for debuging
    // - callback
    // - userptr
    pub fn new(y: i32, x: i32, rows: u32, cols: u32) -> Self {
        Self {
            data: &mut sys::NcPlaneOptions::new(y, x, rows, cols),
        }
    }

    pub fn new_halign(y: i32, align: Align, rows: u32, cols: u32) -> Self {
        Self {
            data: &mut sys::NcPlaneOptions::new_halign(y, align as sys::NcAlign, rows, cols),
        }
    }

    pub fn with_flags(y: i32, x: i32, rows: u32, cols: u32, flags: u64) -> Self {
        Self {
            data: &mut sys::NcPlaneOptions::with_flags(y, x, rows, cols, flags),
        }
    }
}

// NOTE: it is an error to call ncplane_destroy, ncplane_resize, or ncplane_move on the standard plane.
//
impl Plane {
    // CONSTRUCTORS: aligned(), bound(), dup(), new() --------------------------

    /*
    // TODO: DELETE when implemented Create

    pub fn aligned(
        plane: Plane,
        rows: i32,
        cols: i32,
        yoff: i32,
        align: Align,
    ) -> Result<Self, Error> {
        Ok(Plane {
            data: unsafe {
                sys::ncplane_aligned(
                    plane.data,
                    rows,
                    cols,
                    yoff,
                    align as sys::NcAlign,
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
    pub fn bound(
        plane: Plane,
        rows: i32,
        cols: i32,
        yoff: i32,
        xoff: i32,
    ) -> Result<Self, Error> {
        Ok(Plane {
            data: unsafe { sys::ncplane_bound(plane.data, rows, cols, yoff, xoff, null_mut()) },
        })
    }
    */

    /// Duplicates a plane
    pub fn dup(&self) -> Result<Self, Error> {
        Ok(Plane {
            data: unsafe { sys::ncplane_dup(self.data, null_mut()) },
        })
    }

    /// Creates a new plane, bound to another plane
    pub fn new(bplane: &mut Plane, y: i32, x: i32, cols: u32, rows: u32) -> Result<Self, Error> {
        let options = PlaneOptions::new(y, x, cols, rows);
        Ok(Self {
            data: unsafe { sys::ncplane_create(bplane.data, options.data) },
        })
    }

    /// Creates a new plane (with flags), bound to another plane
    // NOTE unnecessary duplication?
    pub fn with_flags(
        bplane: &mut Plane,
        y: i32,
        x: i32,
        cols: u32,
        rows: u32,
        flags: u64,
    ) -> Result<Self, Error> {
        let options = PlaneOptions::with_flags(y, x, cols, rows, flags);
        Ok(Self {
            data: unsafe { sys::ncplane_create(bplane.data, options.data) },
        })
    }

    // DEPRECATED
    /*
    pub fn new_old(
        notcurses: &mut FullMode,
        rows: i32,
        cols: i32,
        yoff: i32,
        xoff: i32,
    ) -> Result<Self, Error> {
        Ok(Plane {
            // https://nick-black.com/notcurses/html/notcurses_8c.html#ae53e76e41aa6f82e1f1130093df1b007
            // data: unsafe { sys::ncplane_new(notcurses.data, rows, cols, yoff, xoff, null_mut()) },
            data: unsafe { sys::ncplane_create(notcurses.data) },
        })
    }
    */

    /// Returns a new Plane struct from an existing notcurses_ncplane struct
    pub(crate) fn new_from(ncplane: *mut sys::NcPlane) -> Self {
        Plane { data: ncplane }
    }

    // ----------------------------------------------------------^ CONSTRUCTORS

    // TODO
    ///
    /// NOTE: Errors when called on the standard plane
    pub fn destroy(&mut self) {}

    /// Returns the plane dimension x
    pub fn dim_x(&self) -> i32 {
        unsafe { sys::ncplane_dim_x(&*self.data) }
    }

    /// Returns the plane dimension y
    pub fn dim_y(&self) -> i32 {
        unsafe { sys::ncplane_dim_y(&*self.data) }
    }

    /// Returns a tuple with the plane dimensions (y,x)
    pub fn dim_yx(&self) -> (i32, i32) {
        let mut y = 0;
        let mut x = 0;
        unsafe { sys::ncplane_dim_yx(self.data, &mut y, &mut x) }
        (y, x)
    }

    /// Zeroes out every cell of the plane, dumps the egcpool, and homes the cursor. The base cell is preserved.
    pub fn erase(&mut self) {
        unsafe {
            sys::ncplane_erase(self.data);
        }
    }

    // /// Replace the cell at the specified coordinates with the provided cell
    // pub fn putc_yx(&mut self, cell: ) -> Result<Self, Error> {
    //
    //     Ok(())
    // }

    // TODO
    // !cell_char_initializer[]

    ///
    ///
    pub fn putstr(&mut self, text: &str) {
        //let ptext = CString::new(text).unwrap().as_ptr();
        unsafe {
            sys::ncplane_putstr(&mut *self.data, text.as_bytes());
        }
    }
    /// convenience wrappers over ncplane_putnstr
    ///
    /// no more than len bytes will be written
    ///
    pub fn putnstr(&mut self, len: usize, text: &str) {
        unsafe {
            sys::ncplane_putnstr(&mut *self.data, len as u64, text.as_bytes());
        }
    }

    // TODO
    ///
    /// NOTE: Errors when called on the standard plane
    pub fn r#move(&mut self) {}

    /// Detaches the plane from any plane to which it is bound, and binds it to newparent if newparent is not NULL
    pub fn reparent(&mut self, newparent: &mut Plane) {
        unsafe { sys::ncplane_reparent(self.data, newparent.data) };
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
    #[allow(clippy::too_many_arguments)]
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
            sys::ncplane_resize(
                self.data, keepy, keepx, keepleny, keeplenx, yoff, xoff, ylen, xlen,
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
    #[test]
    fn () -> Result<(), Error> {
        let mut nc = FullMode::new_test()?;
        let plane = Plane::new(&mut nc, 50, 100, 0, 0)?;
        assert_eq!(, );
        Ok()
    }
    */

    #[test]
    #[ignore]
    fn new() -> Result<(), Error> {
        let mut nc = FullMode::new_test()?;
        Plane::new(&mut nc.stdplane(), 50, 100, 0, 0)?;
        Ok(())
    }
    #[test]
    #[ignore]
    fn dim_x() -> Result<(), Error> {
        let mut nc = FullMode::new_test()?;

        let plane = Plane::new(&mut nc.stdplane(), 50, 100, 0, 0)?;
        println!("{:?}", plane.dim_y());
        assert_eq!(100, 100);
        Ok(())
    }

    #[test]
    #[ignore]
    fn dim_y() -> Result<(), Error> {
        let mut nc = FullMode::new_test()?;
        let plane = Plane::new(&mut nc.stdplane(), 50, 100, 0, 0)?;
        assert_eq!(50, plane.dim_y());
        Ok(())
    }

    #[test]
    #[ignore]
    fn dim_yx() -> Result<(), Error> {
        let mut nc = FullMode::new_test()?;
        let plane = Plane::new(&mut nc.stdplane(), 50, 100, 0, 0)?;
        assert_eq!((50, 100), plane.dim_yx());
        Ok(())
    }
}
