extern crate tcp_channel;
#[macro_use]
extern crate quick_error;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::any::Any;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread::JoinHandle;
// Yeah, regular channels are used to tell the client when the server has started!
use std::time::{Duration, SystemTime};

use tcp_channel::{BigEndian, ChannelRecv, ChannelSend, ReceiverBuilder, SenderBuilder};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Request {
    RequestTime,
    SetTime(SystemTime),
    Stop,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
enum Response {
    Respond(SystemTime),
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
        SendErr2(err: tcp_channel::SendError) {
            from()
        }
        RecvErr2(err: tcp_channel::RecvError) {
            from()
        }
        JoinErr(err: Box<Any + Send + 'static>) {
            from()
        }
    }
}

#[test]
fn time() -> Result<(), Error> {
    // This test sets up a simple time server, enum based.
    let initial_time = SystemTime::now();

    let (sender, receiver) = std::sync::mpsc::channel();

    let thread: JoinHandle<Result<(), Error>> = std::thread::spawn(move || {
        let mut time = initial_time;
        let listener = TcpListener::bind("127.0.0.1:8888")?;
        sender.send(())?;
        let (stream, _) = listener.accept()?;

        let mut receiver = ReceiverBuilder::buffered()
            .with_type::<Request>()
            .with_endianness::<BigEndian>()
            .build(BufReader::new(stream.try_clone()?));

        let mut sender = SenderBuilder::realtime()
            .with_type::<Response>()
            .with_endianness::<BigEndian>()
            .build(stream);

        while let Ok(command) = receiver.recv() {
            match command {
                Request::RequestTime => sender.send(&Response::Respond(time))?,
                Request::SetTime(new_time) => time = new_time,
                Request::Stop => return Ok(()),
            }
        }

        Ok(())
    });
    receiver.recv()?;
    let stream = TcpStream::connect("127.0.0.1:8888")?;
    let mut sender = SenderBuilder::realtime()
        .with_type::<Request>()
        .with_endianness::<BigEndian>()
        .build(stream.try_clone()?);

    let mut receiver = ReceiverBuilder::buffered()
        .with_type::<Response>()
        .with_endianness::<BigEndian>()
        .build(BufReader::new(stream));

    sender.send(&Request::RequestTime)?;
    assert_eq!(receiver.recv()?, Response::Respond(initial_time));
    let new_time = initial_time + Duration::from_secs(42);

    sender.send(&Request::SetTime(new_time))?;
    sender.send(&Request::RequestTime)?;
    assert_eq!(receiver.recv()?, Response::Respond(new_time));

    sender.send(&Request::Stop)?;

    thread.join()??;

    Ok(())
}
