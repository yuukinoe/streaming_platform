use crate::{models::anime::AnimeModel, models::users::UserPublic, serializers::File};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use surrealdb::{Surreal, engine::remote::ws::Client, sql::Thing};
use uuid::Uuid;








#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicRole {
    pub id: Thing,
    pub name: String,
    pub administrative_role: bool,
    pub visible: bool,
    pub hierarchy: i32,
    pub color: String,
    pub icon: File,
    pub date: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserPublic {
    pub id: surrealdb::sql::Thing,
    pub name: String,
    pub avatar: Option<File>,
    pub description: Option<String>,
    pub translator: bool,
    pub proofreader: bool,
    pub uploader: bool,
    pub editor: bool,
    pub roles: Option<Vec<PublicRole>>,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicEpisodes {
    pub id: surrealdb::sql::Thing,
    pub title: Option<String>,
    pub translators: Option<Vec<UserPublic>>,
    pub proofreaders: Option<Vec<UserPublic>>,
    pub uploaders: Option<Vec<UserPublic>>,
    pub typesetters: Option<Vec<UserPublic>>,
    pub anime: Anime,
    pub episode: f64,
    pub length: i32,
    pub image: File,
    pub description: Option<String>,
    pub color: String,
    pub video_players: Option<Vec<String>>,
    pub date: Option<NaiveDateTime>,
    pub slug: String,
}













































#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewEpisodes {
    pub title: Option<String>,
    pub discord_id: Option<i64>,
    pub translators: Option<UserPublic>,
    pub proofreaders: Option<UserPublic>,
    pub uploaders: Option<UserPublic>,
    pub typesetters: Option<UserPublic>,
    pub anime: AnimeModel,
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
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Episodes {
    pub id: surrealdb::sql::Thing,
    pub title: Option<String>,
    pub discord_id: Option<i64>,
    pub translators: Option<UserPublic>,
    pub proofreaders: Option<UserPublic>,
    pub uploaders: Option<UserPublic>,
    pub typesetters: Option<UserPublic>,
    pub anime: AnimeModel,
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
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EpisodesPublic {
    pub id: Thing,
    pub title: Option<String>,
    pub discord_id: Option<i64>,
    pub translators: Option<UserPublic>,
    pub proofreaders: Option<UserPublic>,
    pub uploaders: Option<UserPublic>,
    pub typesetters: Option<UserPublic>,
    pub anime: AnimeModel,
    pub episode: f64,
    pub length: i32,
    pub image: File,
    pub description: Option<String>,
    pub color: String,
    pub video_players: Option<Vec<String>>,
    pub discord_ping: bool,
    pub webhook: bool,
    pub date: Option<NaiveDateTime>,
}

impl Episodes {
    pub async fn new(
        db: &Surreal<Client>,
        title: Option<String>,
        discord_id: Option<i64>,
        translators: Option<UserPublic>,
        proofreaders: Option<UserPublic>,
        uploaders: Option<UserPublic>,
        typesetters: Option<UserPublic>,
        anime: AnimeModel,
        episode: f64,
        length: i32,
        image: File,
        description: Option<String>,
        color: String,
        subtitles: File,
        torrent: File,
        video_players: Option<Vec<String>>,
        discord_ping: bool,
        webhook: bool,
        date: Option<NaiveDateTime>,
    ) -> Self {
        let slug = slugify!(&anime.clone().title.clone(), separator = "-");
        let new_episode = NewEpisodes {
            title,
            discord_id,
            translators,
            proofreaders,
            uploaders,
            typesetters,
            anime,
            episode,
            length,
            image,
            description,
            color,
            subtitles,
            torrent,
            video_players,
            discord_ping,
            webhook,
            date,
            slug,
        };
        let res: Option<Episodes> = db
            .create("episodes")
            .content(new_episode)
            .await
            .unwrap()
            .take()
            .unwrap();
        res.unwrap()
    }

    pub async fn new_from_struct(
        db: &Surreal<Client>,
        episode: Episodes,
    ) -> Result<Episodes, surrealdb::Error> {
        let new_episode = NewEpisodes {
            title: episode.title,
            discord_id: episode.discord_id,
            translators: episode.translators,
            proofreaders: episode.proofreaders,
            uploaders: episode.uploaders,
            typesetters: episode.typesetters,
            anime: episode.anime,
            episode: episode.episode,
            length: episode.length,
            image: episode.image,
            description: episode.description,
            color: episode.color,
            subtitles: episode.subtitles,
            torrent: episode.torrent,
            video_players: episode.video_players,
            discord_ping: episode.discord_ping,
            webhook: episode.webhook,
            date: episode.date,
            slug: episode.slug,
        };
        let res: Option<Episodes> = db
            .create("episodes")
            .content(new_episode)
            .await?
            .take()
            .unwrap();
        match res {
            Some(episode) => Ok(episode),
            None => Err(surrealdb::Error::Api(
                surrealdb::error::Api::InvalidRequest("Failed to create episode".into()),
            )),
        }
    }
    pub async fn exists_by_field(db: &Surreal<Client>, field: &str, value: &str) -> bool {
        let query = format!("SELECT * FROM episodes WHERE {} = $value", field);
        let value = value.to_string();
        let res = db.query(query).bind(("value", value)).await;
        match res {
            Ok(mut result) => {
                let episodes: Vec<Episodes> = result.take(0).unwrap_or_default();
                !episodes.is_empty()
            }
            Err(_) => false,
        }
    }

    pub async fn update(
        &mut self,
        db: &Surreal<Client>,
        title: Option<String>,
        discord_id: Option<i64>,
        translators: Option<UserPublic>,
        proofreaders: Option<UserPublic>,
        uploaders: Option<UserPublic>,
        typesetters: Option<UserPublic>,
        anime: AnimeModel,
        episode: f64,
        length: i32,
        image: File,
        description: Option<String>,
        color: String,
        subtitles: File,
        torrent: File,
        video_players: Option<Vec<String>>,
        discord_ping: bool,
        webhook: bool,
        date: Option<NaiveDateTime>,
    ) -> Self {
        if let Some(title) = title {
            self.title = Some(title);
        }

        if let Some(discord_id) = discord_id {
            self.discord_id = Some(discord_id);
        }
        if let Some(translators) = translators {
            self.translators = Some(translators);
        }
        if let Some(proofreaders) = proofreaders {
            self.proofreaders = Some(proofreaders);
        }
        if let Some(uploaders) = uploaders {
            self.uploaders = Some(uploaders);
        }
        if let Some(typesetters) = typesetters {
            self.typesetters = Some(typesetters);
        }
        self.anime = anime;
        self.episode = episode;
        self.length = length;
        self.image = image;
        if let Some(description) = description {
            self.description = Some(description);
        }
        self.color = color;
        self.subtitles = subtitles;
        self.torrent = torrent;
        if let Some(video_players) = video_players {
            self.video_players = Some(video_players);
        }
        self.discord_ping = discord_ping;
        self.webhook = webhook;
        if let Some(date) = date {
            self.date = Some(date);
        }
        self.slug = self.slug.clone();
        let episode: Option<Episodes> = db
            .update(("episodes", &self.id.to_string()))
            .content(self.clone())
            .await
            .unwrap()
            .take();
        episode.unwrap()
    }
    
    
    pub async fn save(self, db: &Surreal<Client>) -> Option<Self> {
        let episode: Option<Episodes> = db
            .update(("episodes", &self.id.id.to_raw()))
            .content(self.clone())
            .await
            .unwrap()
            .take();
        episode
    }
    
    pub async fn get(db: &Surreal<Client>, id: String) -> Option<Episodes> {
        let res: Result<Option<Episodes>, surrealdb::Error> =
            db.select(("episodes", &id.to_string())).await;
        res.unwrap_or_else(|_| None)
    }

    pub async fn update_from_struct(
        &mut self,
        db: &Surreal<Client>,
        episodes: Episodes,
    ) -> Result<Episodes, surrealdb::Error> {
        let res: Option<Episodes> = db
            .update(("episodes", episodes.id.id.to_raw()))
            .merge(NewEpisodes::from(episodes))
            .await?;
        res.ok_or(surrealdb::Error::Api(
            surrealdb::error::Api::InvalidRequest("Failed to update episodes".into()),
        ))
    }

    pub async fn get_by_field(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<Option<Episodes>, surrealdb::Error> {
        let query = format!("SELECT * FROM episodes WHERE {} = $value", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<Episodes>>(0);
        match res {
            Ok(episodes) => Ok(episodes.into_iter().next()),
            Err(e) => Err(e),
        }
    }
    pub async fn all(db: &Surreal<Client>) -> Result<Vec<Episodes>, surrealdb::Error> {
        let mut res = db.query("SELECT * FROM episodes ORDER BY anime.title DESC, episode ASC").await?;
        res.take::<Vec<Episodes>>(0)
    }
    pub async fn count(db: &Surreal<Client>) -> Result<i64, surrealdb::Error> {
        let mut res = db.query("SELECT COUNT() FROM episodes").await?;
        let count: i64 = res.take::<Vec<i64>>(0)?.first().copied().unwrap_or(0);
        Ok(count)
    }
    pub async fn exists(db: &Surreal<Client>, id: Uuid) -> bool {
        let res: Result<Option<Episodes>, surrealdb::Error> =
            db.select(("episodes", &id.to_string())).await;
        match res {
            Ok(Some(_)) => true,
            _ => false,
        }
    }
    pub async fn filter(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<Vec<Episodes>, surrealdb::Error> {
        let query = format!("SELECT * FROM episodes WHERE {} = $value ORDER BY episode ASC", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<Episodes>>(0);
        res
    }
    pub async fn delete(db: &Surreal<Client>, id: &str) -> bool {
        let res: Result<Option<Episodes>, surrealdb::Error> =
            db.delete(("episodes", &id.to_string())).await;
        match res {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl From<Episodes> for NewEpisodes {
    fn from(episode: Episodes) -> Self {
        NewEpisodes {
            title: episode.title,
            discord_id: episode.discord_id,
            translators: episode.translators,
            proofreaders: episode.proofreaders,
            uploaders: episode.uploaders,
            typesetters: episode.typesetters,
            anime: episode.anime,
            episode: episode.episode,
            length: episode.length,
            image: episode.image,
            description: episode.description,
            color: episode.color,
            subtitles: episode.subtitles,
            torrent: episode.torrent,
            video_players: episode.video_players,
            discord_ping: episode.discord_ping,
            webhook: episode.webhook,
            date: episode.date,
            slug: episode.slug.clone(),
        }
    }
}

impl From<Episodes> for EpisodesPublic {
    fn from(episode: Episodes) -> Self {
        EpisodesPublic {
            id: episode.id,
            title: episode.title,
            discord_id: episode.discord_id,
            translators: episode.translators,
            proofreaders: episode.proofreaders,
            uploaders: episode.uploaders,
            typesetters: episode.typesetters,
            anime: episode.anime,
            episode: episode.episode,
            length: episode.length,
            image: episode.image,
            description: episode.description,
            color: episode.color,
            video_players: episode.video_players,
            discord_ping: episode.discord_ping,
            webhook: episode.webhook,
            date: episode.date,
        }
    }
}
