use std::io::prelude::*;
use std::io::{ErrorKind as IoErrorKind, Result};
use std::time::{Duration, Instant};

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

fn emulate_nonblocking(last_io: &mut Option<Instant>) -> Result<()> {
    match *last_io {
        Some(last_io_some) => {
            if last_io_some + Duration::from_millis(DELAY) < Instant::now() {
                *last_io = None;
                return Err(IoErrorKind::WouldBlock.into());
            }
        }
        None => *last_io = Some(Instant::now()),
    }
    Ok(())
}

impl<T: Write> Write for SlowWriter<T> {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        if self.slow {
            if self.blocking {
                std::thread::sleep(Duration::from_millis(DELAY));
            } else {
                emulate_nonblocking(&mut self.last_write)?
            }
        }
        self.inner.write(data)
    }
    fn flush(&mut self) -> Result<()> {
        if self.slow {
            if self.blocking {
                std::thread::sleep(Duration::from_millis(DELAY));
            } else {
                emulate_nonblocking(&mut self.last_write)?;
            }
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
                emulate_nonblocking(&mut self.last_read)?
            }
        }
        self.inner.read(buffer)
    }
}
