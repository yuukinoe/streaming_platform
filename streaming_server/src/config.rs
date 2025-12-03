use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub group: GroupConfig,
    pub session: SessionConfig,
    pub debug: bool,
    pub surrealdb: SurrealDbConfig,
    pub discord: DiscordConfig,
    pub directories: DirectoriesConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub domain: String,
    pub backend_url: String,
    pub frontend_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupConfig {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionConfig {
    pub duration_minutes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealDbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub episodes_webhook_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoriesConfig {
    pub media_dir: String,
    pub anime_folder: String,
    pub anime_thumbnails_folder: String,
    pub episodes_folder: String,
    pub episodes_images_folder: String,
    pub users_folder: String,
    pub torrents_folder: String,
    pub subtitles_folder: String,
    pub avatars_folder: String,
    pub roles_icons_folder: String,
    pub news_main_folder: String,
    pub news_images_folder: String,
    pub news_thumbnails_folder: String,
}
