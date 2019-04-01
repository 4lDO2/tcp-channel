extern crate tcp_channel;
#[macro_use] extern crate quick_error;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::any::Any;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::thread::JoinHandle;

use rand::{FromEntropy, RngCore, rngs::SmallRng};
use tcp_channel::{SenderBuilder, ReceiverBuilder, ChannelSend, ChannelRecv, BigEndian};

// This emulates a real TCP connection.
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
        SendErr(err: std::sync::mpsc::SendError<()>) {
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

fn blob(slow: bool) -> Result<(), Error> {
    const SIZE: usize = 262_144;
    // This test generates a random 256KiB BLOB, sends it, and then receives the BLOB, where every byte is
    // added by 1.
    
    let (sender, receiver) = std::sync::mpsc::channel();

    let thread: JoinHandle<Result<(), Error>> = std::thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:8888")?;
        sender.send(())?;
        let (stream, _) = listener.accept()?;

        let mut receiver = ReceiverBuilder::buffered()
            .with_type::<Request>()
            .with_endianness::<BigEndian>()
            .with_reader::<BufReader<SlowReader<TcpStream>>>()
            .build(BufReader::new(SlowReader::new(stream.try_clone()?, slow)));

        let mut sender = SenderBuilder::buffered()
            .with_type::<Response>()
            .with_endianness::<BigEndian>()
            .with_writer::<BufWriter<SlowWriter<TcpStream>>>()
            .build(BufWriter::new(SlowWriter::new(stream, slow)));

        while let Ok(command) = receiver.recv() {
            match command {
                Request::SendBlob(mut blob) => {
                    for byte in blob.iter_mut() {
                        *byte = byte.wrapping_add(1)
                    }
                    sender.send(&Response::Respond(blob))?;
                    sender.flush()?;
                },
                Request::Stop => return Ok(())
            }
        }

        Ok(())
    });
    receiver.recv()?;
    let stream = TcpStream::connect("127.0.0.1:8888")?;
    let mut sender = SenderBuilder::realtime()
        .with_type::<Request>()
        .with_writer::<SlowWriter<TcpStream>>()
        .with_endianness::<BigEndian>()
        .build(SlowWriter::new(stream.try_clone()?, slow));

    let mut receiver = ReceiverBuilder::buffered()
        .with_type::<Response>()
        .with_reader::<BufReader<SlowReader<TcpStream>>>()
        .with_endianness::<BigEndian>()
        .build(BufReader::new(SlowReader::new(stream, slow)));

    let blob = {
        let mut blob = vec! [0u8; SIZE];

        SmallRng::from_entropy().fill_bytes(&mut blob);

        blob.into_boxed_slice()
    };

    sender.send(&Request::SendBlob(blob.clone()))?;
    sender.flush()?;

    let new_blob = match receiver.recv()? {
        Response::Respond(new_blob) => new_blob,
    };
    let precalculated_new_blob = blob.into_iter()
        .map(|byte| byte.wrapping_add(1))
        .collect::<Box<[u8]>>();

    assert_ne!(blob, new_blob);
    assert_eq!(new_blob, precalculated_new_blob);

    sender.send(&Request::Stop)?;
    sender.flush()?;

    thread.join()??;

    Ok(())
}
#[test]
fn fast_blob() -> Result<(), Error> {
    blob(false)
}
#[test]
fn slow_blob() -> Result<(), Error> {
    blob(true)
}
