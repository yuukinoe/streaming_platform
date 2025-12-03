use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use surrealdb::{Surreal, engine::remote::ws::Client};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStudio {
    pub name: String,
    pub description: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Studio {
    pub id: surrealdb::sql::Thing,
    pub name: String,
    pub description: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub slug: String,
}

impl Studio {
    pub async fn new(db: &Surreal<Client>, name: String, description: Option<String>) -> Self {
        let slug = slugify!(&name, separator = "-");
        let new_studio = NewStudio {
            name,
            description,
            date: Some(chrono::Local::now().naive_local()),
            slug,
        };
        let res: Option<Studio> = db
            .create("studio")
            .content(new_studio)
            .await
            .unwrap()
            .take()
            .unwrap();
        res.unwrap()
    }

    pub async fn new_from_struct(
        db: &Surreal<Client>,
        studio: Studio,
    ) -> Result<Studio, surrealdb::Error> {
        let new_studio = NewStudio {
            name: studio.name,
            description: studio.description,
            date: studio.date,
            slug: studio.slug,
        };
        let res: Option<Studio> = db
            .create("studio")
            .content(new_studio)
            .await?
            .take()
            .unwrap();
        match res {
            Some(studio) => Ok(studio),
            None => Err(surrealdb::Error::Api(
                surrealdb::error::Api::InvalidRequest("Failed to create studio".into()),
            )),
        }
    }

    pub async fn exists_by_field(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> bool {
        let query = format!("SELECT * FROM studio WHERE {} = $value", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await;
        match res {
            Ok(mut result) => {
                let studios: Vec<Studio> = result.take::<Vec<Studio>>(0).unwrap_or_default();
                !studios.is_empty()
            }
            Err(_) => false,
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
        let studio: Option<Studio> = db
            .update(("studio", &self.id.to_string()))
            .content(self.clone())
            .await
            .unwrap()
            .take();
        studio.unwrap()
    }

    pub async fn get(db: &Surreal<Client>, id: Uuid) -> Option<Studio> {
        let res: Result<Option<Studio>, surrealdb::Error> =
            db.select(("studio", &id.to_string())).await;
        res.unwrap_or_else(|_| None)
    }

    pub async fn get_by_field(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<Option<Studio>, surrealdb::Error> {
        let query = format!("SELECT * FROM studio WHERE {} = $value", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<Studio>>(0);
        match res {
            Ok(studios) => Ok(studios.into_iter().next()),
            Err(e) => Err(e),
        }
    }

    pub async fn all(db: &Surreal<Client>) -> Result<Vec<Studio>, surrealdb::Error> {
        let mut res = db.query("SELECT * FROM studio").await?;
        res.take::<Vec<Studio>>(0)
    }

    pub async fn count(db: &Surreal<Client>) -> Result<i64, surrealdb::Error> {
        let mut res = db.query("SELECT COUNT() FROM studio").await?;
        let count: i64 = res.take::<Vec<i64>>(0)?.first().copied().unwrap_or(0);
        Ok(count)
    }

    pub async fn exists(db: &Surreal<Client>, id: Uuid) -> bool {
        let res: Result<Option<Studio>, surrealdb::Error> =
            db.select(("studio", &id.to_string())).await;
        match res {
            Ok(Some(_)) => true,
            _ => false,
        }
    }

    pub async fn filter(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<Vec<Studio>, surrealdb::Error> {
        let query = format!("SELECT * FROM studio WHERE {} = $value", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<Studio>>(0);
        res
    }

    pub async fn delete(db: &Surreal<Client>, id: Uuid) -> bool {
        let res: Result<Option<Studio>, surrealdb::Error> =
            db.delete(("studio", &id.to_string())).await;
        match res {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
