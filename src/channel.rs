use std::sync::mpsc::{Sender as StdSender, Receiver as StdReceiver, SendError as StdSendError, RecvError as StdRecvError};

pub trait ChannelSend<T> {
    type Error;
    fn send(&mut self, value: &T) -> Result<(), Self::Error>;
}
pub trait ChannelRecv<T> {
    type Error;
    fn recv(&mut self) -> Result<T, Self::Error>;
}

impl<T: Clone> ChannelSend<T> for StdSender<T> {
    type Error = StdSendError<T>;

    fn send(&mut self, value: &T) -> Result<(), Self::Error> {
        StdSender::send(self, value.clone())
    }
}
impl<T> ChannelRecv<T> for StdReceiver<T> {
    type Error = StdRecvError;

    fn recv(&mut self) -> Result<T, Self::Error> {
        StdReceiver::recv(self)
    }
}
