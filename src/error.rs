use bincode::Error as BincodeError;
use std::io::Error as IoError;

use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum RecvError {
        Disconnected {}
        BincodeError(err: BincodeError) {
            from()
        }
        IoError(err: IoError) {
            from()
        }
        TooLarge(size: usize) {}
    }
}
quick_error! {
    #[derive(Debug)]
    pub enum SendError {
        Disconnected {}
        BincodeError(err: BincodeError) {
            from()
        }
        IoError(err: IoError) {
            from()
        }
    }
}
