extern crate tcp_channel;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::net::TcpStream;
use std::time::Duration;

use tcp_channel::{ChannelSend, Sender, ChannelRecv, Receiver};

mod common;
use common::{ClientToServer, ServerToClient};

fn main() {
    let address = std::env::args().nth(1).unwrap();
    let stream = TcpStream::connect(address).unwrap();

    let mut sender = Sender::new(stream.try_clone().unwrap());
    let mut receiver = Receiver::<ServerToClient>::new(stream);

    fn message(index: u8) -> ClientToServer {
        if index % 2 == 0 {
            ClientToServer::Say("Hello, world!".into())
        } else {
            ClientToServer::Leave
        }
    }

    for i in 0..4 {
        sender.send(&message(i)).unwrap();
        println!("Server: {:?}", receiver.recv().unwrap());
    }
}
