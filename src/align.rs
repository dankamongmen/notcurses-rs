use crate::sys;

bitflags! {
    /// The alignment within a plane or terminal.
    /// Either left/right-justified, centered, or unaligned.
    pub struct Align: u32 {
        const LEFT = sys::NCALIGN_LEFT;
        const RIGHT = sys::NCALIGN_RIGHT;
        const CENTER = sys::NCALIGN_CENTER;
        const UNALIGNED = sys::NCALIGN_UNALIGNED;
    }
}
