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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioLanguageItem {
    pub name: String,
    pub code2: String,
    pub code3: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subtitle {
    pub path: Option<String>,
    #[serde(flatten)]
    pub audio_language_item: AudioLanguageItem,
}
