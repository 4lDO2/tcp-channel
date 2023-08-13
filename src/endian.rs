use bincode::Config;
use byteorder::ByteOrder;
pub use byteorder::{BigEndian, LittleEndian, NativeEndian};

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
