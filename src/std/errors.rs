extern crate std;

use crate::Error;

impl From<std::io::Error> for Error {
    fn from(_value: std::io::Error) -> Self {
        Self::Io
    }
}
