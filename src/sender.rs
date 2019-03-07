use std::io::{BufWriter, Write};
use std::marker::PhantomData;
use std::net::{TcpStream, ToSocketAddrs};

use bincode::Config;
use byteorder::{BigEndian, WriteBytesExt};
use serde::Serialize;

use crate::{ChannelSend, Endian, SendError};

/// The sending side of a channel.
pub struct Sender<T: Serialize, E: Endian, W: Write = BufWriter<TcpStream>> {
    writer: W,
    config: Config,
    _marker: PhantomData<(T, E)>,
}

/// A more convenient way of initializing senders.
pub struct SenderBuilder<T: Serialize, W: Write, E: Endian> {
    _marker: PhantomData<(T, W, E)>,
}

impl<T: Serialize, W: Write, E: Endian> SenderBuilder<T, W, E> {
    /// Begin building a new, buffered channel.
    pub fn new() -> SenderBuilder<T, BufWriter<TcpStream>, BigEndian> {
        Self::buffered()
    }
    /// Begin building a new, buffered channel.
    pub fn buffered() -> SenderBuilder<T, BufWriter<TcpStream>, BigEndian> {
        SenderBuilder {
            _marker: PhantomData,
        }
    }
    /// Begin building a new, non-buffered channel.
    pub fn realtime() -> SenderBuilder<T, TcpStream, BigEndian> {
        SenderBuilder {
            _marker: PhantomData,
        }
    }
    /// Specify the endianness.
    ///
    /// *NOTE* This has to be either BigEndian or LittleEndian, 
    /// since bincode doesn't use Endian, but instead has big_endian() and little_endian() in
    /// its Config.
    pub fn with_endianness<F: Endian>(self) -> SenderBuilder<T, W, F> {
        SenderBuilder {
            _marker: PhantomData,
        }
    }
}
impl<T: Serialize, W: Write, E: Endian> SenderBuilder<T, W, E> {
    /// Initialize the sender with the current variables.
    pub fn build(self, writer: W) -> Sender<T, BigEndian, W> {
        Sender {
            _marker: PhantomData,
            writer,
            config: E::config(),
        }
    }
}
impl<T: Serialize, E: Endian> SenderBuilder<T, BufWriter<TcpStream>, E> {
    /// Connect to a listening receiver, at a specified address.
    pub fn connect<A: ToSocketAddrs>(self, address: A) -> std::io::Result<Sender<T, E, BufWriter<TcpStream>>> {
        let stream = TcpStream::connect(address)?;
        stream.set_nodelay(false)?;
        stream.set_nonblocking(false)?;

        Ok(Sender {
            writer: BufWriter::new(stream),
            _marker: PhantomData,
            config: E::config(),
        })
    }
}
impl<T: Serialize, E: Endian> SenderBuilder<T, TcpStream, E> {
    /// Connect to a listening receiver, at a specified address.
    pub fn connect<A: ToSocketAddrs>(self, address: A) -> std::io::Result<Sender<T, E, TcpStream>> {
        let stream = TcpStream::connect(address)?;
        stream.set_nodelay(true)?;
        stream.set_nonblocking(false)?;

        Ok(Sender {
            writer: stream,
            _marker: PhantomData,
            config: E::config(),
        })
    }
}
impl<T: Serialize, W: Write, E: Endian> ChannelSend<T> for Sender<T, E, W> {
    type Error = SendError;
    fn send(&mut self, value: &T) -> Result<(), SendError> {
        let bytes = self.config.serialize(value)?;
        self.writer.write_u64::<E>(bytes.len() as u64)?;
        self.writer.write(&bytes)?;

        Ok(())
    }
}
