use std::io::{Read, Result, Write, ErrorKind as IoErrorKind};
use std::time::{Instant, Duration};

// In milliseconds.
const DELAY: u64 = 200;

pub struct SlowWriter<T: Write> {
    inner: T,
    slow: bool,
    blocking: bool,
    last_write: Option<Instant>,
}
impl<T: Write> SlowWriter<T> {
    pub fn new(inner: T, slow: bool, blocking: bool) -> Self {
        Self {
            inner,
            slow,
            blocking,
            last_write: None,
        }
    }
}
impl<T: Write> Write for SlowWriter<T> {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        if self.slow {
            if self.blocking {
                std::thread::sleep(Duration::from_millis(DELAY));
            } else {
                match self.last_write {
                    Some(last_write) => if last_write + Duration::from_millis(DELAY) > Instant::now() {
                    } else {
                        return Err(IoErrorKind::WouldBlock.into())
                    },
                    None => self.last_write = Some(Instant::now()),
                }
            }
        }
        self.inner.write(data)
    }
    fn flush(&mut self) -> Result<()> {
        if self.slow {
            std::thread::sleep(Duration::from_millis(DELAY));
        }
        self.inner.flush()
    }
}
pub struct SlowReader<T: Read> {
    inner: T,
    slow: bool,
    blocking: bool,
    last_read: Option<Instant>,
}
impl<T: Read> SlowReader<T> {
    pub fn new(inner: T, slow: bool, blocking: bool) -> Self {
        Self {
            inner,
            slow,
            blocking,
            last_read: None,
        }
    }
}
impl<T: Read> Read for SlowReader<T> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        if self.slow {
            if self.blocking {
                std::thread::sleep(Duration::from_millis(DELAY));
            } else {
                match self.last_read {
                    Some(last_write) => if last_write + Duration::from_millis(DELAY) > Instant::now() {
                    } else {
                        return Err(IoErrorKind::WouldBlock.into())
                    },
                    None => self.last_read = Some(Instant::now()),
                }
            }
        }
        self.inner.read(buffer)
    }
}
