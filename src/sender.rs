use std::io::{BufWriter, Write};
use std::marker::PhantomData;
use std::net::{TcpStream, ToSocketAddrs};

use bincode::Config;
use byteorder::WriteBytesExt;
use serde::Serialize;

use crate::{ChannelSend, Endian, BigEndian, SendError};

/// The sending side of a channel.
pub struct Sender<T: Serialize, E: Endian, W: Write = BufWriter<TcpStream>> {
    writer: W,
    config: Config,
    _marker: PhantomData<(T, E)>,
}

/// A more convenient way of initializing senders.
pub struct SenderBuilder;

pub struct TypedSenderBuilder<T, W, E> {
    _marker: PhantomData<(T, W, E)>,
}

impl SenderBuilder {
    /// Begin building a new, buffered channel.
    pub fn new() -> TypedSenderBuilder<(), BufWriter<TcpStream>, BigEndian> {
        Self::buffered()
    }
    /// Begin building a new, buffered channel.
    pub fn buffered() -> TypedSenderBuilder<(), BufWriter<TcpStream>, BigEndian> {
        TypedSenderBuilder {
            _marker: PhantomData,
        }
    }
    /// Begin building a new, non-buffered channel.
    pub fn realtime() -> TypedSenderBuilder<(), TcpStream, BigEndian> {
        TypedSenderBuilder {
            _marker: PhantomData,
        }
    }
}
impl<T, W, E> TypedSenderBuilder<T, W, E> {
    /// Specify the type to send.
    pub fn with_type<U: Serialize>(self) -> TypedSenderBuilder<U, W, E> {
        TypedSenderBuilder {
            _marker: PhantomData,
        }
    }
    /// Specify the underlying writer type.
    pub fn with_writer<X: Write>(self) -> TypedSenderBuilder<T, X, E> {
        TypedSenderBuilder {
            _marker: PhantomData,
        }
    }
    /// Specify the endianness.
    pub fn with_endianness<F: Endian>(self) -> TypedSenderBuilder<T, W, F> {
        TypedSenderBuilder {
            _marker: PhantomData,
        }
    }
}
impl<T: Serialize, W: Write, E: Endian> TypedSenderBuilder<T, W, E> {
    /// Initialize the sender with the current variables.
    pub fn build(self, writer: W) -> Sender<T, E, W> {
        Sender {
            _marker: PhantomData,
            writer,
            config: E::config(),
        }
    }
}
impl<T: Serialize, E: Endian> TypedSenderBuilder<T, BufWriter<TcpStream>, E> {
    /// Connect to a listening receiver, at a specified address.
    pub fn connect<A: ToSocketAddrs>(self, address: A) -> std::io::Result<Sender<T, E, BufWriter<TcpStream>>> {
        let stream = TcpStream::connect(address)?;

        Ok(Sender {
            writer: BufWriter::new(stream),
            _marker: PhantomData,
            config: E::config(),
        })
    }
}
impl<T: Serialize, E: Endian> TypedSenderBuilder<T, TcpStream, E> {
    /// Connect to a listening receiver, at a specified address.
    pub fn connect<A: ToSocketAddrs>(self, address: A) -> std::io::Result<Sender<T, E, TcpStream>> {
        let stream = TcpStream::connect(address)?;
        stream.set_nodelay(true)?;

        Ok(Sender {
            writer: stream,
            _marker: PhantomData,
            config: E::config(),
        })
    }
}
impl<T: Serialize, E: Endian, W: Write> Sender<T, E, W> {
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
impl<T: Serialize, E: Endian, W: Write> ChannelSend<T> for Sender<T, E, W> {
    type Error = SendError;
    fn send(&mut self, value: &T) -> Result<(), SendError> {
        let buffer = self.config.serialize(value)?;
        self.writer.write_u64::<E>(buffer.len() as u64)?;
        self.writer.write(&buffer)?;
        Ok(())
    }
}
