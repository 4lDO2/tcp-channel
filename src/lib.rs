//! SPSC channels in Rust, transmitted through anything that implements Read.
//! It uses bincode and serde for serialization and deserialization.

extern crate bincode;
extern crate byteorder;
#[macro_use] extern crate quick_error;
extern crate serde;

mod channel;
mod endian;
mod error;
mod receiver;
mod sender;

pub use channel::{ChannelRecv, ChannelSend};
pub use endian::Endian;
pub use error::{RecvError, SendError};
pub use receiver::{Receiver, ReceiverBuilder};
pub use sender::{Sender, SenderBuilder};

pub use byteorder::{BigEndian, LittleEndian};
