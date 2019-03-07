use bincode::Config;
use byteorder::{BigEndian, ByteOrder, LittleEndian};

/// This trait only exists because bincode wasn't compatible with byteorder.
pub(crate) trait Endian: ByteOrder {
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


