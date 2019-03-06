pub trait ChannelSend<T> {
    type Error;
    fn send(&mut self, value: &T) -> Result<(), Self::Error>;
}
pub trait ChannelRecv<T> {
    type Error;
    fn recv(&mut self) -> Result<T, Self::Error>;
}
