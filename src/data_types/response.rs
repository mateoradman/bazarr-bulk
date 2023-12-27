use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    data: Vec<T>,
    total: i32,
}

#[derive(Debug, Deserialize)]
struct PaginatedEpisodeResponse {
    data: Vec<Episode>,
}

#[derive(Debug, Deserialize)]
pub struct Movie {
    subtitles: Vec<Subtitle>,
    #[serde(rename = "radarrId")]
    radarr_id: i32,
    #[serde(flatten)]
    common_attributes: CommonMediaAttributes,
}

#[derive(Debug, Deserialize)]
struct TVShow {
    episode_file_count: i32,
    episode_missing_count: i32,
    series_type: String,
    sonarr_series_id: i32,
    tvdb_id: i32,
    #[serde(flatten)]
    common_attributes: CommonMediaAttributes,
}

#[derive(Debug, Deserialize)]
struct Episode {
    audio_language: Vec<AudioLanguageItem>,
    episode: i32,
    monitored: bool,
    path: String,
    season: i32,
    sonarr_episode_id: i32,
    sonarr_series_id: i32,
    subtitles: Vec<Subtitle>,
    title: String,
}

#[derive(Debug, Deserialize)]
struct CommonMediaAttributes {
    title: String,
    #[serde(rename = "alternativeTitles")]
    alternative_titles: Vec<String>,
    audio_language: Vec<AudioLanguageItem>,
    fanart: String,
    #[serde(rename = "imdbId")]
    imdb_id: String,
    monitored: bool,
    overview: String,
    path: String,
    poster: String,
    #[serde(rename = "profileId")]
    profile_id: i32,
    year: String,
}

#[derive(Debug, Deserialize)]
struct AudioLanguageItem {
    name: String,
    code2: String,
    code3: String,
}

#[derive(Debug, Deserialize)]
struct Subtitle {
    path: Option<String>,
    forced: bool,
    hi: bool,
    file_size: Option<i32>,
    #[serde(flatten)]
    audio_language_item: AudioLanguageItem,
}
