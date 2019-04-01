use std::io::{Read, Result, Write};
use std::time::Duration;

// In milliseconds.
const DELAY: u64 = 200;

pub struct SlowWriter<T: Write> {
    inner: T,
    slow: bool,
}
impl<T: Write> SlowWriter<T> {
    pub fn new(inner: T, slow: bool) -> Self {
        Self {
            inner,
            slow,
        }
    }
}
impl<T: Write> Write for SlowWriter<T> {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        if self.slow {
            std::thread::sleep(Duration::from_millis(DELAY));
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
impl<T: Write> From<T> for SlowWriter<T> {
    fn from(inner: T) -> Self {
        Self {
            inner,
            slow: false,
        }
    }
}
pub struct SlowReader<T: Read> {
    inner: T,
    slow: bool,
}
impl<T: Read> SlowReader<T> {
    pub fn new(inner: T, slow: bool) -> Self {
        Self {
            inner,
            slow,
        }
    }
}
impl<T: Read> Read for SlowReader<T> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        if self.slow {
            std::thread::sleep(Duration::from_millis(DELAY));
        }
        self.inner.read(buffer)
    }
}
impl<T: Read> From<T> for SlowReader<T> {
    fn from(inner: T) -> Self {
        Self {
            inner,
            slow: false,
        }
    }
}
