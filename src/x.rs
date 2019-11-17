use std::{env, fmt};
use std::path::Path;
use std::fs::File;
use std::fmt::Debug;
use std::error::Error;
use rand::Rng;
use std::process::{Command, Child};
use xcb::{Connection, ConnError};
use nix::sys::signal::kill;
use nix::unistd::Pid;
use nix::sys::ptrace::cont;
use nix::errno::Errno;

#[derive(Debug)]
pub enum XError {
    IOError,
    XAuthError,
    NoFreeDisplayError,
    XStartError,
    DEStartError,
    XCBConnectionError,
    NoSHELLError
}

impl Error for XError {}
impl fmt::Display for XError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <dyn Debug>::fmt(self, f)
    }
}


pub fn mcookie() -> String{
    let mut rng = rand::thread_rng();

    let cookie: u128 = rng.gen();
    format!("{:032x}", cookie)
}

/// Loops through all displays and finds the first free one.
fn get_free_display() -> Result<i32, XError>{
    for i in 0..200 {
        if !Path::new(&format!("/tmp/.X{}-lock", i)).exists() {
            return Ok(i);
        }
    }

    Err(XError::NoFreeDisplayError)
}

/// Create our auth file (.cdxauth).
fn xauth(display: &String, home: &Path) -> Result<(), XError> {
    let xauth_path = home.join(".cdxauth");

    // set the XAUTHORITY environment variable
    env::set_var("XAUTHORITY", &xauth_path);

    File::create(xauth_path).map_err(|_| XError::IOError)?;
    
    // use `xauth` to generate the xauthority file for us
    Command::new("/usr/bin/xauth")
        .args(&["add", display, ".", &mcookie()])
        .output().map_err(|_| XError::XAuthError)?;

    Ok(())
}


pub fn start_x(tty: u32, home: &Path, de: &str) -> Result<(), XError> {
    let display = format!(":{}", get_free_display()?);
    // set the DISPLAY environment variable
    env::set_var("DISPLAY", &display);

    xauth(&display, home)?;

    let xorg_process = Command::new("/usr/bin/X")
        .args(&[&display, &format!("vt{}", tty)])
        .spawn().map_err(|_| XError::XStartError)?;

    // Wait for the process to start running
    loop {
        if let Err(e) = kill(Pid::from_raw(xorg_process.id() as i32), None) {
            match e.as_errno() {
                Some(e) => match e {
                    Errno::ESRCH => {
                        continue;
                    }
                    _ => return Err(XError::XCBConnectionError)
                }
                None => return Err(XError::XCBConnectionError)
            }
        };
    }

    let c = match Connection::connect(Some(&display)){
        Ok(c) => c,
        Err(e) => {
//            match e {
//                ConnError::Connection => continue,
//                _ => return Err(XError::XCBConnectionError)
//            }
            return Err(XError::XCBConnectionError)
        }
    };

    let c = Connection::connect(None).map_err(|_| XError::XCBConnectionError)?;

    let mut de_process = Command::new(env::var("SHELL").map_err(|_| XError::NoSHELLError)?)
        .arg("-c").arg(include_str!("../res/xsetup.sh")).arg(de).spawn().map_err(|_| XError::DEStartError)?;
    
    de_process.wait();

    Ok(())
}


#[cfg(test)]
mod test {
    use crate::x::mcookie;

    #[test]
    fn test_mcookie_length() {
        assert_eq!(mcookie().len(), 32)
    }

    #[test]
    fn test_mcookie_same() {
        assert_ne!(mcookie(), mcookie());
    }
}
