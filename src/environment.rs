use std::fs;

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub connection_string: String,
}

impl Config {
    pub fn new() -> Self {
        let toml_str = fs::read_to_string("AppSettings.toml").unwrap();
        let value = toml_str.parse::<Value>().unwrap();

        Config {
            connection_string: value["Database"]["connection_string"]
                .as_str()
                .unwrap()
                .to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
