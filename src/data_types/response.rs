use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i32,
}

#[derive(Debug, Deserialize)]
pub struct PaginatedEpisodeResponse {
    pub data: Vec<Episode>,
}

#[derive(Debug, Deserialize)]
pub struct Movie {
    pub subtitles: Vec<Subtitle>,
    #[serde(rename = "radarrId")]
    pub radarr_id: i32,
    #[serde(flatten)]
    pub common_attributes: CommonMediaAttributes,
}

#[derive(Debug, Deserialize)]
pub struct TVShow {
    pub episode_file_count: i32,
    pub episode_missing_count: i32,
    pub series_type: String,
    pub sonarr_series_id: i32,
    pub tvdb_id: i32,
    #[serde(flatten)]
    pub common_attributes: CommonMediaAttributes,
}

#[derive(Debug, Deserialize)]
pub struct Episode {
    pub audio_language: Vec<AudioLanguageItem>,
    pub episode: i32,
    pub monitored: bool,
    pub path: String,
    pub season: i32,
    pub sonarr_episode_id: i32,
    pub sonarr_series_id: i32,
    pub subtitles: Vec<Subtitle>,
    pub title: String,
}

#[derive(Debug, Deserialize)]
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
    pub profile_id: i32,
    pub year: String,
}

#[derive(Debug, Deserialize)]
pub struct AudioLanguageItem {
    pub name: String,
    pub code2: String,
    pub code3: String,
}

#[derive(Debug, Deserialize)]
pub struct Subtitle {
    pub path: Option<String>,
    pub forced: bool,
    pub hi: bool,
    pub file_size: Option<i32>,
    #[serde(flatten)]
    pub audio_language_item: AudioLanguageItem,
}
