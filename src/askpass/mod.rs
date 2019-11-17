use std::error::Error;
use core::fmt;
use fmt::Debug;

pub mod simple;


#[derive(Debug)]
pub enum AskPassError {

}
impl Error for AskPassError {}
impl fmt::Display for AskPassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <dyn Debug>::fmt(self, f)
    }
}

pub struct UserInfo {
    pub username: String,
    pub password: String,
}

