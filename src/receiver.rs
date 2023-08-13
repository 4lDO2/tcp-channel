use std::io::{BufReader, Read};
use std::marker::PhantomData;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use byteorder::{LittleEndian, ReadBytesExt};
use serde::de::DeserializeOwned;

use crate::{ChannelRecv, RecvError};

pub const DEFAULT_MAX_SIZE: usize = 64 * 0x100_000;

/// The receiving side of a channel.
pub struct Receiver<T: DeserializeOwned, R: Read = BufReader<TcpStream>> {
    reader: R,
    max_size: usize,
    _marker: PhantomData<T>,

    // This buffer is used for storing the currently read bytes in case the stream is nonblocking.
    // Otherwise, bincode would deserialize only the currently read bytes.
    buffer: Vec<u8>,

    bytes_read: usize,
    bytes_to_read: usize,
}

/// A more convenient way of initializing receivers.
pub struct ReceiverBuilder;

pub struct TypedReceiverBuilder<T, R> {
    _marker: PhantomData<(T, R)>,
    max_size: usize,
}
impl ReceiverBuilder {
    /// Begin building a new, buffered channel.
    pub fn new() -> TypedReceiverBuilder<(), BufReader<TcpStream>> {
        Self::buffered()
    }
    /// Begin building a new, buffered channel.
    pub fn buffered() -> TypedReceiverBuilder<(), BufReader<TcpStream>> {
        TypedReceiverBuilder {
            _marker: PhantomData,
            max_size: DEFAULT_MAX_SIZE,
        }
    }
    /// Begin building a new, non-buffered channel.
    pub fn realtime() -> TypedReceiverBuilder<(), TcpStream> {
        TypedReceiverBuilder {
            _marker: PhantomData,
            max_size: DEFAULT_MAX_SIZE,
        }
    }
}
impl<T, R> TypedReceiverBuilder<T, R> {
    /// Specify the type to send.
    pub fn with_type<U: DeserializeOwned>(self) -> TypedReceiverBuilder<U, R> {
        TypedReceiverBuilder {
            _marker: PhantomData,
            max_size: self.max_size,
        }
    }
    /// Specify the underlying reader type.
    pub fn with_reader<S: Read>(self) -> TypedReceiverBuilder<T, S> {
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
impl<T: DeserializeOwned, R: Read> TypedReceiverBuilder<T, R> {
    /// Initialize the receiver with the current variables.
    pub fn build(self, reader: R) -> Receiver<T, R> {
        Receiver {
            _marker: PhantomData,
            reader,
            max_size: self.max_size,
            buffer: Vec::new(),
            bytes_read: 0,
            bytes_to_read: 0,
        }
    }
}
impl<T: DeserializeOwned> TypedReceiverBuilder<T, BufReader<TcpStream>> {
    /// Listen for a sender, binding the listener to the specified address.
    pub fn listen_once<A: ToSocketAddrs>(
        self,
        address: A,
    ) -> std::io::Result<Receiver<T, BufReader<TcpStream>>> {
        let listener = TcpListener::bind(address)?;

        let (stream, _) = listener.accept()?;

        Ok(Receiver {
            _marker: PhantomData,
            reader: BufReader::new(stream),
            max_size: self.max_size,
            buffer: Vec::new(),
            bytes_read: 0,
            bytes_to_read: 0,
        })
    }
}
impl<T: DeserializeOwned> TypedReceiverBuilder<T, TcpStream> {
    /// Listen for a sender, binding the listener to the specified address.
    pub fn listen_once<A: ToSocketAddrs>(
        self,
        address: A,
    ) -> std::io::Result<Receiver<T, TcpStream>> {
        let listener = TcpListener::bind(address)?;

        let (stream, _) = listener.accept()?;

        Ok(Receiver {
            _marker: PhantomData,
            reader: stream,
            max_size: self.max_size,
            buffer: Vec::new(),
            bytes_read: 0,
            bytes_to_read: 0,
        })
    }
}

impl<T: DeserializeOwned, R: Read> ChannelRecv<T> for Receiver<T, R> {
    type Error = RecvError;

    fn recv(&mut self) -> Result<T, RecvError> {
        if self.bytes_to_read == 0 {
            let length = self.reader.read_u64::<LittleEndian>()? as usize;
            if length > self.max_size {
                return Err(RecvError::TooLarge(length));
            }

            if self.buffer.len() < length {
                self.buffer
                    .extend(std::iter::repeat(0).take(length - self.buffer.len()));
            }

            self.bytes_to_read = length;
            self.bytes_read = 0;
        }

        loop {
            match self
                .reader
                .read(&mut self.buffer[self.bytes_read..self.bytes_to_read])
            {
                Ok(size) => {
                    self.bytes_read += size;
                    if self.bytes_read >= self.bytes_to_read {
                        let length = self.bytes_to_read;
                        self.bytes_to_read = 0;
                        return Ok(bincode::deserialize(&self.buffer[0..length])?);
                    }
                }
                Err(error) => return Err(error.into()),
            }
        }
    }
}
