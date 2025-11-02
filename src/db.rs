use directories::ProjectDirs;
use rusqlite::{params, Connection, Result};
use std::{collections::HashSet, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

use crate::data_types::response::{Episode, Movie, Subtitle};

fn get_db_path(custom_path: Option<PathBuf>) -> std::result::Result<PathBuf, String> {
    // Priority: 1. CLI argument, 2. Environment variable, 3. Default user data directory
    if let Some(path) = custom_path {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create database directory: {}", e))?;
        }
        return Ok(path);
    }

    // Check BB_DATA_DIR environment variable
    if let Ok(data_dir) = std::env::var("BB_DATA_DIR") {
        let db_path = PathBuf::from(data_dir).join("database.db");
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create database directory: {}", e))?;
        }
        return Ok(db_path);
    }

    // Fallback to default user data directory
    ProjectDirs::from("com", "mateoradman", "bazarr-bulk")
        .and_then(|proj_dirs| {
            let data_dir = proj_dirs.data_local_dir();
            std::fs::create_dir_all(data_dir).ok()?;
            Some(data_dir.join("database.db"))
        })
        .ok_or_else(|| "Failed to obtain a default database path".to_string())
}

pub async fn init_db(custom_path: Option<PathBuf>) -> Result<Arc<Mutex<Connection>>> {
    let db_path = get_db_path(custom_path).map_err(|e| rusqlite::Error::InvalidPath(e.into()))?;

    println!("Using database at: {}", db_path.display());

    let conn = tokio::task::spawn_blocking(move || {
        let mut conn = Connection::open(db_path)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        create_tables(&mut conn)?;
        Ok::<_, rusqlite::Error>(conn)
    })
    .await
    .map_err(|e| rusqlite::Error::InvalidPath(e.to_string().into()))??;

    Ok(Arc::new(Mutex::new(conn)))
}

fn create_tables(conn: &mut Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS processed_movie_subtitles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            radarr_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            language_code TEXT NOT NULL,
            language_name TEXT NOT NULL,
            path TEXT,
            processed_at INTEGER NOT NULL,
            UNIQUE(radarr_id, language_code)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS processed_episode_subtitles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sonarr_episode_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            language_code TEXT NOT NULL,
            language_name TEXT NOT NULL,
            path TEXT,
            processed_at INTEGER NOT NULL,
            UNIQUE(sonarr_episode_id, language_code)
        )",
        [],
    )?;

    let table_exists: bool = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='processed_movie_subtitles'",
            [],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if table_exists {
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_movie_radarr ON processed_movie_subtitles(radarr_id)",
            [],
        )?;
    }

    let table_exists: bool = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='processed_episode_subtitles'",
            [],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if table_exists {
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_episode_sonarr ON processed_episode_subtitles(sonarr_episode_id)",
            [],
        )?;
    }

    Ok(())
}

pub async fn is_movie_subtitle_processed(
    conn: Arc<Mutex<Connection>>,
    radarr_id: u32,
    language_code: String,
) -> Result<bool> {
    tokio::task::spawn_blocking(move || {
        let conn = conn.blocking_lock();
        let mut stmt = conn.prepare(
            "SELECT 1 FROM processed_movie_subtitles 
             WHERE radarr_id = ?1 AND language_code = ?2",
        )?;
        stmt.exists(params![radarr_id, language_code])
    })
    .await
    .map_err(|e| rusqlite::Error::InvalidPath(e.to_string().into()))?
}

pub async fn is_episode_subtitle_processed(
    conn: Arc<Mutex<Connection>>,
    sonarr_episode_id: u32,
    language_code: String,
) -> Result<bool> {
    tokio::task::spawn_blocking(move || {
        let conn = conn.blocking_lock();
        let mut stmt = conn.prepare(
            "SELECT 1 FROM processed_episode_subtitles 
             WHERE sonarr_episode_id = ?1 AND language_code = ?2",
        )?;
        stmt.exists(params![sonarr_episode_id, language_code])
    })
    .await
    .map_err(|e| rusqlite::Error::InvalidPath(e.to_string().into()))?
}

pub async fn mark_episode_subtitle_processed(
    conn: Arc<Mutex<Connection>>,
    sonarr_episode_id: u32,
    title: String,
    subtitle: Subtitle,
) -> Result<bool> {
    let Some(language_code) = subtitle.audio_language_item.code2 else {
        return Ok(false);
    };

    tokio::task::spawn_blocking(move || {
        let conn = conn.blocking_lock();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let rows = conn.execute(
            "INSERT INTO processed_episode_subtitles 
             (sonarr_episode_id, title, language_code, language_name, path, processed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(sonarr_episode_id, language_code) DO NOTHING",
            params![
                sonarr_episode_id,
                title,
                language_code,
                subtitle.audio_language_item.name,
                subtitle.path,
                now
            ],
        )?;

        Ok(rows > 0)
    })
    .await
    .map_err(|e| rusqlite::Error::InvalidPath(e.to_string().into()))?
}

pub async fn mark_movie_subtitle_processed(
    conn: Arc<Mutex<Connection>>,
    radarr_id: u32,
    title: String,
    subtitle: Subtitle,
) -> Result<bool> {
    let Some(language_code) = subtitle.audio_language_item.code2 else {
        return Ok(false);
    };

    tokio::task::spawn_blocking(move || {
        let conn = conn.blocking_lock();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let rows = conn.execute(
            "INSERT INTO processed_movie_subtitles 
             (radarr_id, title, language_code, language_name, path, processed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(radarr_id, language_code) DO NOTHING",
            params![
                radarr_id,
                title,
                language_code,
                subtitle.audio_language_item.name,
                subtitle.path,
                now
            ],
        )?;

        Ok(rows > 0)
    })
    .await
    .map_err(|e| rusqlite::Error::InvalidPath(e.to_string().into()))?
}

pub async fn filter_unprocessed_movies(
    conn: Arc<Mutex<Connection>>,
    movies: Vec<Movie>,
) -> Result<Vec<Movie>> {
    if movies.is_empty() {
        return Ok(vec![]);
    }

    let radarr_ids: Vec<u32> = movies.iter().map(|m| m.radarr_id).collect();
    let conn_clone = conn.clone();

    let processed_ids: HashSet<u32> = tokio::task::spawn_blocking(move || {
        let conn = conn_clone.blocking_lock();
        let placeholders = radarr_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT DISTINCT radarr_id FROM processed_movie_subtitles 
             WHERE radarr_id IN ({})",
            placeholders
        );

        let mut stmt = conn.prepare(&query)?;
        let params: Vec<&dyn rusqlite::ToSql> = radarr_ids
            .iter()
            .map(|id| id as &dyn rusqlite::ToSql)
            .collect();

        let processed: HashSet<u32> =
            stmt.query_map(params.as_slice(), |row| row.get(0))?
                .collect::<std::result::Result<HashSet<u32>, rusqlite::Error>>()?;

        Ok::<_, rusqlite::Error>(processed)
    })
    .await
    .map_err(|e| rusqlite::Error::InvalidPath(e.to_string().into()))??;

    let mut unprocessed = Vec::new();
    for movie in movies {
        if !processed_ids.contains(&movie.radarr_id) {
            unprocessed.push(movie);
            continue;
        }

        let mut has_unprocessed = false;
        for sub in &movie.subtitles {
            if let Some(ref code) = sub.audio_language_item.code2 {
                if is_movie_subtitle_processed(conn.clone(), movie.radarr_id, code.clone()).await? {
                    continue;
                }
                has_unprocessed = true;
                break;
            }
        }

        if has_unprocessed {
            unprocessed.push(movie);
        }
    }

    Ok(unprocessed)
}

pub async fn filter_unprocessed_episodes(
    conn: Arc<Mutex<Connection>>,
    episodes: Vec<Episode>,
) -> Result<Vec<Episode>> {
    if episodes.is_empty() {
        println!("No episodes to filter");
        return Ok(vec![]);
    }

    let episode_ids: Vec<u32> = episodes.iter().map(|e| e.sonarr_episode_id).collect();
    println!("Checking {} episodes in database", episode_ids.len());
    let conn_clone = conn.clone();

    let processed_ids: HashSet<u32> = tokio::task::spawn_blocking(move || {
        let conn = conn_clone.blocking_lock();
        let placeholders = episode_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");
        let query = format!(
            "SELECT DISTINCT sonarr_episode_id FROM processed_episode_subtitles 
             WHERE sonarr_episode_id IN ({})",
            placeholders
        );

        let mut stmt = conn.prepare(&query)?;
        let params: Vec<&dyn rusqlite::ToSql> = episode_ids
            .iter()
            .map(|id| id as &dyn rusqlite::ToSql)
            .collect();

        let processed: HashSet<u32> =
            stmt.query_map(params.as_slice(), |row| row.get(0))?
                .collect::<std::result::Result<HashSet<u32>, rusqlite::Error>>()?;

        Ok::<_, rusqlite::Error>(processed)
    })
    .await
    .map_err(|e| rusqlite::Error::InvalidPath(e.to_string().into()))??;

    let mut unprocessed = Vec::new();
    for episode in episodes {
        if !processed_ids.contains(&episode.sonarr_episode_id) {
            unprocessed.push(episode);
            continue;
        }

        let mut has_unprocessed = false;
        for sub in &episode.subtitles {
            if let Some(ref code) = sub.audio_language_item.code2 {
                if is_episode_subtitle_processed(
                    conn.clone(),
                    episode.sonarr_episode_id,
                    code.clone(),
                )
                .await?
                {
                    continue;
                }
                has_unprocessed = true;
                break;
            }
        }

        if has_unprocessed {
            unprocessed.push(episode);
        }
    }

    Ok(unprocessed)
}
