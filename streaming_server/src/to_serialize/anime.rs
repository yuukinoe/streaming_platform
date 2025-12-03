use crate::{models::studio::Studio, models::tags::Tags, serializers::File};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use surrealdb::{Surreal, engine::remote::ws::Client};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAnimeModel {
    pub mal: Option<String>,
    pub shinden: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub anime_type: Option<String>,
    pub episodes: Option<i32>,
    pub status: Option<String>,
    pub in_progress: bool,
    pub release_time: Option<String>,
    pub aired: Option<NaiveDateTime>,
    pub source: Option<String>,
    pub tags: Option<Vec<crate::models::tags::Tags>>,
    pub studio: Option<Vec<crate::models::studio::Studio>>,
    pub image: crate::serializers::File,
    pub date: Option<NaiveDateTime>,
    pub slug: String,
    pub alternative_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimeModel {
    pub id: surrealdb::sql::Thing,
    pub mal: Option<String>,
    pub shinden: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub anime_type: Option<String>,
    pub episodes: Option<i32>,
    pub status: Option<String>,
    pub in_progress: bool,
    pub release_time: Option<String>,
    pub aired: Option<NaiveDateTime>,
    pub source: Option<String>,
    pub tags: Option<Vec<crate::models::tags::Tags>>,
    pub studio: Option<Vec<crate::models::studio::Studio>>,
    pub image: crate::serializers::File,
    pub date: Option<NaiveDateTime>,
    pub slug: String,
    pub alternative_title: Option<String>,
}

impl AnimeModel {
    pub async fn new(
        db: &Surreal<Client>,
        mal: Option<String>,
        shinden: Option<String>,
        title: String,
        description: Option<String>,
        anime_type: Option<String>,
        episodes: Option<i32>,
        status: Option<String>,
        in_progress: bool,
        release_time: Option<String>,
        aired: Option<NaiveDateTime>,
        source: Option<String>,
        tags: Option<Vec<crate::models::tags::Tags>>,
        studio: Option<Vec<crate::models::studio::Studio>>,
        image: crate::serializers::File,
        date: Option<NaiveDateTime>,
        alternative_title: Option<String>,
    ) -> Self {
        let slug = slugify!(&title, separator = "-");
        let new_anime = NewAnimeModel {
            mal,
            shinden,
            title,
            description,
            anime_type,
            episodes,
            status,
            in_progress,
            release_time,
            aired,
            source,
            tags,
            studio,
            image,
            date,
            slug: slug.to_string(),
            alternative_title,
        };
        let res: Option<AnimeModel> = db
            .create("anime")
            .content(new_anime)
            .await
            .unwrap()
            .take()
            .unwrap();
        res.unwrap()
    }

    // pub async fn order_by(&mut self, field: String) -> Vec<AnimeModel> {
    //
    // };

    pub async fn order_by(db: &Surreal<Client>, field: String) -> Vec<AnimeModel> {
        let query = format!("SELECT * FROM anime ORDER BY {}", field);
        let res = db.query(query).await;
        match res {
            Ok(mut result) => result.take(0).unwrap_or_default(),
            Err(_) => vec![],
        }
    }

    pub async fn new_from_struct(
        db: &Surreal<Client>,
        anime: AnimeModel,
    ) -> Result<AnimeModel, surrealdb::Error> {
        let new_anime = NewAnimeModel {
            mal: anime.mal,
            shinden: anime.shinden,
            title: anime.title,
            description: anime.description,
            anime_type: anime.anime_type,
            episodes: anime.episodes,
            status: anime.status,
            in_progress: anime.in_progress,
            release_time: anime.release_time,
            aired: anime.aired,
            source: anime.source,
            tags: anime.tags,
            studio: anime.studio,
            image: anime.image,
            date: anime.date,
            slug: anime.slug,
            alternative_title: anime.alternative_title,
        };
        let res: Option<AnimeModel> = db.create("anime").content(new_anime).await?.take().unwrap();
        match res {
            Some(anime) => Ok(anime),
            None => Err(surrealdb::Error::Api(
                surrealdb::error::Api::InvalidRequest("Failed to create anime".into()),
            )),
        }
    }
    pub async fn exists_by_field(db: &Surreal<Client>, field: &str, value: &str) -> bool {
        let query = format!("SELECT * FROM anime WHERE {} = $value", field);
        let value = value.to_string();
        let res = db.query(query).bind(("value", value)).await;
        match res {
            Ok(mut result) => {
                let animes: Vec<AnimeModel> = result.take(0).unwrap_or_default();
                !animes.is_empty()
            }
            Err(_) => false,
        }
    }

    pub async fn update(
        &mut self,
        db: &Surreal<Client>,
        mal: Option<String>,
        shinden: Option<String>,
        title: String,
        description: Option<String>,
        anime_type: Option<String>,
        episodes: Option<i32>,
        status: Option<String>,
        in_progress: Option<bool>,
        release_time: Option<String>,
        aired: Option<NaiveDateTime>,
        source: Option<String>,
        tags: Option<Vec<Tags>>,
        studio: Option<Vec<Studio>>,
        image: File,
        date: Option<NaiveDateTime>,
        slug: String,
        alternative_title: Option<String>,
    ) -> Self {
        if let Some(mal) = mal {
            self.mal = Some(mal);
        }
        if let Some(shinden) = shinden {
            self.shinden = Some(shinden);
        }
        self.title = title;
        if let Some(description) = description {
            self.description = Some(description);
        }
        if let Some(anime_type) = anime_type {
            self.anime_type = Some(anime_type);
        }
        if let Some(episodes) = episodes {
            self.episodes = Some(episodes);
        }
        if let Some(status) = status {
            self.status = Some(status);
        }
        if let Some(in_progress) = in_progress {
            self.in_progress = in_progress;
        }
        if let Some(release_time) = release_time {
            self.release_time = Some(release_time);
        }
        if let Some(aired) = aired {
            self.aired = Some(aired);
        }
        if let Some(source) = source {
            self.source = Some(source);
        }

        if let Some(tags) = tags {
            self.tags = Some(tags);
        }
        if let Some(studio) = studio {
            self.studio = Some(studio);
        }
        self.image = image;
        if let Some(date) = date {
            self.date = Some(date);
        }
        self.slug = slug;
        if let Some(alternative_title) = alternative_title {
            self.alternative_title = Some(alternative_title);
        }
        self.slug = self.slug.clone();
        let anime: Option<AnimeModel> = db
            .update(("anime", &self.id.to_string()))
            .content(self.clone())
            .await
            .unwrap()
            .take();
        anime.unwrap()
    }

    pub async fn update_from_struct(
        &mut self,
        db: &Surreal<Client>,
        anime: AnimeModel,
    ) -> Result<AnimeModel, surrealdb::Error> {
        let res: Option<AnimeModel> = db
            .update(("anime", anime.id.id.to_raw()))
            .merge(NewAnimeModel::from(anime))
            .await?;
        res.ok_or(surrealdb::Error::Api(
            surrealdb::error::Api::InvalidRequest("Failed to update anime".into()),
        ))
    }

    pub async fn get(db: &Surreal<Client>, id: String) -> Option<AnimeModel> {
        let res: Result<Option<AnimeModel>, surrealdb::Error> =
            db.select(("anime", &id.to_string())).await;
        res.unwrap_or_else(|_| None)
    }
    pub async fn get_by_field(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<Option<AnimeModel>, surrealdb::Error> {
        let query = format!("SELECT * FROM anime WHERE {} = $value", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<AnimeModel>>(0);
        match res {
            Ok(animes) => Ok(animes.into_iter().next()),
            Err(e) => Err(e),
        }
    }
    pub async fn all(db: &Surreal<Client>) -> Result<Vec<AnimeModel>, surrealdb::Error> {
        let mut res = db.query("SELECT * FROM anime").await?;
        res.take::<Vec<AnimeModel>>(0)
    }
    pub async fn count(db: &Surreal<Client>) -> Result<i64, surrealdb::Error> {
        let mut res = db.query("SELECT COUNT() FROM anime").await?;
        let count: i64 = res.take::<Vec<i64>>(0)?.first().copied().unwrap_or(0);
        Ok(count)
    }
    pub async fn exists(db: &Surreal<Client>, id: Uuid) -> bool {
        let res: Result<Option<AnimeModel>, surrealdb::Error> =
            db.select(("anime", &id.to_string())).await;
        match res {
            Ok(Some(_)) => true,
            _ => false,
        }
    }
    pub async fn filter(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<Vec<AnimeModel>, surrealdb::Error> {
        let query = format!("SELECT * FROM anime WHERE {} = $value", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<AnimeModel>>(0);
        res
    }
    pub async fn delete(db: &Surreal<Client>, id: &str) -> bool {
        let res: Result<Option<AnimeModel>, surrealdb::Error> =
            db.delete(("anime", &id.to_string())).await;
        match res {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl From<AnimeModel> for NewAnimeModel {
    fn from(anime: AnimeModel) -> Self {
        NewAnimeModel {
            mal: anime.mal,
            shinden: anime.shinden,
            title: anime.title,
            description: anime.description,
            anime_type: anime.anime_type,
            episodes: anime.episodes,
            status: anime.status,
            in_progress: anime.in_progress,
            release_time: anime.release_time,
            aired: anime.aired,
            source: anime.source,
            tags: anime.tags,
            studio: anime.studio,
            image: anime.image,
            date: anime.date,
            slug: anime.slug.clone(),
            alternative_title: anime.alternative_title,
        }
    }
}
