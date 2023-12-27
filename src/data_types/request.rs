use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
pub struct SyncRequest {
    id: i32,
    media_type: MediaType,
    language: String,
    path: String,
}
