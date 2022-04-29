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
    "A pair of positive lengths.\n\n`(height, width)` | `(vertical, horizontal)`.",
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

impl_ops![pair_op: Add, add, Size, Size, =Size, u32];
impl_ops![pair_op: Sub, sub, Size, Size, =Size, u32];
impl_ops![pair_op: Mul, mul, Size, Size, =Size, u32];
impl_ops![pair_op: Div, div, Size, Size, =Size, u32];
impl_ops![pair_opa: AddAssign, add, Size, Size, u32];
impl_ops![pair_opa: SubAssign, sub, Size, Size, u32];
impl_ops![pair_opa: MulAssign, mul, Size, Size, u32];
impl_ops![pair_opa: DivAssign, div, Size, Size, u32];

// Size *op* Position = Size
impl_ops![pair_op: Add, add, Size, Position, =Size, u32];
impl_ops![pair_op: Sub, sub, Size, Position, =Size, u32];
impl_ops![pair_op: Mul, mul, Size, Position, =Size, u32];
impl_ops![pair_op: Div, div, Size, Position, =Size, u32];
impl_ops![pair_opa: AddAssign, add, Size, Position, u32];
impl_ops![pair_opa: SubAssign, sub, Size, Position, u32];
impl_ops![pair_opa: MulAssign, mul, Size, Position, u32];
impl_ops![pair_opa: DivAssign, div, Size, Position, u32];

#[rustfmt::skip]
impl_from_tuple_array![Size, u32, list: u8, u16, u32, u64, usize, i8, i16, i32, i64, isize];
impl_methods![Size, u32];

// Creates `Position`.
// -----------------------------------------------------------------------------
create_pair![
    "A pair of coordinates.\n\n`(y, x)` | `(row, column)` | `(vertical, horizontal)`.",
    "coordinate",
    Position,
    i32,
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

impl_ops![pair_op: Add, add, Position, Position, =Position, i32];
impl_ops![pair_op: Sub, sub, Position, Position, =Position, i32];
impl_ops![pair_op: Mul, mul, Position, Position, =Position, i32];
impl_ops![pair_op: Div, div, Position, Position, =Position, i32];
impl_ops![pair_opa: AddAssign, add, Position, Position, i32];
impl_ops![pair_opa: SubAssign, sub, Position, Position, i32];
impl_ops![pair_opa: MulAssign, mul, Position, Position, i32];
impl_ops![pair_opa: DivAssign, div, Position, Position, i32];

#[rustfmt::skip]
impl_from_tuple_array![Position, i32, list: u8, u16, u32, usize, i8, i16, i32, i64, isize];
impl_methods![Position, i32];
