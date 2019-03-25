use bincode::Config;

pub enum BigEndian {}
pub enum LittleEndian {}
pub enum NativeEndian {}

pub trait Endian {
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

impl Endian for NativeEndian {
    fn config() -> Config {
        let mut config = bincode::config();
        config.native_endian();
        config
    }
}
