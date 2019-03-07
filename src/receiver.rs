use std::io::{BufReader, Read};
use std::net::TcpStream;
use std::marker::PhantomData;

use byteorder::{BigEndian, ReadBytesExt};
use serde::de::DeserializeOwned;

use crate::{ChannelRecv, RecvError};

pub struct Receiver<T, R: Read = BufReader<TcpStream>> {
    reader: R,
    _marker: PhantomData<T>,
}


impl<T: DeserializeOwned> Receiver<T> {
    pub fn new<R: Read>(reader: R) -> Receiver<T, R> {
        Receiver {
            reader,
            _marker: PhantomData,
        }
    }
}

impl<T: DeserializeOwned, R: Read> ChannelRecv<T> for Receiver<T, R> {
    type Error = RecvError;

    fn recv(&mut self) -> Result<T, RecvError> {
        let length = self.reader.read_u64::<BigEndian>()? as usize;

        let mut buffer = vec! [0; length];
        self.reader.read_exact(&mut buffer)?;

        Ok(bincode::deserialize(&buffer)?)
    }
}
