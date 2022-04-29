// notcurses::cell::channel
//
//!
//

use crate::{sys::{NcChannel, NcChannels}};

///
#[derive(Clone, Copy, Debug)]
pub struct Channel {
    nc: NcChannel
}

///
#[derive(Clone, Copy, Debug)]
pub struct Channels {
    nc: NcChannels
}

mod std_impls {
    use super::*;

    impl From<Channel> for NcChannel {
        fn from(c: Channel) -> NcChannel {
            c.nc
        }
    }
    impl From<Channels> for NcChannels {
        fn from(c: Channels) -> NcChannels {
            c.nc
        }
    }

    //

    impl From<NcChannel> for Channel {
        fn from(nc: NcChannel) -> Channel {
            Self { nc }
        }
    }
    impl From<NcChannels> for Channels {
        fn from(nc: NcChannels) -> Channels {
            Self { nc }
        }
    }

}
