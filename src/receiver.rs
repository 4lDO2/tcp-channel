use std::io::{BufReader, Read};
use std::net::TcpStream;
use std::marker::PhantomData;

use byteorder::{BigEndian, ReadBytesExt};
use serde::de::DeserializeOwned;

pub struct Receiver<T, R: Read = BufReader<TcpStream>> {
    reader: R,
    _marker: PhantomData<T>,
}

#[derive(Debug)]
pub enum RecvError {
    Disconnected,
    BincodeError(bincode::Error),
    IoError(std::io::Error),
}

impl From<bincode::Error> for RecvError {
    fn from(error: bincode::Error) -> Self {
        RecvError::BincodeError(error)
    }
}
impl From<std::io::Error> for RecvError {
    fn from(error: std::io::Error) -> Self {
        RecvError::IoError(error)
    }
}

impl<T: DeserializeOwned> Receiver<T> {
    pub fn new<R: Read>(reader: R) -> Receiver<T, R> {
        Receiver {
            reader,
            _marker: PhantomData,
        }
    }
}

impl<T: DeserializeOwned, R: Read> Receiver<T, R> {
    pub fn recv(&mut self) -> Result<T, RecvError> {
        let length = self.reader.read_u64::<BigEndian>()? as usize;

        let mut buffer = vec! [0; length];
        self.reader.read_exact(&mut buffer)?;

        Ok(bincode::deserialize(&buffer)?)
    }
}
