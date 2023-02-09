// notcurses::color
//
//!
//

mod alpha;
mod channel;
mod channels;
mod palette;
mod rgb;

pub use self::rgb::{Rgb, Rgba};
pub use alpha::Alpha;
pub use channel::Channel;
pub use channels::Channels;
pub use palette::Palette;
