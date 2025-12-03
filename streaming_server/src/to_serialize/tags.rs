use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use surrealdb::{Surreal, engine::remote::ws::Client, sql::Thing};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTag {
    pub name: String,
    pub description: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tags {
    pub id: Thing,
    pub name: String,
    pub description: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub slug: String,
}

impl Tags {
    pub async fn new(db: &Surreal<Client>, name: String, description: Option<String>) -> Self {
        let slug = slugify!(&name, separator = "-");
        let new_tag = NewTag {
            name,
            description,
            date: Some(chrono::Local::now().naive_local()),
            slug,
        };
        let res: Option<Tags> = db
            .create("tags")
            .content(new_tag)
            .await
            .unwrap()
            .take()
            .unwrap();
        res.unwrap()
    }

    pub async fn new_from_struct(
        db: &Surreal<Client>,
        tag: Tags,
    ) -> Result<Tags, surrealdb::Error> {
        let new_tag = NewTag {
            name: tag.name,
            description: tag.description,
            date: tag.date,
            slug: tag.slug,
        };
        let res: Option<Tags> = db
            .create("tags")
            .content(new_tag)
            .await?
            .take()
            .unwrap();
        match res {
            Some(tag) => Ok(tag),
            None => Err(surrealdb::Error::Api(
                surrealdb::error::Api::InvalidRequest("Failed to create tag".into()),
            )),
        }
    }

    pub async fn update(
        &mut self,
        db: &Surreal<Client>,
        name: Option<String>,
        description: Option<String>,
    ) -> Self {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(description) = description {
            self.description = Some(description);
        }
        self.date = Some(chrono::Local::now().naive_local());
        self.slug = self.slug.clone();
        let tag: Option<Tags> = db
            .update(("tags", &self.id.to_string()))
            .content(self.clone())
            .await
            .unwrap()
            .take();
        tag.unwrap()
    }

    pub async fn get(db: &Surreal<Client>, id: Uuid) -> Option<Tags> {
        let res: Result<Option<Tags>, surrealdb::Error> =
            db.select(("tags", &id.to_string())).await;
        res.unwrap_or_else(|_| None)
    }

    pub async fn get_by_field(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<Option<Tags>, surrealdb::Error> {
        let query = format!("SELECT * FROM tags WHERE {} = $value", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<Tags>>(0);
        match res {
            Ok(tags) => Ok(tags.into_iter().next()),
            Err(e) => Err(e),
        }
    }
    pub async fn all(db: &Surreal<Client>) -> Result<Vec<Tags>, surrealdb::Error> {
        let mut res = db.query("SELECT * FROM tags").await?;
        res.take::<Vec<Tags>>(0)
    }
    pub async fn count(db: &Surreal<Client>) -> Result<i64, surrealdb::Error> {
        let mut res = db.query("SELECT COUNT() FROM tags").await?;
        let count: i64 = res.take::<Vec<i64>>(0)?.first().copied().unwrap_or(0);
        Ok(count)
    }
    pub async fn exists(db: &Surreal<Client>, id: Uuid) -> bool {
        let res: Result<Option<Tags>, surrealdb::Error> =
            db.select(("tags", &id.to_string())).await;
        match res {
            Ok(Some(_)) => true,
            _ => false,
        }
    }
    pub async fn exists_by_field(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<bool, surrealdb::Error> {
        let query = format!("SELECT * FROM tags WHERE {} = $value", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<Tags>>(0);
        match res {
            Ok(tags) => Ok(!tags.is_empty()),
            Err(e) => Err(e),
        }
    }

    pub async fn filter(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<Vec<Tags>, surrealdb::Error> {
        let query = format!("SELECT * FROM tags WHERE {} = $value", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<Tags>>(0);
        res
    }
    pub async fn delete(db: &Surreal<Client>, id: Uuid) -> bool {
        let res: Result<Option<Tags>, surrealdb::Error> =
            db.delete(("tags", &id.to_string())).await;
        match res {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
