//! SPSC channels in Rust, transmitted through anything that implements Read and Write.
//! It uses bincode and serde for serialization and deserialization.

extern crate bincode;
extern crate byteorder;
extern crate quick_error;
extern crate serde;

mod channel;
mod endian;
mod error;
mod receiver;
mod sender;

pub use channel::{ChannelRecv, ChannelSend};
pub use endian::{Endian, BigEndian, LittleEndian, NativeEndian};
pub use error::{RecvError, SendError};
pub use receiver::{Receiver, ReceiverBuilder, DEFAULT_MAX_SIZE};
pub use sender::{Sender, SenderBuilder};
