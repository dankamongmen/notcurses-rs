// notcurses::tuples
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
        #[derive(Clone, Copy, Default, Eq, PartialEq)]
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

            /// Swaps the values.
            pub fn swap(self) -> Self {
                Self(self.1, self.0)
            }

            /// Swaps the values.
            pub fn set_swap(&mut self) {
                core::mem::swap(&mut self.0, &mut self.1)
            }
        }

        impl core::fmt::Display for $tname {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{},{}", self.0, self.1)
            }
        }

        impl core::fmt::Debug for $tname {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({})", stringify![$tname], self)
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
    // implements all arithmetic operators. Saturating.
    (pair_all_ops: $for: ty, $rhs:ty, =$result:ty, $ftype:ty) => {
        // non-assign ops
        impl_ops![pair_op: Add, add, $for, $rhs, =$result, $ftype];
        impl_ops![pair_op: Sub, sub, $for, $rhs, =$result, $ftype];
        impl_ops![pair_op: Mul, mul, $for, $rhs, =$result, $ftype];
        impl_ops![pair_op: Div, div, $for, $rhs, =$result, $ftype];

        // assign ops
        impl_ops![pair_opa: AddAssign, add, $for, $rhs, $ftype];
        impl_ops![pair_opa: SubAssign, sub, $for, $rhs, $ftype];
        impl_ops![pair_opa: MulAssign, mul, $for, $rhs, $ftype];
        impl_ops![pair_opa: DivAssign, div, $for, $rhs, $ftype];
    };
    //
    // implements a single `non-assign` operator. Saturating.
    (pair_op: $op:tt, $fn:ident, $for: ty, $rhs:ty, =$result:ty, $ftype:ty) => {
        impl core::ops::$op<$rhs> for $for {
            type Output = $result;
            fn $fn(self, rhs: $rhs) -> Self::Output {
                use az::SaturatingAs;
                paste::paste! {
                    <$result>::new(
                        self.0.[< saturating_ $fn >](rhs.0.saturating_as::<$ftype>()),
                        self.1.[< saturating_ $fn >](rhs.1.saturating_as::<$ftype>()),
                    )
                }
            }
        }
    };
    //
    // implements a single `assign` operator. Saturating.
    (pair_opa: $op:tt, $fn:ident, $for:ty, $rhs:ty, $ftype:ty) => {
        impl core::ops::$op<$rhs> for $for {
            paste::paste! {
                fn [< $fn _assign >](&mut self, rhs: $rhs) {
                    use az::SaturatingAs;
                    <$for>::new(
                        self.0.[< saturating_ $fn >](rhs.0.saturating_as::<$ftype>()),
                        self.1.[< saturating_ $fn >](rhs.1.saturating_as::<$ftype>()),
                    );
                }
            }
        }
    };

    //

    (pair_all_ops_ints: $for: ty, =$result:ty, $ftype:ty) => {
        // non-assign ops with integers
        impl_ops![pair_op_ints: Add, add, $for, =$result, $ftype,
            [i8, i16, i32, i64, isize, u8, u16, u32, u64, usize]];
        impl_ops![pair_op_ints: Sub, sub, $for, =$result, $ftype,
            [i8, i16, i32, i64, isize, u8, u16, u32, u64, usize]];
        impl_ops![pair_op_ints: Mul, mul, $for, =$result, $ftype,
            [i8, i16, i32, i64, isize, u8, u16, u32, u64, usize]];
        impl_ops![pair_op_ints: Div, div, $for, =$result, $ftype,
            [i8, i16, i32, i64, isize, u8, u16, u32, u64, usize]];

        impl_ops![pair_opa_ints: AddAssign, add, $for, $ftype,
            [i8, i16, i32, i64, isize, u8, u16, u32, u64, usize]];
        impl_ops![pair_opa_ints: SubAssign, sub, $for, $ftype,
            [i8, i16, i32, i64, isize, u8, u16, u32, u64, usize]];
        impl_ops![pair_opa_ints: MulAssign, mul, $for, $ftype,
            [i8, i16, i32, i64, isize, u8, u16, u32, u64, usize]];
        impl_ops![pair_opa_ints: DivAssign, div, $for, $ftype,
            [i8, i16, i32, i64, isize, u8, u16, u32, u64, usize]];
    };
    //
    // implements a single `non-assign` operator against multiple integer types.
    (pair_op_ints: $op:tt, $fn:ident, $for: ty, =$result:ty, $ftype:ty, [$($int:ty),+]) => {
        $(
            impl_ops![pair_op_int: $op, $fn, $for, $int, =$result, $ftype];
        )+
    };
    //
    // implements a single `non-assign` operator against a single integer type.
    (pair_op_int: $op:tt, $fn:ident, $for: ty, $rhs_int:ty, =$result:ty, $ftype:ty) => {
        impl core::ops::$op<$rhs_int> for $for {
            type Output = $result;
            fn $fn(self, rhs: $rhs_int) -> Self::Output {
                use az::SaturatingAs;
                paste::paste! {
                    <$result>::new(
                        self.0.[< saturating_ $fn >](rhs.saturating_as::<$ftype>()),
                        self.1.[< saturating_ $fn >](rhs.saturating_as::<$ftype>()),
                    )
                }
            }
        }
    };
    //
    // implements a single `assign` operator against multiple integer types.
    (pair_opa_ints: $op:tt, $fn:ident, $for: ty, $ftype:ty, [$($int:ty),+]) => {
        $(
            impl_ops![pair_opa_int: $op, $fn, $for, $int, $ftype];
        )+
    };
    //
    // implements a single `assign` operator against a single integer type.
    (pair_opa_int: $op:tt, $fn:ident, $for:ty, $rhs_int:ty, $ftype:ty) => {
        impl core::ops::$op<$rhs_int> for $for {
            paste::paste! {
                fn [< $fn _assign >](&mut self, rhs: $rhs_int) {
                    use az::SaturatingAs;
                    <$for>::new(
                        self.0.[< saturating_ $fn >](rhs.saturating_as::<$ftype>()),
                        self.1.[< saturating_ $fn >](rhs.saturating_as::<$ftype>()),
                    );
                }
            }
        }
    };

}

/// Implements `From<($element, $element)>` and `From<[$element; 2]>` for `$pair` and viceversa.
///
/// args:
/// - $pair :
/// - $pair_type :
/// - $element :
macro_rules! impl_from_tuple_array {
    // multiple types
    ($pair:ident, $pair_type:ty, list: $( $element:ty ),+ ) => {
        $(
            impl_from_tuple_array![$pair, $pair_type, $element];
        )+
    };

    // single type
    ($pair:ident, $pair_type:ty, $element:ty) => {
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

/// Implements several common methods.
macro_rules! impl_methods {
    ($pair:ty, $ftype:ty) => {
        /// # Methods
        impl $pair {
            /// Returns a tuple with the pair of inner values.
            #[inline]
            pub const fn into_tuple(&self) -> ($ftype, $ftype) {
                (self.0, self.1)
            }

            /// Returns an array with the pair of inner values.
            pub const fn into_array(&self) -> [$ftype; 2] {
                [self.0, self.1]
            }
        }
    };
}

// Creates `Size`.
// -----------------------------------------------------------------------------
create_pair![
    "A pair of positive lengths.\n\n`(x, y)` | `(width, height)` | `(horizontal, vertical)`.",
    "dimension",
    Size,
    u32,
    width,
    height,
    [new, horizontal, vertical],
    [new_wh, width, height],
    [new_width_height, width, height],
    (width, w, 0),
    (height, h, 1),
    (width, x, 0),
    (height, y, 1),
    (width, c, 0),
    (width, cols, 0),
    (width, columns, 0),
    (height, r, 1),
    (height, rows, 1),
    (horizontal, horizontal, 0),
    (vertical, vertical, 1)
];

impl_ops![pair_all_ops: Size, Size, =Size, u32];
impl_ops![pair_all_ops: Size, Position, =Size, u32]; // Size *op* Position = Size
impl_ops![pair_all_ops_ints: Size, =Size, u32]; // Position *op* {int} = Position

#[rustfmt::skip]
impl_from_tuple_array![Size, u32, list: u8, u16, u32, u64, usize, i8, i16, i32, i64, isize];
impl_methods![Size, u32];

// Creates `Position`.
// -----------------------------------------------------------------------------
create_pair![
    "A pair of coordinates.\n\n`(x, y)` | `(column, row)` | `(horizontal, vertical)`.",
    "coordinate",
    Position,
    i32,
    x,
    y,
    [new, horizontal, vertical],
    [from_xy, x, y],
    [from_col_row, column, row],
    [from_column_row, column, row],
    (column, c, 0),
    (column, col, 0),
    (column, column, 0),
    (row, r, 1),
    (row, row, 1),
    (horizontal, horizontal, 0),
    (vertical, vertical, 1)
];

impl_ops![pair_all_ops: Position, Position, =Position, i32];
impl_ops![pair_all_ops: Position, Size, =Position, i32]; // Position *op* Size = Position
impl_ops![pair_all_ops_ints: Position, =Position, i32]; // Position *op* {int} = Position

#[rustfmt::skip]
impl_from_tuple_array![Position, i32, list: u8, u16, u32, usize, i8, i16, i32, i64, isize];
impl_methods![Position, i32];
