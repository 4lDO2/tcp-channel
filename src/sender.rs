use std::io::{BufWriter, Write};
use std::marker::PhantomData;
use std::net::{TcpStream, ToSocketAddrs};

use byteorder::{BigEndian, WriteBytesExt};
use serde::Serialize;

use crate::{ChannelSend, SendError};

pub struct Sender<T: Serialize, W: Write = BufWriter<TcpStream>> {
    writer: W,
    _marker: PhantomData<T>,
}


impl<T: Serialize> Sender<T> {
    pub fn connect<A: ToSocketAddrs>(address: A) -> std::io::Result<Self> {
        Ok(Sender::new(BufWriter::new(TcpStream::connect(address)?)))
    }
    pub fn connect_realtime<A: ToSocketAddrs>(address: A) -> std::io::Result<Sender<T, TcpStream>> {
        Ok(Sender::new(TcpStream::connect(address)?))
    }
    pub fn new<W: Write>(writer: W) -> Sender<T, W> {
        Sender {
            writer,
            _marker: PhantomData,
        }
    }
}
impl<T: Serialize, W: Write> ChannelSend<T> for Sender<T, W> {
    type Error = SendError;
    fn send(&mut self, value: &T) -> Result<(), SendError> {
        let bytes = bincode::serialize(value)?;
        self.writer.write_u64::<BigEndian>(bytes.len() as u64)?;
        self.writer.write(&bytes)?;

        Ok(())
    }
}
