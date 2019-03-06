extern crate tcp_channel;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::net::TcpListener;

mod common;
use common::ClientToServer;

use tcp_channel::Receiver;

fn main() {
    let address = std::env::args().nth(1).unwrap();
    let listener = TcpListener::bind(address).unwrap();

    while let Ok((stream, client_address)) = listener.accept() {
        println!("INFO: Started connection with {}", client_address);
        let mut receiver = Receiver::<ClientToServer>::new(stream);

        while let Ok(message) = receiver.recv() {
            println!("INFO: Received message: {:?}", message);
        }
    }
    println!("INFO: Stopped server");
}