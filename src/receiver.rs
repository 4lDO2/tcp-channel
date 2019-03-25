use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::marker::PhantomData;

use bincode::Config;
use serde::de::DeserializeOwned;

use crate::{ChannelRecv, Endian, BigEndian, RecvError};

/// The receiving side of a channel.
pub struct Receiver<T: DeserializeOwned, R: Read = BufReader<TcpStream>> {
    reader: R,
    config: Config,
    _marker: PhantomData<T>,
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
    pub fn with_endianness<F: Endian>(self) -> TypedReceiverBuilder<T, R, F> {
        TypedReceiverBuilder {
            _marker: PhantomData,
        }
    }
}
impl<T: DeserializeOwned, R: Read, E: Endian> TypedReceiverBuilder<T, R, E> {
    /// Initialize the receiver with the current variables.
    pub fn build(self, reader: R) -> Receiver<T, R> {
        Receiver {
            _marker: PhantomData,
            reader,
            config: E::config(),
        }
    }
}
impl<T: DeserializeOwned, E: Endian> TypedReceiverBuilder<T, BufReader<TcpStream>, E> {
    /// Listen for a sender, binding the listener to the specified address.
    pub fn listen_once<A: ToSocketAddrs>(self, address: A) -> std::io::Result<Receiver<T, BufReader<TcpStream>>> {
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
    pub fn listen_once<A: ToSocketAddrs>(self, address: A) -> std::io::Result<Receiver<T, TcpStream>> {
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

impl<T: DeserializeOwned, R: Read> ChannelRecv<T> for Receiver<T, R> {
    type Error = RecvError;

    fn recv(&mut self) -> Result<T, RecvError> {
        Ok(self.config.deserialize_from(&mut self.reader)?)
    }
}
