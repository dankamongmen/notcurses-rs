//!

mod alpha;
mod channel;
#[allow(clippy::module_inception)]
mod channels;
mod rgb;

pub use alpha::Alpha;
pub use channel::Channel;
pub use channels::Channels;
pub use rgb::Rgb;
