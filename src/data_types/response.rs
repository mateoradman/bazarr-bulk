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
    #[serde(flatten)]
    pub common_attributes: CommonMediaAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TVShow {
    #[serde(rename = "episodeFileCount")]
    pub episode_file_count: u32,
    #[serde(rename = "episodeMissingCount")]
    pub episode_missing_count: u32,
    #[serde(rename = "seriesType")]
    pub series_type: String,
    #[serde(rename = "sonarrSeriesId")]
    pub sonarr_series_id: u32,
    #[serde(rename = "tvdbId")]
    pub tvdb_id: u32,
    #[serde(flatten)]
    pub common_attributes: CommonMediaAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub audio_language: Vec<AudioLanguageItem>,
    pub episode: u32,
    pub monitored: bool,
    pub path: String,
    pub season: u32,
    #[serde(rename = "sonarrEpisodeId")]
    pub sonarr_episode_id: u32,
    pub subtitles: Vec<Subtitle>,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonMediaAttributes {
    pub title: String,
    #[serde(rename = "alternativeTitles")]
    pub alternative_titles: Vec<String>,
    pub audio_language: Vec<AudioLanguageItem>,
    pub fanart: String,
    #[serde(rename = "imdbId")]
    pub imdb_id: String,
    pub monitored: bool,
    pub overview: String,
    pub path: String,
    pub poster: String,
    #[serde(rename = "profileId")]
    pub profile_id: u32,
    pub year: String,
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
    pub forced: bool,
    pub hi: bool,
    pub file_size: Option<u32>,
    #[serde(flatten)]
    pub audio_language_item: AudioLanguageItem,
}
