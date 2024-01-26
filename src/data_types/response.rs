use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub subtitles: Vec<Subtitle>,
    #[serde(rename = "radarrId")]
    pub radarr_id: u32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TVShow {
    #[serde(rename = "sonarrSeriesId")]
    pub sonarr_series_id: u32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    #[serde(rename = "sonarrEpisodeId")]
    pub sonarr_episode_id: u32,
    pub subtitles: Vec<Subtitle>,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioLanguageItem {
    pub name: String,
    pub code2: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subtitle {
    pub path: Option<String>,
    #[serde(flatten)]
    pub audio_language_item: AudioLanguageItem,
}

impl Subtitle {
    pub fn is_valid(&self) -> bool {
        self.path.is_some() && self.audio_language_item.code2.is_some()
    }
}
