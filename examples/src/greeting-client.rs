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

    sender.send(&ClientToServer::Say("Hello!".into())).unwrap();
    
    // TODO: No current way to sync.
    std::thread::sleep(Duration::from_secs(1));
}
