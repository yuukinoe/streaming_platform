use crate::serializers::File;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::{Surreal, engine::remote::ws::Client, sql::Thing};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRole {
    pub name: String,
    pub permissions: PermissionsMap,
    pub administrative_role: bool,
    pub visible: bool,
    pub hierarchy: i32,
    pub color: String,
    pub icon: File,
    pub date: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Role {
    pub id: Thing,
    pub name: String,
    pub permissions: PermissionsMap,
    pub administrative_role: bool,
    pub visible: bool,
    pub hierarchy: i32,

    pub color: String,
    pub icon: File,
    pub date: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicRole {
    pub name: String,
    pub administrative_role: bool,
    pub visible: bool,
    pub hierarchy: i32,
    pub color: String,
    pub icon: File,
    pub date: Option<NaiveDateTime>,
}

impl PublicRole {
    pub async fn from(role: Role) -> PublicRole {
        return PublicRole {
            name: role.name,
            administrative_role: role.administrative_role,
            visible: role.visible,
            color: role.color,
            hierarchy: role.hierarchy,
            icon: role.icon,
            date: role.date,
        };
    }
    pub async fn from_option(role: Option<Role>) -> Option<PublicRole> {
        return match role {
            Some(role) => Some(PublicRole::from(role).await),
            _ => None,
        };
    }
    pub async fn from_vec_option_to_vec_option(
        roles: Option<Vec<Role>>,
    ) -> Option<Vec<PublicRole>> {
        return match roles {
            Some(r) => {
                let mut roles_vec = Vec::new();
                for role in r {
                    roles_vec.push(PublicRole::from(role).await);
                }
                Some(roles_vec)
            }
            _ => None,
        };
    }
}

pub type PermissionsMap = HashMap<String, PermissionFlags>;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PermissionFlags {
    pub add: bool,
    pub read: bool,
    pub edit: bool,
    pub self_action: bool,
    pub post_requests: bool,
    pub delete: bool,
}

impl PermissionFlags {}

impl NewRole {
    pub async fn from(t: Role) -> NewRole {
        NewRole {
            name: t.name,
            permissions: t.permissions,
            administrative_role: t.administrative_role,
            visible: t.visible,
            color: t.color,
            hierarchy: t.hierarchy,
            icon: t.icon,
            date: t.date,
        }
    }
}

impl Role {
    pub async fn new(db: &Surreal<Client>, role: NewRole) -> Role {
        let res: Option<Role> = db
            .create("role")
            .content(role)
            .await
            .unwrap()
            .take()
            .unwrap();
        res.unwrap()
    }
    pub async fn get_by_vec(
        db: &Surreal<Client>,
        role_ids: Vec<String>,
    ) -> surrealdb::Result<Vec<Role>> {
        let query = format!(
            "SELECT * FROM [{}] ORDER BY hierarchy ASC;",
            role_ids
                .iter()
                .map(|id| format!("'{id}'"))
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut response = db.query(query).await?;
        let roles: Vec<Role> = response.take(0)?;
        Ok(roles)
    }

    pub async fn get_by_vec_optional(
        db: &Surreal<Client>,
        role_ids: Option<Vec<String>>,
    ) -> surrealdb::Result<Option<Vec<Role>>> {
        let Some(ids) = role_ids else {
            return Ok(None);
        };

        if ids.is_empty() {
            return Ok(None);
        }

        let query = format!(
            "SELECT * FROM [{}] ORDER BY hierarchy ASC;",
            ids.iter()
                .map(|id| format!("'{id}'"))
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut response = db.query(query).await?;
        let roles: Vec<Role> = response.take(0)?;

        if roles.is_empty() {
            Ok(None)
        } else {
            Ok(Some(roles))
        }
    }
    pub async fn get_vec_optional_public(
        db: &Surreal<Client>,
        role_ids: Option<Vec<String>>,
    ) -> surrealdb::Result<Option<Vec<Role>>> {
        let Some(ids) = role_ids else {
            return Ok(None);
        };

        if ids.is_empty() {
            return Ok(None);
        }

        let query = format!(
            "SELECT * FROM [{}] WHERE visible = true ORDER BY hierarchy ASC;",
            ids.iter()
                .map(|id| format!("'{id}'"))
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut response = db.query(query).await?;
        let roles: Vec<Role> = response.take(0)?;

        if roles.is_empty() {
            Ok(None)
        } else {
            Ok(Some(roles))
        }
    }

    pub async fn get_all_filtered_by_vec(
        db: &Surreal<Client>,
    ) -> surrealdb::Result<Option<Vec<Role>>> {
        let mut response = db.query("SELECT * FROM role ORDER BY hierarchy ASC;").await?;
        let all_roles: Vec<Role> = response.take(0)?;

        if all_roles.is_empty() {
            Ok(None)
        } else {
            Ok(Some(all_roles))
        }
    }
    pub async fn get_all_filtered_by_vec_public(
        db: &Surreal<Client>,
    ) -> surrealdb::Result<Option<Vec<Role>>> {
        let mut response = db.query("SELECT * FROM role WHERE visible = true ORDER BY hierarchy ASC;").await?;
        let all_roles: Vec<Role> = response.take(0)?;

        if all_roles.is_empty() {
            Ok(None)
        } else {
            Ok(Some(all_roles))
        }
    }

    pub async fn filter_roles(
        all_roles: Option<Vec<Role>>,
        filter_ids: Option<Vec<String>>,
    ) -> Option<Vec<Role>> {
        let filter_ids = match filter_ids {
            Some(ids) if !ids.is_empty() => ids,
            _ => return None,
        };
        let all_roles= all_roles.unwrap();

        let filtered_roles: Vec<Role> = all_roles
            .into_iter()
            .filter(|role| filter_ids.contains(&role.id.id.to_raw()))
            .collect();
        Some(filtered_roles)
    }

    pub async fn all(
        db: &Surreal<surrealdb::engine::remote::ws::Client>,
    ) -> Result<Vec<Role>, surrealdb::Error> {
        let mut res = db.query("SELECT * FROM role ORDER BY hierarchy ASC").await?;
        res.take::<Vec<Role>>(0)
    }

    pub async fn get(db: &Surreal<Client>, id: String) -> Option<Role> {
        let res: Result<Option<Role>, surrealdb::Error> =
            db.select(("role", &id.to_string())).await;
        res.unwrap_or_else(|_| None)
    }

    pub async fn get_by_field(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<Option<Role>, surrealdb::Error> {
        let query = format!("SELECT * FROM role WHERE {} = $value ORDER BY hierarchy ASC", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<Role>>(0);
        match res {
            Ok(role) => Ok(role.into_iter().next()),
            Err(e) => Err(e),
        }
    }
    pub async fn update(self, db: &Surreal<Client>) -> Role {
        // dbg!(&self);
        let role: Option<Role> = db
            .update(("role", &self.id.id.to_raw()))
            .content(self.clone())
            .await
            .unwrap()
            .take();
        role.unwrap()
    }
    pub async fn delete(db: &Surreal<Client>, id: String) -> bool {
        let res: Result<Option<Role>, surrealdb::Error> =
            db.delete(("role", &id.to_string())).await;
        res.is_ok()
    }

    pub async fn delete_all(db: &Surreal<Client>) -> bool {
        db.query("DELETE FROM role").await.unwrap();
        true
    }
}
