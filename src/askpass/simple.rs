use crate::askpass::UserInfo;
use rpassword::read_password;
use std::io;
use std::io::Write;

pub fn simple_get_credentials() -> io::Result<UserInfo> {
    println!("Login:");
    print!("username: ");
    io::stdout().flush().expect("Could not flush stdout");

    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    username.truncate(username.trim_end().len());

    print!("password (hidden): ");
    io::stdout().flush().expect("Could not flush stdout");

    let password = read_password()?;

    Ok(UserInfo { username, password })
}
