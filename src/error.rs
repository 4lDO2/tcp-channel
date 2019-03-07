use bincode::Error as BincodeError;
use std::io::Error as IoError;

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
