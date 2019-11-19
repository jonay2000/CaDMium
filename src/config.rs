extern crate config;
extern crate glob;

use config::{Config, Environment, File};
use glob::glob;
use std::collections::HashMap;

pub fn get_config() -> HashMap<String, String> {
    let etc = File::with_name("/etc/cadmium.toml");

    let mut settings = Config::default();
    settings
        .merge(etc).expect("The required file '/etc/cadmium.toml' is missing!")
        .merge(Environment::with_prefix("CADMIUM")).expect("Could not load environment!")
        .merge(glob("/etc/cadmium.d/*")
            .expect("It seems the extra config dir does not exist, this is a problem...")
            .map(|path| File::from(path.expect("Could not load path")))
            .collect::<Vec<_>>()).expect("Could not load extra files");
    settings.try_into::<HashMap<String, String>>().unwrap()
}
