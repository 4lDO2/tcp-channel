extern crate tcp_channel;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::time::Duration;

use tcp_channel::Sender;

mod common;
use common::ClientToServer;

fn main() {
    let address = std::env::args().nth(1).unwrap();
    let mut sender = Sender::connect(address).unwrap();

    for i in 0..10 {
        if i % 2 == 0 {
            sender.send(&ClientToServer::Say("hi".into())).unwrap()
        } else {
            sender.send(&ClientToServer::Leave).unwrap()
        }
    }

    std::thread::sleep(Duration::from_secs(1));
}
