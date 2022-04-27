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

/// Macro for implementing `ops` traits over pairs. (saturates)
///
macro_rules! impl_ops {
    // non-assign ops:

    // implements a single `non-assign` operator. Saturating.
    (op: $op:tt, $fn:ident, $for: ty, $rhs:ty, =$result:ty) => {
        impl core::ops::$op<$rhs> for $for {
            type Output = $result;
            fn $fn(self, rhs: $rhs) -> Self::Output {
                paste::paste! {
                    <$result>::new(
                        self.0.[< saturating_ $fn >](rhs.0),
                        self.1.[< saturating_ $fn >](rhs.1),
                    )
                }
            }
        }
    };

    // implements a single `non-assign` operator.
    (op_cast: $c1:ty, $c2:ty, $op:tt, $fn:ident, $for: ty, $rhs:ty, =$result:ty) => {
        impl core::ops::$op<$rhs> for $for {
            type Output = $result;
            fn $fn(self, rhs: $rhs) -> Self::Output {
                use az::SaturatingAs;
                paste::paste! {
                    <$result>::new(
                        ((self.0.saturating_as::<$c1>()).[< saturating_ $fn >](
                            rhs.0.saturating_as::<$c1>())).saturating_as::<$c2>(),
                        ((self.1.saturating_as::<$c1>()).[< saturating_ $fn >](
                            rhs.1.saturating_as::<$c1>())).saturating_as::<$c2>(),
                    )
                }
            }
        }
    };

    // assign ops:

    // implements a single `assign` operator. Saturating.
    (opa: $op:tt, $fn:ident, $for:ty, $rhs:ty) => {
        impl core::ops::$op<$rhs> for $for {
            paste::paste! {
                fn [< $fn _assign >](&mut self, rhs: $rhs) {
                    paste::paste! {
                        <$for>::new(
                            self.0.[< saturating_ $fn >](rhs.0),
                            self.1.[< saturating_ $fn >](rhs.1),
                        );
                    }
                }
            }
        }
    };

    // implements a single `non-assign` operator. Saturating.
    // WIP
    (opa_cast: $c1:ty, $c2:ty, $op:tt, $fn: ident, $for:ty, $rhs:ty) => {
        impl core::ops::$op<$rhs> for $for {
            fn $fn(&mut self, rhs: $rhs) {
                use az::SaturatingAs;
                <$for>::new(
                    (self.0.saturating_as::<$c1>().[< saturating_ $fn >](
                        rhs.0.saturating_as::<$c1>())).saturating_as::<$c2>(),
                    (self.1.saturating_as::<$c1>().[< saturating_ $fn >](
                        rhs.1.saturating_as::<$c1>())).saturating_as::<$c2>(),
                );

            }
        }
    };
}

/// Implements `From<($element, $element)>` and `From<[$element; 2]>` for `$pair` and viceversa.
macro_rules! impl_from_tuple_array {
    // multiple types
    ($pair:ty, $pair_type:ty, list: $( $element:ty ),+ ) => {
        $(
            impl_from_tuple_array![$pair, $pair_type, $element];
        )+
    };

    // single type
    ($pair:ty, $pair_type:ty, $element:ty) => {
        // tuples
        impl From<($element, $element)> for $pair {
            fn from(tuple: ($element, $element)) -> $pair {
                use az::SaturatingAs;
                <$pair>::new(tuple.0.saturating_as::<$pair_type>(), tuple.1.saturating_as::<$pair_type>())
            }
        }
        impl From<$pair> for ($element, $element) {
            fn from(pair: $pair) -> ($element, $element) {
                use az::SaturatingAs;
                (pair.0.saturating_as::<$element>(), pair.1.saturating_as::<$element>())
            }
        }

        // arrays:
        impl From<[$element; 2]> for $pair {
            fn from(array: [$element; 2]) -> $pair {
                use az::SaturatingAs;
                <$pair>::new(array[0].saturating_as::<$pair_type>(), array[1].saturating_as::<$pair_type>())
            }
        }
        impl From<$pair> for [$element; 2] {
            fn from(pair: $pair) -> [$element; 2] {
                use az::SaturatingAs;
                [pair.0.saturating_as::<$element>(), pair.1.saturating_as::<$element>()]
            }
        }
    }
}

// Creates `Size`.
// -----------------------------------------------------------------------------
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

impl_ops![op: Add, add, Size, Size, =Size];
impl_ops![op: Sub, sub, Size, Size, =Size];
impl_ops![op: Mul, mul, Size, Size, =Size];
impl_ops![op: Div, div, Size, Size, =Size];
impl_ops![opa: AddAssign, add, Size, Size];
impl_ops![opa: SubAssign, sub, Size, Size];
impl_ops![opa: MulAssign, mul, Size, Size];
impl_ops![opa: DivAssign, div, Size, Size];
impl_ops![op_cast: i64, u32, Add, add, Size, Offset, =Size];

impl_from_tuple_array![Size, u32, list: u8, u16, u32, i32];

// Creates `Coord`.
// -----------------------------------------------------------------------------
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

impl_ops![op: Add, add, Coord, Coord, =Coord];
impl_ops![op: Sub, sub, Coord, Coord, =Coord];
impl_ops![opa: AddAssign, add, Coord, Coord];
impl_ops![opa: SubAssign, sub, Coord, Coord];
impl_ops![op_cast: i64, u32, Add, add, Coord, Offset, =Coord];

impl_from_tuple_array![Coord, u32, list: u8, u16, u32];

// Creates `Offset`.
// -----------------------------------------------------------------------------
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

// Offset
impl_ops![op: Add, add, Offset, Offset, =Offset];
impl_ops![op: Sub, sub, Offset, Offset, =Offset];
impl_ops![opa: AddAssign, add, Offset, Offset];
impl_ops![opa: SubAssign, sub, Offset, Offset];

impl_from_tuple_array![Offset, i32, list: i8, i16, i32];

// TESTS
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ops_coord_offset() {
        assert_eq![Coord::new(8, 14), Coord::new(10, 10) + Offset::new(-2, 4)];
        assert_eq![Coord::new(0, 0), Coord::new(10, 10) + Offset::new(-20, -20)];
    }

    fn ops_size_saturating() {
        assert_eq![Size::new(0, 0), (-10, -10).into()];
    }
}
