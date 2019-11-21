use serde::Deserialize;
use toml::de::Error;
use crate::ErrorKind;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub de: String,
    pub logtty: u16,
    pub displaytty: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            de: String::from("bspwm"),
            logtty: 2,
            displaytty: 3,
        }
    }
}

pub fn config_from_file(file: &str) -> Result<Config, ErrorKind> {
    let config = match std::fs::read_to_string(file) {
        Ok(t) => t,
        Err(_) => String::from("")
    };
    toml::from_str::<Config>(config.as_str()).map_err(|err| ErrorKind::ConfigLoadError(err))
}