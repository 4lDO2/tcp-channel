use std::io::{BufWriter, Write};
use std::marker::PhantomData;
use std::net::{TcpStream, ToSocketAddrs};

use byteorder::{LittleEndian, WriteBytesExt};
use serde::Serialize;

use crate::{ChannelSend, SendError};

/// The sending side of a channel.
pub struct Sender<T: Serialize, W: Write = BufWriter<TcpStream>> {
    writer: W,
    _marker: PhantomData<T>,
}

/// A more convenient way of initializing senders.
pub struct SenderBuilder;

impl SenderBuilder {
    /// Begin building a new, buffered channel.
    pub fn build() -> TypedSenderBuilder<(), BufWriter<TcpStream>> {
        Self::buffered()
    }
    /// Begin building a new, buffered channel.
    pub fn buffered() -> TypedSenderBuilder<(), BufWriter<TcpStream>> {
        TypedSenderBuilder {
            _marker: PhantomData,
        }
    }
    /// Begin building a new, non-buffered channel.
    pub fn realtime() -> TypedSenderBuilder<(), TcpStream> {
        TypedSenderBuilder {
            _marker: PhantomData,
        }
    }
}

pub struct TypedSenderBuilder<T, W> {
    _marker: PhantomData<(T, W)>,
}

impl<T, W> TypedSenderBuilder<T, W> {
    /// Specify the type to send.
    pub fn with_type<U: Serialize>(self) -> TypedSenderBuilder<U, W> {
        TypedSenderBuilder {
            _marker: PhantomData,
        }
    }
    /// Specify the underlying writer type.
    pub fn with_writer<X: Write>(self) -> TypedSenderBuilder<T, X> {
        TypedSenderBuilder {
            _marker: PhantomData,
        }
    }
}

impl<T: Serialize, W: Write> TypedSenderBuilder<T, W> {
    /// Initialize the sender with the current variables.
    pub fn build(self, writer: W) -> Sender<T, W> {
        Sender {
            _marker: PhantomData,
            writer,
        }
    }
}

impl<T: Serialize> TypedSenderBuilder<T, BufWriter<TcpStream>> {
    /// Connect to a listening receiver, at a specified address.
    pub fn connect<A: ToSocketAddrs>(
        self,
        address: A,
    ) -> std::io::Result<Sender<T, BufWriter<TcpStream>>> {
        let stream = TcpStream::connect(address)?;

        Ok(Sender {
            writer: BufWriter::new(stream),
            _marker: PhantomData,
        })
    }
}

impl<T: Serialize> TypedSenderBuilder<T, TcpStream> {
    /// Connect to a listening receiver, at a specified address.
    pub fn connect<A: ToSocketAddrs>(self, address: A) -> std::io::Result<Sender<T, TcpStream>> {
        let stream = TcpStream::connect(address)?;
        stream.set_nodelay(true)?;

        Ok(Sender {
            writer: stream,
            _marker: PhantomData,
        })
    }
}

impl<T: Serialize, W: Write> Sender<T, W> {
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<T: Serialize, W: Write> ChannelSend<T> for Sender<T, W> {
    type Error = SendError;
    fn send(&mut self, value: &T) -> Result<(), SendError> {
        let buffer = bincode::serialize(value)?;
        self.writer.write_u64::<LittleEndian>(buffer.len() as u64)?;
        self.writer.write_all(&buffer)?;
        Ok(())
    }
}
