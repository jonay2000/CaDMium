use crate::askpass::UserInfo;
use crate::error::ErrorKind;
use pam_sys::PamReturnCode;
use crate::askpass::simple::simple_get_credentials;
use logind_dbus::LoginManager;
use pam::Authenticator;

pub fn authenticate() -> Result<UserInfo, ErrorKind>{
    let logind_manager = LoginManager::new().expect("Could not get logind-manager");

    let mut authenticator = Authenticator::with_password("system-auth")
        .expect("Failed to init PAM client.");

    // block where we inhibit suspend
    let login_info= {
        let _suspend_lock = logind_manager.connect().inhibit_suspend("LighterDM", "login").map_err(|_| ErrorKind::InhibitationError)?;

        // TODO: change to generic get credentials
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
