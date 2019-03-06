//! SPSC channels in Rust, transmitted through anything that implements Read.
//! It uses bincode and serde for serialization and deserialization.

extern crate bincode;
extern crate byteorder;
extern crate serde;

mod sender;
mod receiver;
mod channel;

pub use sender::{Sender, SendError};
pub use receiver::{Receiver, RecvError};
pub use channel::{ChannelRecv, ChannelSend};
