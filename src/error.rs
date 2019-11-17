use core::fmt;
use std::error::Error;
use crate::askpass::AskPassError;
use fmt::Debug;

#[derive(Debug)]
pub enum ErrorKind {
    InhibitationError,
    IoError,
    AuthenticationError,
    SessionError,
    AskPassError(AskPassError)

}
impl Error for ErrorKind {}
impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <dyn Debug>::fmt(self, f)
    }
}

