pub use byteorder::{BigEndian, LittleEndian, NativeEndian};
use byteorder::ByteOrder;
use bincode::Config;

pub trait Endian: ByteOrder {
    fn config() -> Config;
}
impl Endian for BigEndian {
    fn config() -> Config {
        let mut config = bincode::config();
        config.big_endian();
        config
    }
}
impl Endian for LittleEndian {
    fn config() -> Config {
        let mut config = bincode::config();
        config.little_endian();
        config
    }
}
