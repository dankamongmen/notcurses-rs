// notcurses::plane::style
//
//!
//

use crate::sys::{c_api::NcStyle_u16, NcStyle};

/// A bitmask of styles.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Style(NcStyle_u16);

/// # Flags
#[allow(non_upper_case_globals)]
impl Style {
    /// Bold.
    pub const Bold: Self = Self(NcStyle::Bold.0);

    /// Italic.
    pub const Italic: Self = Self(NcStyle::Italic.0);

    /// Struck.
    pub const Struck: Self = Self(NcStyle::Struck.0);

    /// Underline.
    pub const Underline: Self = Self(NcStyle::Underline.0);

    /// Undercurl.
    pub const Undercurl: Self = Self(NcStyle::Undercurl.0);

    /// None of the styles (all bits set to 0).
    pub const None: Self = Self(0);

    /// The mask of all styles (all bits set to 1).
    pub const Mask: Self = Self(NcStyle::Mask.0);
}

mod std_impls {
    use super::{NcStyle, NcStyle_u16, Style};
    use std::fmt;

    impl Default for Style {
        fn default() -> Self {
            Style::None
        }
    }

    impl fmt::Display for Style {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut string = String::new();
            for s in self.to_vec() {
                string.push_str(match s {
                    Style::Italic => "Italic ",
                    Style::Underline => "Underline ",
                    Style::Undercurl => "Undercurl ",
                    Style::Struck => "Struck ",
                    Style::Bold => "Bold ",
                    Style::None => "None ",
                    _ => "",
                });
            }
            let _ = string.pop();
            write!(f, "{}", string)
        }
    }

    impl fmt::Debug for Style {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut string = String::new();
            for s in self.to_vec() {
                string.push_str(match s {
                    Style::Italic => "Italic+",
                    Style::Underline => "Underline+",
                    Style::Undercurl => "Undercurl+",
                    Style::Struck => "Struck+",
                    Style::Bold => "Bold+",
                    Style::None => "None ",
                    _ => "",
                });
            }
            let _ = string.pop();
            write!(f, "{}", string)
        }
    }

    //

    crate::from_primitive![Style, NcStyle_u16];
    crate::unit_impl_ops![bitwise; Style, NcStyle_u16];
    crate::unit_impl_fmt![bases; Style];

    //

    impl From<NcStyle> for Style {
        fn from(nc: NcStyle) -> Style {
            let mut new = Style::None;
            for s in nc.to_vec() {
                match s {
                    NcStyle::Italic => new.set(Style::Italic),
                    NcStyle::Underline => new.set(Style::Underline),
                    NcStyle::Undercurl => new.set(Style::Undercurl),
                    NcStyle::Struck => new.set(Style::Struck),
                    NcStyle::Bold => new.set(Style::Bold),
                    _ => (),
                };
            }
            new
        }
    }
    impl From<Style> for NcStyle {
        fn from(style: Style) -> NcStyle {
            let mut new = NcStyle::None;
            for s in style.to_vec() {
                match s {
                    Style::Italic => new.set(NcStyle::Italic),
                    Style::Underline => new.set(NcStyle::Underline),
                    Style::Undercurl => new.set(NcStyle::Undercurl),
                    Style::Struck => new.set(NcStyle::Struck),
                    Style::Bold => new.set(NcStyle::Bold),
                    _ => (),
                };
            }
            new
        }
    }

    impl From<NcStyle_u16> for Style {
        fn from(nci: NcStyle_u16) -> Style {
            NcStyle::from(nci).into()
        }
    }
    impl From<Style> for NcStyle_u16 {
        fn from(pi: Style) -> NcStyle_u16 {
            NcStyle::from(pi).into()
        }
    }
}

/// # methods
impl Style {
    /// Returns a style from a case-insensitive string representation.
    ///
    /// The received styles must be separated by spaces.
    pub fn from_names(names: &str) -> Style {
        let mut style = Style::None;
        for s in names.split(' ') {
            match s.to_lowercase().as_str() {
                "italic" => style.set(Style::Italic),
                "underline" => style.set(Style::Underline),
                "undercurl" => style.set(Style::Undercurl),
                "struck" => style.set(Style::Struck),
                "bold" => style.set(Style::Bold),
                _ => (),
            }
        }
        style
    }
}

/// # methods
impl Style {
    /// Returns a `Vec` with all the styles contained in the current style.
    pub fn to_vec(&self) -> Vec<Style> {
        let mut v = vec![];
        let styles = [
            Style::Italic,
            Style::Underline,
            Style::Undercurl,
            Style::Struck,
            Style::Bold,
        ];
        for s in &styles {
            if self.has(*s) {
                v.push(*s)
            }
        }
        if v.is_empty() {
            v.push(Style::None);
        }
        v
    }

    /// Returns true if the current style has included the `other_style`.
    pub fn has(&self, other: impl Into<Style>) -> bool {
        let other = other.into();
        (self.0 & other.0) == other.0
    }

    /// Sets the `other` style in the current style.
    pub fn set(&mut self, other: impl Into<Style>) {
        self.0 |= other.into().0
    }

    /// Unsets the `other` style in the current style.
    pub fn unset(&mut self, other: impl Into<Style>) {
        self.0 &= !other.into().0
    }
}
