extern crate tcp_channel;
#[macro_use]
extern crate quick_error;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::any::Any;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, ErrorKind as IoErrorKind};
use std::net::{TcpListener, TcpStream};
use std::thread::JoinHandle;

use rand::{rngs::SmallRng, FromEntropy, RngCore};
use serde::de::DeserializeOwned;
use tcp_channel::{
    BigEndian, ChannelRecv, ChannelSend, Receiver as TcpReceiver, ReceiverBuilder, RecvError,
    SenderBuilder, DEFAULT_MAX_SIZE,
};

// This emulates a real delayed TCP connection.
mod slow_io;
use slow_io::{SlowReader, SlowWriter};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Request {
    SendBlob(Box<[u8]>),
    Stop,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
enum Response {
    Respond(Box<[u8]>),
}

quick_error! {
    #[derive(Debug)]
    enum Error {
        Io(err: std::io::Error) {
            from()
        }
        SendErr(err: std::sync::mpsc::SendError<u16>) {
            from()
        }
        RecvErr(err: std::sync::mpsc::RecvError) {
            from()
        }
        TcpSendErr(err: tcp_channel::SendError) {
            from()
        }
        TcpRecvErr(err: tcp_channel::RecvError) {
            from()
        }
        JoinErr(err: Box<Any + Send + 'static>) {
            from()
        }
    }
}

fn pretend_blocking_read<T: DeserializeOwned, R: Read>(
    receiver: &mut TcpReceiver<T, BigEndian, R>,
) -> Result<T, RecvError> {
    loop {
        match receiver.recv() {
            Ok(value) => return Ok(value),
            Err(RecvError::IoError(ioerror)) => match ioerror.kind() {
                IoErrorKind::WouldBlock => continue,
                _ => return Err(RecvError::IoError(ioerror).into()),
            },
            Err(error) => return Err(error.into()),
        }
    }
}
fn blob(slow: bool, blocking: bool, max_size: usize) -> Result<(), Error> {
    const SIZE: usize = 262_144;
    // This test generates a random 256KiB BLOB, sends it, and then receives the BLOB, where every byte is
    // added by 1.

    let (sender, receiver) = std::sync::mpsc::channel();

    let thread: JoinHandle<Result<(), Error>> = std::thread::spawn(move || {
        let mut port = 2000;
        let listener = loop {
            match TcpListener::bind(format!("127.0.0.1:{}", port)) {
                Ok(listener) => break Ok(listener),
                Err(ioerror) => {
                    if let IoErrorKind::AddrInUse = ioerror.kind() {
                        port += 1;
                        if port >= 8000 {
                            break Err(ioerror);
                        }
                        continue;
                    } else {
                        break Err(ioerror);
                    }
                }
            }
        }
        .unwrap();

        sender.send(port).unwrap();
        let (stream, _) = listener.accept().unwrap();

        let mut receiver = ReceiverBuilder::buffered()
            .with_type::<Request>()
            .with_endianness::<BigEndian>()
            .with_reader::<BufReader<SlowReader<TcpStream>>>()
            .with_max_size(max_size)
            .build(BufReader::new(SlowReader::new(
                stream.try_clone().unwrap(),
                slow,
                blocking,
            )));

        let mut sender = SenderBuilder::buffered()
            .with_type::<Response>()
            .with_endianness::<BigEndian>()
            .with_writer::<BufWriter<SlowWriter<TcpStream>>>()
            .build(BufWriter::new(SlowWriter::new(stream, slow, true)));

        while let Ok(command) = pretend_blocking_read(&mut receiver) {
            match command {
                Request::SendBlob(mut blob) => {
                    for byte in blob.iter_mut() {
                        *byte = byte.wrapping_add(1)
                    }
                    sender.send(&Response::Respond(blob)).unwrap();
                    sender.flush().unwrap();
                }
                Request::Stop => return Ok(()),
            }
        }

        Ok(())
    });
    let port = receiver.recv().unwrap();
    let stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
    let mut sender = SenderBuilder::realtime()
        .with_type::<Request>()
        .with_writer::<SlowWriter<TcpStream>>()
        .with_endianness::<BigEndian>()
        .build(SlowWriter::new(stream.try_clone().unwrap(), slow, true));

    let mut receiver = ReceiverBuilder::buffered()
        .with_type::<Response>()
        .with_reader::<BufReader<SlowReader<TcpStream>>>()
        .with_endianness::<BigEndian>()
        .with_max_size(max_size)
        .build(BufReader::new(SlowReader::new(stream, slow, blocking)));

    let blob = {
        let mut blob = vec![0u8; SIZE];

        SmallRng::from_entropy().fill_bytes(&mut blob);

        blob.into_boxed_slice()
    };

    sender.send(&Request::SendBlob(blob.clone())).unwrap();
    sender.flush().unwrap();

    let new_blob = match pretend_blocking_read(&mut receiver).unwrap() {
        Response::Respond(blob) => blob,
    };
    let precalculated_new_blob = blob
        .into_iter()
        .map(|byte| byte.wrapping_add(1))
        .collect::<Box<[u8]>>();

    assert_ne!(blob, new_blob);
    assert_eq!(new_blob, precalculated_new_blob);
    println!("Asserted");

    sender.send(&Request::Stop).unwrap();
    sender.flush().unwrap();

    thread.join().unwrap().unwrap();

    Ok(())
}
#[test]
fn fast_blob() -> Result<(), Error> {
    blob(false, true, DEFAULT_MAX_SIZE)
}
#[test]
fn slow_blob() -> Result<(), Error> {
    blob(true, true, DEFAULT_MAX_SIZE)
}
#[test]
fn slow_nonblocking_blob() -> Result<(), Error> {
    blob(true, false, DEFAULT_MAX_SIZE)
}
#[should_panic]
#[test]
fn fast_blob_too_small() {
    blob(true, true, 1024).unwrap()
}
