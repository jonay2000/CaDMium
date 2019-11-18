use crate::askpass::UserInfo;
use crate::error::ErrorKind;
use pam_sys::PamReturnCode;
use crate::askpass::simple::simple_get_credentials;
use logind_dbus::LoginManager;
use pam::Authenticator;
use users::get_user_by_name;
use std::env;

fn xdg(tty: u32, _uid: u32) {
//    let user = format!("/run/user/{}", uid);
//    env::set_var("XDG_RUNTIME_DIR", format!("/run/user/{}", uid));

    env::set_var("XDG_SESSION_CLASS", "greeter");

    //TODO: should be seat{display}. might need to move to a place where we actually know the display.
    env::set_var("XDG_SEAT", "seat0");

    env::set_var("XDG_VTNR", format!("{}", tty));
    env::set_var("XDG_SESSION_ID", "1");

    // temp
//    env::set_var("DBUS_SESSION_BUS_ADDRESS", format!("unix:path=/run/user/{}/bus", uid));

    env::set_var("XDG_SESSION_TYPE", "tty");
}

pub fn authenticate(tty: u32) -> Result<(UserInfo, LoginManager), ErrorKind>{
    let logind_manager = LoginManager::new().expect("Could not get logind-manager");

    let mut authenticator = Authenticator::with_password("login")
        .expect("Failed to init PAM client.");

    // block where we inhibit suspend
    let login_info= {
        let _suspend_lock = logind_manager.connect()
            .inhibit_suspend("Cadmium", "login")
            .map_err(|_| ErrorKind::InhibitationError)?;

        // TODO: change to generic get credentials
        let login_info = simple_get_credentials().map_err(|_| ErrorKind::IoError)?;

        let user= get_user_by_name(&login_info.username).expect("Couldn't find username");
        xdg(tty as u32, user.uid());

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

//        logind_manager.register().map_err(|_| ErrorKind::DBusError)?;

        (
            UserInfo{
                username: login_info.username,
                password: String::new()
            },
            logind_manager
        )
    };

    authenticator.open_session().map_err(|_| ErrorKind::SessionError)?;

    Ok(login_info)
}
