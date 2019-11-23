use crate::askpass::AskPassError;
use crate::x::XError;
use core::fmt;
use fmt::Debug;
use std::error::Error;
use toml::de::Error as TomlError;

#[derive(Debug)]
pub enum ErrorKind {
    InhibitationError,
    IoError,
    AuthenticationError,
    DBusError,
    SessionError,
    ForkFailed,
    ConfigLoadError(TomlError),
    AskPassError(AskPassError),
    XError(XError),
}
impl Error for ErrorKind {}
impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <dyn Debug>::fmt(self, f)
    }
}
