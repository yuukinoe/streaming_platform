use crate::models::users::{UserModel, UserPublic};
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewLog {
    pub user_id: String,
    pub action: String,
    pub object: String,
    pub object_id: String,
    pub description: Option<String>,
    pub date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Logs {
    pub id: Thing,
    pub user_id: String,
    pub action: String,
    pub object: String,
    pub object_id: String,
    pub description: Option<String>,
    pub date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogsPublic {
    pub id: Thing,
    pub user: UserPublic,
    pub action: String,
    pub object: String,
    pub object_id: String,
    pub description: Option<String>,
    pub date: chrono::NaiveDateTime,
}

impl Logs {
    pub async fn new(
        db: &Surreal<Client>,
        user_id: String,
        action: String,
        object: String,
        object_id: String,
    ) -> Self {
        let date = chrono::Local::now().naive_local();
        let new_log = NewLog {
            user_id,
            action,
            description: None,
            object: object.to_lowercase(),
            object_id,
            date,
        };
        let res: Option<Logs> = db
            .create("logs")
            .content(new_log)
            .await
            .expect("Failed to create log")
            .take()
            .unwrap();
        res.unwrap()
    }
    pub async fn new_error(
        db: &Surreal<Client>,
        user_id: String,
        error: String,
        error_place: String,
    ) -> Self {
        Self::new(db, user_id, error, "error".to_string(), error_place).await
    }

    pub async fn update_description(&mut self, db: &Surreal<Client>) {
        let _: Option<Logs> = db
            .update(("logs", &self.id.id.to_raw()))
            .merge(self.clone())
            .await
            .expect("Failed to update log");
    }
    pub async fn get_by_user_id(
        db: &Surreal<Client>,
        #[allow(unused_variables)] user_id: &str,
    ) -> Result<Vec<Logs>, surrealdb::Error> {
        let mut res = db
            .query("SELECT * FROM logs WHERE user_id = $user_id")
            .await?;
        res.take::<Vec<Logs>>(0)
    }
    pub async fn get_all(
        db: &Surreal<surrealdb::engine::remote::ws::Client>,
    ) -> Result<Vec<Logs>, surrealdb::Error> {
        let mut res = db.query("SELECT * FROM logs ORDER BY date DESC").await?;
        res.take::<Vec<Logs>>(0)
    }
    pub async fn get_by_id(
        db: &Surreal<surrealdb::engine::remote::ws::Client>,
        id: &str,
    ) -> Result<LogsPublic, surrealdb::Error> {
        let res: Result<Option<Logs>, surrealdb::Error> =
            db.select(("logs", &id.to_string())).await;

        let log = res?.ok_or(surrealdb::Error::Api(surrealdb::error::Api::Http(
            String::from("Log not found"),
        )))?;

        let user = UserModel::get_public(db, log.user_id).await.unwrap();
        Ok(LogsPublic {
            id: log.id,
            user,
            action: log.action,
            object: log.object,
            object_id: log.object_id,
            description: log.description,
            date: log.date,
        })
    }
    pub async fn delete_all(
        db: &Surreal<surrealdb::engine::remote::ws::Client>,
    ) -> Result<(), surrealdb::Error> {
        db.query("DELETE FROM logs").await?;
        Ok(())
    }

    pub async fn get_all_public(
        db: &Surreal<surrealdb::engine::remote::ws::Client>,
    ) -> Result<Vec<LogsPublic>, surrealdb::Error> {
        let logs = Self::get_all(db).await?;
        let users = UserModel::all_public(db).await?;

        let logs_public = logs
            .into_iter()
            .filter_map(|log| {
                let user_public = users.iter().find(|u| u.id.id.to_raw() == log.user_id);
                user_public.map(|user| LogsPublic {
                    id: log.id,
                    user: user.clone(),
                    action: log.action,
                    object: log.object,
                    object_id: log.object_id,
                    description: log.description,
                    date: log.date,
                })
            })
            .collect();
        Ok(logs_public)
    }

    pub async fn all_insecure(
        db: &Surreal<surrealdb::engine::remote::ws::Client>,
    ) -> Result<Vec<Logs>, surrealdb::Error> {
        let mut res = db.query("SELECT * FROM logs ORDER BY date DESC").await?;
        res.take::<Vec<Logs>>(0)
    }

    pub async fn get_by_object_and_id(
        db: &Surreal<surrealdb::engine::remote::ws::Client>,
        object: &str,
        object_id: &str,
    ) -> Result<Vec<LogsPublic>, surrealdb::Error> {
        let object_lower = object.to_lowercase();
        let mut res = db
            .query("SELECT * FROM logs WHERE object = $object AND object_id = $object_id ORDER BY date DESC")
            .bind(("object", object_lower))
            .bind(("object_id", object_id.to_string()))
            .await?;
        let logs = res.take::<Vec<Logs>>(0)?;
        let users = UserModel::all_public(db).await?;

        let logs_public = logs
            .into_iter()
            .filter_map(|log| {
                let user_public = users.iter().find(|u| u.id.id.to_raw() == log.user_id);
                user_public.map(|user| LogsPublic {
                    id: log.id,
                    user: user.clone(),
                    action: log.action,
                    object: log.object,
                    object_id: log.object_id,
                    description: log.description,
                    date: log.date,
                })
            })
            .collect();
        // dbg!(&logs_public);
        Ok(logs_public)
    }
}
