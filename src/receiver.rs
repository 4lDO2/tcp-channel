use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::marker::PhantomData;

use bincode::Config;
use byteorder::ReadBytesExt;
use serde::de::DeserializeOwned;

use crate::{ChannelRecv, Endian, BigEndian, RecvError};

pub const DEFAULT_MAX_SIZE: usize = 64 * 0x100_000;

/// The receiving side of a channel.
pub struct Receiver<T: DeserializeOwned, E: Endian, R: Read = BufReader<TcpStream>> {
    reader: R,
    config: Config,
    max_size: usize,
    _marker: PhantomData<(T, E)>,
}

/// A more convenient way of initializing receivers.
pub struct ReceiverBuilder;

pub struct TypedReceiverBuilder<T, R, E> {
    _marker: PhantomData<(T, R, E)>,
    max_size: usize,
}
impl ReceiverBuilder {
    /// Begin building a new, buffered channel.
    pub fn new() -> TypedReceiverBuilder<(), BufReader<TcpStream>, BigEndian> {
        Self::buffered()
    }
    /// Begin building a new, buffered channel.
    pub fn buffered() -> TypedReceiverBuilder<(), BufReader<TcpStream>, BigEndian> {
        TypedReceiverBuilder {
            _marker: PhantomData,
            max_size: DEFAULT_MAX_SIZE,
        }
    }
    /// Begin building a new, non-buffered channel.
    pub fn realtime() -> TypedReceiverBuilder<(), TcpStream, BigEndian> {
        TypedReceiverBuilder {
            _marker: PhantomData,
            max_size: DEFAULT_MAX_SIZE,
        }
    }
}
impl<T, R, E> TypedReceiverBuilder<T, R, E> {
    /// Specify the type to send.
    pub fn with_type<U: DeserializeOwned>(self) -> TypedReceiverBuilder<U, R, E> {
        TypedReceiverBuilder {
            _marker: PhantomData,
            max_size: self.max_size,
        }
    }
    /// Specify the underlying reader type.
    pub fn with_reader<S: Read>(self) -> TypedReceiverBuilder<T, S, E> {
        TypedReceiverBuilder {
            _marker: PhantomData,
            max_size: self.max_size,
        }
    }
    /// Specify the endianness.
    pub fn with_endianness<F: Endian>(self) -> TypedReceiverBuilder<T, R, F> {
        TypedReceiverBuilder {
            _marker: PhantomData,
            max_size: self.max_size,
        }
    }
    /// Specify the max size to be allocated when receiving.
    pub fn with_max_size(self, max_size: usize) -> Self {
        Self {
            _marker: PhantomData,
            max_size,
        }
    }
}
impl<T: DeserializeOwned, R: Read, E: Endian> TypedReceiverBuilder<T, R, E> {
    /// Initialize the receiver with the current variables.
    pub fn build(self, reader: R) -> Receiver<T, E, R> {
        Receiver {
            _marker: PhantomData,
            reader,
            config: E::config(),
            max_size: self.max_size,
        }
    }
}
impl<T: DeserializeOwned, E: Endian> TypedReceiverBuilder<T, BufReader<TcpStream>, E> {
    /// Listen for a sender, binding the listener to the specified address.
    pub fn listen_once<A: ToSocketAddrs>(self, address: A) -> std::io::Result<Receiver<T, E, BufReader<TcpStream>>> {
        let listener = TcpListener::bind(address)?;

        let (stream, _) = listener.accept()?;

        Ok(Receiver {
            config: E::config(),
            _marker: PhantomData,
            reader: BufReader::new(stream),
            max_size: self.max_size,
        })
    }
}
impl<T: DeserializeOwned, E: Endian> TypedReceiverBuilder<T, TcpStream, E> {
    /// Listen for a sender, binding the listener to the specified address.
    pub fn listen_once<A: ToSocketAddrs>(self, address: A) -> std::io::Result<Receiver<T, E, TcpStream>> {
        let listener = TcpListener::bind(address)?;

        let (stream, _) = listener.accept()?;
        stream.set_nodelay(true)?;

        Ok(Receiver {
            config: E::config(),
            _marker: PhantomData,
            reader: stream,
            max_size: self.max_size,
        })
    }
}

impl<T: DeserializeOwned, E: Endian, R: Read> ChannelRecv<T> for Receiver<T, E, R> {
    type Error = RecvError;

    fn recv(&mut self) -> Result<T, RecvError> {
        let length = self.reader.read_u64::<E>()? as usize;
        if length > self.max_size {
            return Err(RecvError::TooLarge(length))
        }
        let mut buffer = vec! [0; length];
        self.reader.read_exact(&mut buffer)?;
        Ok(self.config.deserialize(&buffer)?)
    }
}
