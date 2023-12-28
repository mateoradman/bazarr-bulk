use std::fmt;

use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Http,
    Https,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Protocol::Http => write!(f, "http"),
            Protocol::Https => write!(f, "https"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub protocol: Protocol,
    pub host: String,
    pub port: String,
    pub api_key: String,
}

impl AppConfig {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::new(config_path, FileFormat::Json))
            .set_default("host", "0.0.0.0")?
            .set_default("port", "6767")?
            .set_default("protocol", "http")?
            .build()?;

        config.try_deserialize()
    }
}
