use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::marker::PhantomData;

use bincode::Config;
use byteorder::{BigEndian, ReadBytesExt};
use serde::de::DeserializeOwned;

use crate::{ChannelRecv, Endian, RecvError};

/// The receiving side of a channel.
pub struct Receiver<T: DeserializeOwned, E: Endian, R: Read = BufReader<TcpStream>> {
    reader: R,
    config: Config,
    _marker: PhantomData<(T, E)>,
}

/// A more convenient way of initializing receivers.
pub struct ReceiverBuilder;

pub struct TypedReceiverBuilder<T, R, E> {
    _marker: PhantomData<(T, R, E)>,
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
        }
    }
    /// Begin building a new, non-buffered channel.
    pub fn realtime() -> TypedReceiverBuilder<(), TcpStream, BigEndian> {
        TypedReceiverBuilder {
            _marker: PhantomData,
        }
    }
}
impl<T, R, E> TypedReceiverBuilder<T, R, E> {
    /// Specify the type to send.
    pub fn with_type<U: DeserializeOwned>(self) -> TypedReceiverBuilder<U, R, E> {
        TypedReceiverBuilder {
            _marker: PhantomData,
        }
    }
    /// Specify the endianness.
    ///
    /// *NOTE* This has to be either BigEndian or LittleEndian, 
    /// since bincode doesn't use Endian, but instead has big_endian() and little_endian() in
    /// its Config.
    pub fn with_endianness<F: Endian>(self) -> TypedReceiverBuilder<T, R, F> {
        TypedReceiverBuilder {
            _marker: PhantomData,
        }
    }
}
impl<T: DeserializeOwned, R: Read, E: Endian> TypedReceiverBuilder<T, R, E> {
    /// Initialize the receiver with the current variables.
    pub fn build(self, reader: R) -> Receiver<T, BigEndian, R> {
        Receiver {
            _marker: PhantomData,
            reader,
            config: E::config(),
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
        })
    }
}

impl<T: DeserializeOwned, E: Endian, R: Read> ChannelRecv<T> for Receiver<T, E, R> {
    type Error = RecvError;

    fn recv(&mut self) -> Result<T, RecvError> {
        let length = self.reader.read_u64::<E>()? as usize;

        let mut buffer = vec! [0; length];
        self.reader.read_exact(&mut buffer)?;

        Ok(self.config.deserialize(&buffer)?)
    }
}
