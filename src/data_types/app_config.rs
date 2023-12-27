use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub bazarr_host: String,
    pub bazarr_port: String,
    pub bazarr_api_key: String,
}

impl AppConfig {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::new(config_path, FileFormat::Json))
            .set_default("bazarr_host", "0.0.0.0")?
            .set_default("bazarr_port", "6767")?
            .build()?;

        config.try_deserialize()
    }
}
