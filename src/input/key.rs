//!

use crate::sys::{c_api, NcKey};

/// A synthesized [`Received`][crate::Received] input event other than a `char`.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Key(u32);

mod std_impls {
    use super::{Key, NcKey};
    use std::fmt;

    impl fmt::Display for Key {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.name())
        }
    }

    impl fmt::Debug for Key {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Key::{}", self)
        }
    }

    impl From<NcKey> for Key {
        fn from(nc: NcKey) -> Key {
            Key(nc.0)
        }
    }

    impl From<Key> for NcKey {
        fn from(k: Key) -> NcKey {
            NcKey(k.0)
        }
    }

    // TEMP
    // impl From<NcKey> for u32 {
    //     fn from(k: NcKey) -> Self {
    //         k.0
    //     }
    // }
}

/// # constants
#[allow(non_upper_case_globals)]
impl Key {
    pub const Invalid: Key = Key(c_api::NCKEY_INVALID);
    /// we received `SIGWINCH`.
    pub const Resize: Key = Key(c_api::NCKEY_RESIZE);
    pub const Up: Key = Key(c_api::NCKEY_UP);
    pub const Right: Key = Key(c_api::NCKEY_RIGHT);
    pub const Down: Key = Key(c_api::NCKEY_DOWN);
    pub const Left: Key = Key(c_api::NCKEY_LEFT);
    pub const Ins: Key = Key(c_api::NCKEY_INS);
    pub const Del: Key = Key(c_api::NCKEY_DEL);
    pub const Backspace: Key = Key(c_api::NCKEY_BACKSPACE);
    pub const PgDown: Key = Key(c_api::NCKEY_PGDOWN);
    pub const PgUp: Key = Key(c_api::NCKEY_PGUP);
    pub const Home: Key = Key(c_api::NCKEY_HOME);
    pub const End: Key = Key(c_api::NCKEY_END);
    pub const F00: Key = Key(c_api::NCKEY_F00);
    pub const F01: Key = Key(c_api::NCKEY_F01);
    pub const F02: Key = Key(c_api::NCKEY_F02);
    pub const F03: Key = Key(c_api::NCKEY_F03);
    pub const F04: Key = Key(c_api::NCKEY_F04);
    pub const F05: Key = Key(c_api::NCKEY_F05);
    pub const F06: Key = Key(c_api::NCKEY_F06);
    pub const F07: Key = Key(c_api::NCKEY_F07);
    pub const F08: Key = Key(c_api::NCKEY_F08);
    pub const F09: Key = Key(c_api::NCKEY_F09);
    pub const F10: Key = Key(c_api::NCKEY_F10);
    pub const F11: Key = Key(c_api::NCKEY_F11);
    pub const F12: Key = Key(c_api::NCKEY_F12);
    pub const F13: Key = Key(c_api::NCKEY_F13);
    pub const F14: Key = Key(c_api::NCKEY_F14);
    pub const F15: Key = Key(c_api::NCKEY_F15);
    pub const F16: Key = Key(c_api::NCKEY_F16);
    pub const F17: Key = Key(c_api::NCKEY_F17);
    pub const F18: Key = Key(c_api::NCKEY_F18);
    pub const F19: Key = Key(c_api::NCKEY_F19);
    pub const F20: Key = Key(c_api::NCKEY_F20);
    pub const F21: Key = Key(c_api::NCKEY_F21);
    pub const F22: Key = Key(c_api::NCKEY_F22);
    pub const F23: Key = Key(c_api::NCKEY_F23);
    pub const F24: Key = Key(c_api::NCKEY_F24);
    pub const F25: Key = Key(c_api::NCKEY_F25);
    pub const F26: Key = Key(c_api::NCKEY_F26);
    pub const F27: Key = Key(c_api::NCKEY_F27);
    pub const F28: Key = Key(c_api::NCKEY_F28);
    pub const F29: Key = Key(c_api::NCKEY_F29);
    pub const F30: Key = Key(c_api::NCKEY_F30);
    pub const F31: Key = Key(c_api::NCKEY_F31);
    pub const F32: Key = Key(c_api::NCKEY_F32);
    pub const F33: Key = Key(c_api::NCKEY_F33);
    pub const F34: Key = Key(c_api::NCKEY_F34);
    pub const F35: Key = Key(c_api::NCKEY_F35);
    pub const F36: Key = Key(c_api::NCKEY_F36);
    pub const F37: Key = Key(c_api::NCKEY_F37);
    pub const F38: Key = Key(c_api::NCKEY_F38);
    pub const F39: Key = Key(c_api::NCKEY_F39);
    pub const F40: Key = Key(c_api::NCKEY_F40);
    pub const F41: Key = Key(c_api::NCKEY_F41);
    pub const F42: Key = Key(c_api::NCKEY_F42);
    pub const F43: Key = Key(c_api::NCKEY_F43);
    pub const F44: Key = Key(c_api::NCKEY_F44);
    pub const F45: Key = Key(c_api::NCKEY_F45);
    pub const F46: Key = Key(c_api::NCKEY_F46);
    pub const F47: Key = Key(c_api::NCKEY_F47);
    pub const F48: Key = Key(c_api::NCKEY_F48);
    pub const F49: Key = Key(c_api::NCKEY_F49);
    pub const F50: Key = Key(c_api::NCKEY_F50);
    pub const F51: Key = Key(c_api::NCKEY_F51);
    pub const F52: Key = Key(c_api::NCKEY_F52);
    pub const F53: Key = Key(c_api::NCKEY_F53);
    pub const F54: Key = Key(c_api::NCKEY_F54);
    pub const F55: Key = Key(c_api::NCKEY_F55);
    pub const F56: Key = Key(c_api::NCKEY_F56);
    pub const F57: Key = Key(c_api::NCKEY_F57);
    pub const F58: Key = Key(c_api::NCKEY_F58);
    pub const F59: Key = Key(c_api::NCKEY_F59);
    pub const F60: Key = Key(c_api::NCKEY_F60);

    // ... leave room for function keys.

    pub const Enter: Key = Key(c_api::NCKEY_ENTER);
    /// "clear-screen or erase"
    pub const Cls: Key = Key(c_api::NCKEY_CLS);
    /// down + left on keypad
    pub const DLeft: Key = Key(c_api::NCKEY_DLEFT);
    pub const DRight: Key = Key(c_api::NCKEY_DRIGHT);
    /// up + left on keypad
    pub const ULeft: Key = Key(c_api::NCKEY_ULEFT);
    pub const URight: Key = Key(c_api::NCKEY_URIGHT);
    pub const Center: Key = Key(c_api::NCKEY_CENTER);
    pub const Begin: Key = Key(c_api::NCKEY_BEGIN);
    pub const Cancel: Key = Key(c_api::NCKEY_CANCEL);
    pub const Close: Key = Key(c_api::NCKEY_CLOSE);
    pub const Command: Key = Key(c_api::NCKEY_COMMAND);
    pub const Copy: Key = Key(c_api::NCKEY_COPY);
    pub const Exit: Key = Key(c_api::NCKEY_EXIT);
    pub const Print: Key = Key(c_api::NCKEY_PRINT);
    pub const Refresh: Key = Key(c_api::NCKEY_REFRESH);

    // these keys aren't generally available outside of the kitty protocol:

    pub const CapsLock: Key = Key(c_api::NCKEY_CAPS_LOCK);
    pub const ScrollLock: Key = Key(c_api::NCKEY_SCROLL_LOCK);
    pub const NumLock: Key = Key(c_api::NCKEY_NUM_LOCK);
    pub const PrintScreen: Key = Key(c_api::NCKEY_PRINT_SCREEN);
    pub const Pause: Key = Key(c_api::NCKEY_PAUSE);
    pub const Menu: Key = Key(c_api::NCKEY_MENU);

    // media keys, similarly only available through kitty's protocol:

    pub const MediaPlay: Key = Key(c_api::NCKEY_MEDIA_PLAY);
    pub const MediaPause: Key = Key(c_api::NCKEY_MEDIA_PAUSE);
    pub const MediaPPause: Key = Key(c_api::NCKEY_MEDIA_PPAUSE);
    pub const MediaRev: Key = Key(c_api::NCKEY_MEDIA_REV);
    pub const MediaStop: Key = Key(c_api::NCKEY_MEDIA_STOP);
    pub const MediaFF: Key = Key(c_api::NCKEY_MEDIA_FF);
    pub const MediaRewind: Key = Key(c_api::NCKEY_MEDIA_REWIND);
    pub const MediaNext: Key = Key(c_api::NCKEY_MEDIA_NEXT);
    pub const MediaPrev: Key = Key(c_api::NCKEY_MEDIA_PREV);
    pub const MediaRecord: Key = Key(c_api::NCKEY_MEDIA_RECORD);
    pub const MediaLVol: Key = Key(c_api::NCKEY_MEDIA_LVOL);
    pub const MediaRVol: Key = Key(c_api::NCKEY_MEDIA_RVOL);
    pub const MediaMute: Key = Key(c_api::NCKEY_MEDIA_MUTE);

    // modifiers when pressed by themselves. this ordering comes from the Kitty
    // keyboard protocol, and mustn't be changed without updating handlers:

    pub const LShift: Key = Key(c_api::NCKEY_LSHIFT);
    pub const LCtrl: Key = Key(c_api::NCKEY_LCTRL);
    pub const LAlt: Key = Key(c_api::NCKEY_LALT);
    pub const LSuper: Key = Key(c_api::NCKEY_LSUPER);
    pub const LHyper: Key = Key(c_api::NCKEY_LHYPER);
    pub const LMeta: Key = Key(c_api::NCKEY_LMETA);
    pub const RShift: Key = Key(c_api::NCKEY_RSHIFT);
    pub const RCtrl: Key = Key(c_api::NCKEY_RCTRL);
    pub const RAlt: Key = Key(c_api::NCKEY_RALT);
    pub const RSuper: Key = Key(c_api::NCKEY_RSUPER);
    pub const RHyper: Key = Key(c_api::NCKEY_RHYPER);
    pub const RMeta: Key = Key(c_api::NCKEY_RMETA);
    /// `AltGr` in european keyboards
    pub const L3Shift: Key = Key(c_api::NCKEY_L3SHIFT);
    pub const L5Shift: Key = Key(c_api::NCKEY_L5SHIFT);

    // Mouse events. We encode which button was pressed into the number,
    // but position information is embedded in the larger ncinput event:

    pub const Motion: Key = Key(c_api::NCKEY_MOTION);
    pub const Button1: Key = Key(c_api::NCKEY_BUTTON1);
    pub const Button2: Key = Key(c_api::NCKEY_BUTTON2);
    pub const Button3: Key = Key(c_api::NCKEY_BUTTON3);
    /// scrollwheel up
    pub const Button4: Key = Key(c_api::NCKEY_BUTTON4);
    /// scrollwheel down
    pub const Button5: Key = Key(c_api::NCKEY_BUTTON5);
    pub const Button6: Key = Key(c_api::NCKEY_BUTTON6);
    pub const Button7: Key = Key(c_api::NCKEY_BUTTON7);
    pub const Button8: Key = Key(c_api::NCKEY_BUTTON8);
    pub const Button9: Key = Key(c_api::NCKEY_BUTTON9);
    pub const Button10: Key = Key(c_api::NCKEY_BUTTON10);
    pub const Button11: Key = Key(c_api::NCKEY_BUTTON11);

    /// we received SIGCONT
    pub const Signal: Key = Key(c_api::NCKEY_SIGNAL);

    /// Will be returned upon reaching the end of input.
    pub const Eof: Key = Key(c_api::NCKEY_EOF);

    // Aliases from the 128 characters common to ASCII+UTF8:
    pub const Tab: Key = Key(c_api::NCKEY_TAB);
    pub const Esc: Key = Key(c_api::NCKEY_ESC);
    pub const Space: Key = Key(c_api::NCKEY_SPACE);
}

/// # Aliases
#[allow(non_upper_case_globals)]
impl Key {
    /// Alias of [`Button4`][Key::Button4]
    pub const ScrollUp: Key = Key(c_api::NCKEY_SCROLL_UP);
    /// Alias of [`Button5`][Key::Button5]
    pub const Scrolldown: Key = Key(c_api::NCKEY_SCROLL_DOWN);
    /// Alias of [`Enter`][Key::Enter]
    pub const Return: Key = Key(c_api::NCKEY_RETURN);
}
/// # methods
impl Key {
    /// Checks whether a number falls in the range of synthesized events.
    pub fn is(num: u32) -> bool {
        NcKey::is(num)
    }

    /// Returns a new `Key` if the provided number falls in the correct range.
    pub fn new(num: u32) -> Option<Self> {
        if Self::is(num) {
            Some(Self(num))
        } else {
            None
        }
    }

    //

    /// Returns true if it's a function key event.
    pub fn is_function(&self) -> bool {
        matches!(self.0, c_api::NCKEY_F00..=c_api::NCKEY_F60)
    }

    /// Returns true if it's a multimedia key event.
    pub fn is_media(&self) -> bool {
        matches!(self.0, c_api::NCKEY_MEDIA_PLAY..=c_api::NCKEY_MEDIA_MUTE)
    }

    /// Returns true if it's a mouse event.
    pub fn is_mouse(&self) -> bool {
        matches!(self.0, c_api::NCKEY_MOTION..=c_api::NCKEY_BUTTON11)
    }

    /// Returns true if it's a resize event.
    pub fn is_resize(&self) -> bool {
        matches!(self.0, c_api::NCKEY_RESIZE)
    }

    //

    /// Returns the name of the current `Key`.
    pub fn name(&self) -> &'static str {
        Self::check_name(self.0)
    }

    /// Returns the name of the `Key` the number would be.
    pub fn check_name(num: u32) -> &'static str {
        NcKey::check_name(num)
    }
}
