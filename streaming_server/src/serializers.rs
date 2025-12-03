use crate::models::{Anime, News, NewsCategory, Role, Studio, Tags};
use crate::mappers::{EpisodesStaff, EpisodesStaffUpdate, UserProfileEdit, UserPublic, UserStaff};
use chrono::NaiveDateTime;
use rocket::form::FromForm;
use rocket::fs::TempFile;
use rocket::serde::{Deserialize, Serialize};
use serde;
use surrealdb::sql::Thing;

pub type File = String;

#[derive(Deserialize)]
pub struct UserPostRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum GenericResponse {
    Message(String),
    Episode(EpisodesStaff),
    EpisodeUpdate(EpisodesStaffUpdate),
    User(UserStaff),
    UserStaff(UserStaff),
    PublicUser(UserPublic),
    UserProfile(UserProfileEdit),
    Anime(Anime),
    Studio(Studio),
    Tag(Tags),
    Role(Role),
    NewsCategory(NewsCategory),
    News(News),
}

#[derive(Serialize, Deserialize)]
pub struct Password {
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordID {
    pub discord_id: Option<i64>,
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub message: GenericResponse,
    pub status: i16,
}

#[derive(Deserialize)]
pub struct SingleMessage {
    pub message: String,
}

#[derive(Debug, Clone, serde::Deserialize, Serialize)]
pub struct PasswordChange {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MalAnimeData {
    pub data: MalAnime,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MalAnime {
    pub url: String,
    pub images: Option<Images>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub source: Option<String>,
    pub aired: Option<Aired>,
    pub episodes: Option<u32>,
    pub status: Option<String>,
    pub season: Option<String>,
    pub year: Option<u32>,
    pub studios: Vec<Entity>,
    pub genres: Vec<Entity>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Aired {
    pub from: Option<String>,
    pub to: Option<String>,
    pub prop: Option<AiredProp>,
    pub string: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiredProp {
    pub from: Option<DateInfo>,
    pub to: Option<DateInfo>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DateInfo {
    pub day: Option<u8>,
    pub month: Option<u8>,
    pub year: Option<u16>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Images {
    pub jpg: Option<ImageUrls>,
    pub webp: Option<ImageUrls>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageUrls {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Entity {
    pub mal_id: u32,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalEpisodeData {
    pub data: MalEpisode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalEpisode {
    pub mal_id: i64,
    pub url: String,
    pub title: String,
    pub title_japanese: Option<String>,
    pub title_romanji: Option<String>,
    
    pub duration: Option<i16>,
}

impl MalEpisode {
    pub fn default() -> Self {
        Self {
            mal_id: 0,
            url: "".to_string(),
            title: "".to_string(),
            title_japanese: None,
            title_romanji: None,
            duration: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddEpisode {
    pub title: Option<String>,
    pub discord_id: Option<i64>,
    pub translators: String,
    pub proofreaders: String,
    pub uploaders: Option<String>,
    pub typesetters: Option<String>,
    pub anime: String,
    pub episode: f64,
    pub length: i32,
    pub image: File,
    pub description: Option<String>,
    pub color: String,
    pub subtitles: File,
    pub torrent: File,
    pub video_players: Option<Vec<String>>,
    pub discord_ping: bool,
    pub webhook: bool,
    pub date: Option<NaiveDateTime>,
}

#[derive(FromForm, Debug)]
pub struct AddUpdateEpisodeForm<'r> {
    pub data: String,
    #[field(name = "image")]
    pub image: Option<TempFile<'r>>,
    #[field(name = "torrent")]
    pub torrent: Option<TempFile<'r>>,
    #[field(name = "subtitles")]
    pub subtitles: Option<TempFile<'r>>,
}

#[derive(FromForm, Debug)]
pub struct AddUpdateRoleForm<'r> {
    pub data: String,
    #[field(name = "icon")]
    pub icon: Option<TempFile<'r>>,
}

#[derive(FromForm, Debug)]
pub struct UniversalUpdateForm<'r> {
    pub data: String,
    #[field(name = "image")]
    pub image: Option<TempFile<'r>>,
}

#[derive(FromForm, Debug)]
pub struct NewsForm<'r> {
    pub data: String,
    #[field(name = "image")]
    pub image: Option<TempFile<'r>>,
    #[field(name = "thumbnail")]
    pub thumbnail: Option<TempFile<'r>>,
}



#[derive(Debug, FromForm)]
pub struct UpdateAnimeBackgroundPositionForm {
    pub id: String,
    pub background_position: i8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateAnimeBackgroundPositionData {
    pub background_position: i8,
}





pub fn dummy_id() -> Thing {
    Thing::from(("tags", surrealdb::sql::Id::Number(0)))
}

#[derive(Serialize, Debug)]
pub struct WebhookJson {
    pub username: String,
    pub content: String,
    pub embeds: Vec<Embed>,
}

#[derive(Serialize, Debug)]
pub struct Embed {
    #[serde(rename = "type")]
    pub embed_type: String,
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: u32,
    pub fields: Option<Vec<EmbedField>>,
    pub author: Option<EmbedAuthor>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub footer: Option<EmbedFooter>,
}

#[derive(Serialize, Debug)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize, Debug)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Debug)]
pub struct EmbedImage {
    pub url: String,
}

#[derive(Serialize, Debug)]
pub struct EmbedThumbnail {
    pub url: String,
}

#[derive(Serialize, Debug)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: String,
    pub proxy_icon_url: String,
}

impl Embed {
    pub fn add_field(&mut self, name: impl Into<String>, value: impl Into<String>, inline: bool) {
        if self.fields.is_none() {
            self.fields = Some(Vec::new());
        }
        self.fields.as_mut().unwrap().push(EmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        });
    }
}
