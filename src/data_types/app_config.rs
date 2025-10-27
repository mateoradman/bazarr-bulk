use std::fmt;

use config::{Config, ConfigError, File, FileFormat};
use reqwest::Url;
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
    pub port: Option<String>,
    pub base_url: String,
    pub api_key: String,
}

impl AppConfig {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::new(config_path, FileFormat::Json))
            .set_default("host", "0.0.0.0")?
            .set_default("protocol", "http")?
            .set_default("baseUrl", "")?
            .build()?;

        config.try_deserialize()
    }

    pub fn construct_url(&self) -> Url {
        let mut bazarr_url = format!("{}://{}", self.protocol, self.host);

        if let Some(port) = &self.port {
            bazarr_url = format!("{}:{}", bazarr_url, port);
        }

        // clean the base_url by removing leading and trailing slashes
        let clean_base_url = self.base_url.trim_matches('/');

        let mut url = Url::parse(&bazarr_url).unwrap();
        url.path_segments_mut()
            .unwrap()
            .push(clean_base_url)
            .push("api");

        let mut masked_url = url.clone();
        if url.username() != "" {
            masked_url.set_username("*****").unwrap();
        }
        if url.password().is_some() {
            masked_url.set_password(Some("*****")).ok();
        }

        println!("Bazarr API URL: {}", masked_url);
        url
    }
}
