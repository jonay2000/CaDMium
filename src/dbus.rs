use std::env;
use std::process::Command;

pub fn start_dbus() {
    let dbus_output = Command::new("/usr/bin/dbus-launch")
        .output()
        .expect("Couldn't start DBus");
    let results = String::from_utf8_lossy(&dbus_output.stdout);
    for variable in results.lines() {
        let line: Vec<&str> = variable.splitn(2, '=').collect();

        env::set_var(
            line.get(0).expect("Couldn't read dbus-launch return value"),
            line.get(1).expect("Couldn't read dbus-launch return value"),
        );
    }
}
