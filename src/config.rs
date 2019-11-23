use crate::ErrorKind;
use serde::Deserialize;

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
            de: "bspwm".into(),
            logtty: 2,
            displaytty: 3,
        }
    }
}

pub fn config_from_file(file: &str) -> Result<Config, ErrorKind> {
    let config = std::fs::read_to_string(file).unwrap_or_default();
    toml::from_str(config.as_str()).map_err(|err| ErrorKind::ConfigLoadError(err) )
}
