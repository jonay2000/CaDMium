use pam::Authenticator;
use pam_sys::{PamReturnCode, getenv};
use std::io;
use logind_dbus::LoginManager;
use rpassword::read_password;
use std::error::Error;
use core::fmt;
use std::fmt::Debug;
use nix::unistd::{fork, ForkResult, setuid, setgid, Uid, Gid};
use std::process::Command;
use pam_sys::raw::pam_get_user;
use users::get_user_by_name;

#[derive(Debug)]
enum ErrorKind {
    InhibitationError,
    IoError,
    AuthenticationError,
    SessionError

}
impl Error for ErrorKind {}
impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <dyn Debug>::fmt(self, f)
    }
}

struct UserInfo {
    username: String,
    password: String,
}

fn simple_get_credentials() -> io::Result<UserInfo> {
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    username.truncate(username.trim_end().len());
    let password = read_password()?;

    Ok(UserInfo {
        username,
        password
    })
}

fn authenticate() -> Result<UserInfo, ErrorKind>{
    let logind_manager = LoginManager::new().expect("Could not get logind-manager");

    let mut authenticator = Authenticator::with_password("system-auth")
        .expect("Failed to init PAM client.");

    // block where we inhibit suspend
    let login_info= {
        let suspend_lock = logind_manager.connect().inhibit_suspend("LighterDM", "login").map_err(|_| ErrorKind::InhibitationError)?;

        let login_info = simple_get_credentials().map_err(|_| ErrorKind::IoError)?;

        authenticator.get_handler().set_credentials(login_info.username.clone(), login_info.password);

        match authenticator.authenticate() {
            Err(e)=>  {
                if e.to_string() == PamReturnCode::PERM_DENIED.to_string() {
                    println!("Permission denied.");
                } else if e.to_string() == PamReturnCode::AUTH_ERR.to_string() {
                    #[cfg(debug_assertions)]
                    dbg!("AUTH_ERR");

                    println!("Authentication error.");
                } else if e.to_string() == PamReturnCode::USER_UNKNOWN.to_string() {
                    #[cfg(debug_assertions)]
                    dbg!("USER_UNKNOWN");

                    println!("Authentication error.");
                } else if e.to_string() == PamReturnCode::MAXTRIES.to_string() {
                    println!("Maximum login attempts reached.");
                } else if e.to_string() == PamReturnCode::CRED_UNAVAIL.to_string() {
                    println!("Underlying authentication service can not retrieve user credentials unavailable.");
                } else if e.to_string() == PamReturnCode::ACCT_EXPIRED.to_string() {
                    println!("Account expired");
                } else if e.to_string() == PamReturnCode::CRED_EXPIRED.to_string() {
                    println!("Account  expired");
                } else if e.to_string() == PamReturnCode::TRY_AGAIN.to_string() {
                    println!("PAM fucked up, please try again");
                } else if e.to_string() == PamReturnCode::ABORT.to_string() {
                    println!("user's authentication token has expired");
                } else if e.to_string() == PamReturnCode::INCOMPLETE.to_string() {
                    println!("We fucked up, please try again");
                } else {
                    println!("A PAM error occurred: {}", e);
                }

                return Err(ErrorKind::AuthenticationError)
            }
            Ok(_) => ()
        };
        
        UserInfo{
            username: login_info.username,
            password: String::new()
        }
    };

    authenticator.open_session().map_err(|_| ErrorKind::SessionError)?;

    Ok(login_info)
}

fn main() -> io::Result<()>{

    let mut auth: Result<UserInfo, ErrorKind>;

    loop {
        auth = authenticate();

        if auth.is_ok() {
            break;
        }
    }

    // Safe because we check is_ok()
    let user_info = auth.unwrap();

    match fork() {
        Ok(ForkResult::Child) => {
            println!("Logged in as: {}", std::env::var("USER").unwrap());
            println!("Current directory: {}", std::env::var("PWD").unwrap());
            let user= get_user_by_name(&user_info.username).unwrap();

            setuid(Uid::from_raw(user.uid()));
            setgid(Gid::from_raw(user.primary_group_id()));

            //exec /bin/bash --login .xinitrc
            let mut child = Command::new("whoami").spawn().expect("failed to execute child");

            child.wait().expect("failed to wait on child");
        }
        _ => {
            loop {}
        }
    }


    // ask for user / pass
    // authenticate with pam
    // setuid to user
    // startx
    Ok(())

}
