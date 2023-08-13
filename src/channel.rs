use std::sync::mpsc::{
    Receiver as StdReceiver, RecvError as StdRecvError, SendError as StdSendError,
    Sender as StdSender,
};

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
