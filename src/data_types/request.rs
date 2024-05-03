use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::response::Subtitle;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, ValueEnum)]
pub enum MediaType {
    Movie,
    TVShow,
}

impl FromStr for MediaType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "movie" => Ok(MediaType::Movie),
            "tv-show" => Ok(MediaType::TVShow),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionPayload {
    pub id: u32,
    #[serde(rename = "type")]
    pub media_type: String,
    pub language: String,
    pub path: String,

    // used only for sync action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_offset_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_fix_framerate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gss: Option<bool>,
}

impl ActionPayload {
    pub fn new(id: u32, media_type: &str, subtitle: &Subtitle) -> Self {
        ActionPayload {
            id,
            media_type: String::from(media_type),
            language: subtitle.audio_language_item.code2.clone().unwrap(),
            path: subtitle.path.clone().unwrap(),
            reference: None,
            max_offset_seconds: None,
            no_fix_framerate: None,
            gss: None,
        }
    }
}
