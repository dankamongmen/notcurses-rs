// notcurses::geometry::pairs
//
//!
//

/// Creates a structure with a pair of interrelated fields.
///
/// Args:
/// - $tname: the name of the structure.
/// - $ty: the type of the fields.
/// - $n0, $n1: the names of the main methods.
/// - $( [$c_name, {$c_f0, c_f1}, $c_m0, $c_m1] )*: optional custom constructors.
/// - $( ($x_name, $x_method, x_field) )*: optional extra getters & setters.
///
macro_rules! create_pair {
    (
     $doc:literal, $dname:literal,
     $tname:ident, $ftype:ty, $n0:ident, $n1:ident
     $(, [$c_mname:ident, $c_m0:ident, $c_m1:ident] )*
     $(, ($x_name:ident, $x_method:ident, $x_field:tt) )*
     ) =>  {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct $tname(pub $ftype, pub $ftype);

        /// # Constructors
        impl $tname {
            paste::paste! {
                $(
                    create_pair![new: $tname, $c_mname, $ftype, $c_m0, $c_m1];
                )*
            }
        }

        /// # Methods
        impl $tname {
            paste::paste! {
                create_pair![get: $dname, $n0, $n0, $ftype, 0];
                create_pair![get: $dname, $n1, $n1, $ftype, 1];
                create_pair![set: $dname, $n0, $n0, $ftype, 0];
                create_pair![set: $dname, $n1, $n1, $ftype, 1];

                // extra getters & setters
                $(
                    create_pair![get: $dname, $x_name, $x_method, $ftype, $x_field];
                    create_pair![set: $dname, $x_name, $x_method, $ftype, $x_field];
                )*

            }
        }
    };

    // adds a constructor
    (new: $tname:ident, $method:ident, $ftype:ty, $n0:ident, $n1:ident) => {
        paste::paste! {
            #[doc = "Returns a new `" $tname "`." ]
            pub fn $method($n0: $ftype, $n1: $ftype) -> $tname {
                Self($n0, $n1)
            }
        }
    };

    // adds a getter method
    (get: $dname: literal, $name: ident, $method:ident, $ftype:ty, $field:tt) => {
        paste::paste! {
            #[doc = "Gets the `" $name "` " $dname "." ]
            pub fn $method(&self) -> $ftype {
                self.$field
            }
        }
    };

    // adds a setter method
    (set: $dname: literal, $name: ident, $method:ident, $ftype:ty, $field:tt) => {
        paste::paste! {
            #[doc = "Sets the `" $name "` " $dname "" ]
            pub fn [< set_ $method >](&mut self, $name: $ftype) {
                self.$field = $name
            }
        }
    };
}

/// Creates `Size`.
create_pair![
    "A size from a pair of dimensions.\n\n`(height, width)` | `(vertical, horizontal)`.",
    "dimension",
    Size,
    u32,
    height,
    width,
    [new, vertical, horizontal],
    [new_hw, height, width],
    [new_height_width, height, width],
    (height, h, 0),
    (width, w, 1),
    (vertical, vertical, 0),
    (horizontal, horizontal, 1)
];

/// Creates `Coord`.
create_pair![
    "A coordinate pair.\n\n`(y, x)` | `(row, column)` | `(vertical, horizontal)`.",
    "coordinate",
    Coord,
    u32,
    y,
    x,
    [new, vertical, horizontal],
    [from_yx, y, x],
    [from_rc, row, column],
    [from_row_col, row, column],
    [from_row_column, row, column],
    (row, r, 0),
    (row, row, 0),
    (column, c, 1),
    (column, col, 1),
    (column, column, 1),
    (vertical, vertical, 0),
    (horizontal, horizontal, 1)
];

/// Creates `Offset`.
create_pair![
    "An offset pair.\n\n`(y, x)` | `(rows, columns)` | `(vertical, horizontal)`",
    "offset",
    Offset,
    i32,
    y,
    x,
    [new, vertical, horizontal],
    [from_yx, y, x],
    [from_rc, rows, columns],
    [from_rows_cols, rows, columns],
    [from_rows_columns, rows, columns],
    (row, r, 0),
    (row, rows, 0),
    (column, c, 1),
    (column, cols, 1),
    (column, columns, 1),
    (vertical, vertical, 0),
    (horizontal, horizontal, 1)
];
