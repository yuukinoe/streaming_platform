use crate::models::roles::Role;
use crate::serializers::File;
use crate::settings::SESSION_DURATION_MINUTES;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use surrealdb::sql::Id;
use surrealdb::{Surreal, engine::remote::ws::Client, sql::Thing};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewUserModel {
    pub name: String,
    pub password: String,
    pub avatar: Option<File>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub translator: bool,
    pub proofreader: bool,
    pub uploader: bool,
    pub editor: bool,
    pub published: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub is_active: bool,
    pub is_banned: bool,
    pub roles: Option<Vec<String>>,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserModel {
    pub id: surrealdb::sql::Thing,
    pub name: String,
    pub password: String,
    pub avatar: Option<File>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub translator: bool,
    pub proofreader: bool,
    pub uploader: bool,
    pub editor: bool,
    pub published: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub is_active: bool,
    pub is_banned: bool,
    pub roles: Option<Vec<String>>,
    pub session_id: Option<String>,
    pub session_expiration: Option<NaiveDateTime>,
    pub slug: String,
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
    pub roles: Option<Vec<crate::models::roles::PublicRole>>,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserProfileEdit {
    pub id: Thing,
    pub name: String,
    pub password: String,
    pub avatar: Option<File>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl UserPublic {
    pub async fn from_async(all_roles: Option<Vec<Role>>, user: UserModel) -> Self {
        UserPublic {
            id: user.id,
            name: user.name,
            avatar: user.avatar,
            description: user.description,
            translator: user.translator,
            proofreader: user.proofreader,
            uploader: user.uploader,
            editor: user.editor,
            roles: crate::models::roles::PublicRole::from_vec_option_to_vec_option(
                Role::filter_roles(all_roles, user.roles).await,
            )
            .await,
            slug: user.slug,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserModelSuperUser {
    pub id: surrealdb::sql::Thing,
    pub name: String,
    pub password: String,
    pub avatar: Option<File>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub translator: bool,
    pub proofreader: bool,
    pub uploader: bool,
    pub editor: bool,
    pub published: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub is_active: bool,
    pub is_banned: bool,
    pub roles: Option<Vec<Role>>,
    pub slug: String,
}

impl UserModelSuperUser {
    pub async fn from(all_roles: Option<Vec<Role>>, user: UserModel) -> Self {
        UserModelSuperUser {
            id: user.id,
            name: user.name,
            password: user.password,
            avatar: user.avatar,
            email: user.email,
            description: user.description,
            translator: user.translator,
            proofreader: user.proofreader,
            uploader: user.uploader,
            editor: user.editor,
            published: user.published,
            created_at: user.created_at,
            updated_at: user.updated_at,
            is_superuser: user.is_superuser,
            is_staff: user.is_staff,
            is_active: user.is_active,
            is_banned: user.is_banned,
            roles: Role::filter_roles(all_roles, user.roles).await,
            slug: user.slug,
        }
    }
}

impl UserModel {
    pub async fn new(
        db: &Surreal<Client>,
        name: String,
        password: String,
        avatar: Option<File>,
        email: Option<String>,
        description: Option<String>,
        translator: bool,
        proofreader: bool,
        uploader: bool,
        editor: bool,
        published: bool,
        is_superuser: bool,
        is_staff: bool,
        is_active: bool,
        is_banned: bool,
        roles: Option<Vec<String>>,
    ) -> Self {
        let slug = slugify!(&name, separator = "-");
        let new_user = NewUserModel {
            name,
            password,
            avatar,
            email,
            description,
            translator,
            proofreader,
            uploader,
            editor,
            published,
            created_at: Some(chrono::Local::now().naive_local()),
            updated_at: Some(chrono::Local::now().naive_local()),
            is_superuser,
            is_staff,
            is_active,
            is_banned,
            roles,
            slug,
        };
        let res: Option<UserModel> = db
            .create("user")
            .content(new_user)
            .await
            .unwrap()
            .take()
            .unwrap();
        res.unwrap()
    }

    pub async fn example() -> UserModelSuperUser {
        UserModelSuperUser {
            id: Thing::from(("user", Id::from("ugabuga"))),
            name: "Example User".to_string(),
            password: "password".to_string(),
            avatar: None,
            email: Some("".to_string()),
            description: Some("Example".to_string()),
            translator: false,
            proofreader: false,
            uploader: false,
            editor: false,
            published: false,
            created_at: Some(chrono::Local::now().naive_local()),
            updated_at: Some(chrono::Local::now().naive_local()),
            is_superuser: false,
            is_staff: false,
            is_active: false,
            is_banned: false,
            roles: None,
            slug: "example-user".to_string(),
        }
    }

    pub async fn new_from_struct(
        db: &Surreal<Client>,
        user: UserModel,
    ) -> Result<UserModel, surrealdb::Error> {
        let new_user = NewUserModel {
            name: user.name,
            password: user.password,
            avatar: user.avatar,
            email: user.email,
            description: user.description,
            translator: user.translator,
            proofreader: user.proofreader,
            uploader: user.uploader,
            editor: user.editor,
            published: user.published,
            created_at: user.created_at,
            updated_at: user.updated_at,
            is_superuser: user.is_superuser,
            is_staff: user.is_staff,
            is_active: user.is_active,
            is_banned: user.is_banned,
            roles: user.roles,
            slug: user.slug,
        };
        let res: Option<UserModel> = db.create("user").content(new_user).await?.take().unwrap();
        match res {
            Some(user) => Ok(user),
            None => Err(surrealdb::Error::Api(
                surrealdb::error::Api::InvalidRequest("Failed to create user".into()),
            )),
        }
    }

    pub async fn exists_by_field(db: &Surreal<Client>, field: &str, value: &str) -> bool {
        let query = format!("SELECT * FROM user WHERE {} = $value", field);
        let value = value.to_string();
        let res = db.query(query).bind(("value", value)).await;
        match res {
            Ok(mut result) => {
                let users: Vec<UserModel> = result.take(0).unwrap_or_default();
                !users.is_empty()
            }
            Err(_) => false,
        }
    }

    pub async fn get_public(db: &Surreal<Client>, id: String) -> Option<UserPublic> {
        let res: Result<Option<UserModel>, surrealdb::Error> =
            db.select(("user", &id.to_string())).await;
        let all_roles = Role::get_all_filtered_by_vec_public(db).await.unwrap();
        match res {
            Ok(Some(user)) => Some(UserPublic::from_async(all_roles, user).await),
            _ => None,
        }
    }



    pub async fn save(self, db: &Surreal<Client>) -> UserModel {
        // dbg!(&self);
        let user: Option<UserModel> = db
            .update(("user", &self.id.id.to_raw()))
            .merge(self.clone())
            .await
            .unwrap();
        user.unwrap()
    }

    pub async fn update(
        &mut self,
        db: &Surreal<Client>,
        name: Option<String>,
        password: Option<String>,
        avatar: Option<File>,
        email: Option<String>,
        description: Option<String>,
        translator: Option<bool>,
        proofreader: Option<bool>,
        uploader: Option<bool>,
        editor: Option<bool>,
        published: Option<bool>,
        is_superuser: Option<bool>,
        is_staff: Option<bool>,
        is_active: Option<bool>,
        is_banned: Option<bool>,
    ) -> Self {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(password) = password {
            self.password = password;
        }
        if let Some(avatar) = avatar {
            self.avatar = Some(avatar);
        }
        if let Some(email) = email {
            self.email = Some(email);
        }
        if let Some(description) = description {
            self.description = Some(description);
        }
        if let Some(translator) = translator {
            self.translator = translator;
        }
        if let Some(proofreader) = proofreader {
            self.proofreader = proofreader;
        }
        if let Some(uploader) = uploader {
            self.uploader = uploader;
        }
        if let Some(editor) = editor {
            self.editor = editor;
        }

        if let Some(published) = published {
            self.published = published;
        }
        if let Some(is_superuser) = is_superuser {
            self.is_superuser = is_superuser;
        }
        if let Some(is_staff) = is_staff {
            self.is_staff = is_staff;
        }
        if let Some(is_active) = is_active {
            self.is_active = is_active;
        }
        if let Some(is_banned) = is_banned {
            self.is_banned = is_banned;
        }
        self.updated_at = Some(chrono::Local::now().naive_local());
        self.slug = self.slug.clone();
        let user: Option<UserModel> = db
            .update(("user", &self.id.id.to_raw()))
            .merge(self.clone())
            .await
            .unwrap();
        user.unwrap()
    }

    pub async fn update_password(
        &mut self,
        db: &Surreal<Client>,
        new_password: String,
    ) -> Result<UserModel, surrealdb::Error> {
        self.password = new_password;
        self.updated_at = Some(chrono::Local::now().naive_local());
        let res: Option<UserModel> = db
            .update(("user", &self.id.id.to_raw()))
            .merge(self.clone())
            .await?;
        res.ok_or(surrealdb::Error::Api(
            surrealdb::error::Api::InvalidRequest("Failed to update user password".into()),
        ))
    }

    pub async fn update_from_struct(
        &mut self,
        db: &Surreal<Client>,
        updated_user: UserModelSuperUser,
    ) -> Result<UserModel, surrealdb::Error> {
        self.name = updated_user.name;
        self.avatar = updated_user.avatar;
        self.email = updated_user.email;
        self.description = updated_user.description;
        self.translator = updated_user.translator;
        self.proofreader = updated_user.proofreader;
        self.uploader = updated_user.uploader;
        self.editor = updated_user.editor;
        self.published = updated_user.published;
        self.created_at = updated_user.created_at;
        self.updated_at = updated_user.updated_at;
        self.is_superuser = updated_user.is_superuser;
        self.is_staff = updated_user.is_staff;
        self.is_active = updated_user.is_active;
        self.is_banned = updated_user.is_banned;
        let mut roles = Vec::new();
        for role in updated_user.roles.unwrap_or_default() {
            // dbg!(&role);
            roles.push(role.id.id.to_raw());
        }
        self.roles = Some(roles);
        self.slug = updated_user.slug;
        // dbg!(&self);
        let res: Option<UserModel> = db
            .update(("user", &self.id.id.to_raw()))
            .merge(self.clone())
            .await?;
        res.ok_or(surrealdb::Error::Api(
            surrealdb::error::Api::InvalidRequest("Failed to update user".into()),
        ))
    }

    pub async fn get(db: &Surreal<Client>, id: String) -> Option<UserModel> {
        let res: Result<Option<UserModel>, surrealdb::Error> =
            db.select(("user", &id.to_string())).await;
        res.unwrap_or_else(|_| None)
    }
    pub async fn get_by_field(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<Option<UserModel>, surrealdb::Error> {
        let query = format!("SELECT * FROM user WHERE {} = $value", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<UserModel>>(0);
        match res {
            Ok(users) => Ok(users.into_iter().next()),
            Err(e) => Err(e),
        }
    }
    pub async fn all(db: &Surreal<Client>) -> Result<Vec<UserModel>, surrealdb::Error> {
        let mut res = db.query("SELECT * FROM user").await?;
        res.take::<Vec<UserModel>>(0)
    }

    pub async fn all_active(db: &Surreal<Client>) -> Result<Vec<UserModel>, surrealdb::Error> {
        let mut res = db
            .query("SELECT * FROM user WHERE is_staff = true;")
            .await?;
        res.take::<Vec<UserModel>>(0)
    }

    pub async fn all_public(db: &Surreal<Client>) -> Result<Vec<UserPublic>, surrealdb::Error> {
        let mut res = db.query("SELECT * FROM user").await?;
        let mut users_res = Vec::new();
        let all_roles = Role::get_all_filtered_by_vec_public(db).await?;

        let users = res.take::<Vec<UserModel>>(0)?;
        for user in users {
            users_res.push(UserPublic::from_async(all_roles.clone(), user).await);
        }
        Ok(users_res)
    }

    pub async fn count(db: &Surreal<Client>) -> Result<i64, surrealdb::Error> {
        let mut res = db.query("SELECT COUNT() FROM user").await?;
        let count: i64 = res.take::<Vec<i64>>(0)?.first().copied().unwrap_or(0);
        Ok(count)
    }
    pub async fn exists(db: &Surreal<Client>, id: Uuid) -> bool {
        let res: Result<Option<UserModel>, surrealdb::Error> =
            db.select(("user", &id.to_string())).await;
        match res {
            Ok(Some(_)) => true,
            _ => false,
        }
    }

    pub async fn filter(
        db: &Surreal<Client>,
        field: &str,
        value: &str,
    ) -> Result<Vec<UserModel>, surrealdb::Error> {
        let query = format!("SELECT * FROM user WHERE {} = $value", field);
        let value = value.to_string();
        let res = db
            .query(query)
            .bind(("value", value))
            .await?
            .take::<Vec<UserModel>>(0);
        res
    }

    pub async fn delete(db: &Surreal<Client>, id: &str) -> bool {
        let res: Result<Option<UserModel>, surrealdb::Error> =
            db.delete(("user", &id.to_string())).await;
        match res {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl UserModelSuperUser {
    pub async fn update_from_profile_struct(
        &mut self,
        db: &Surreal<Client>,
        updated_user: UserProfileEdit,
    ) -> Result<UserModelSuperUser, surrealdb::Error> {
        self.name = updated_user.name;
        self.password = updated_user.password;
        self.avatar = updated_user.avatar;
        self.description = updated_user.description;
        self.email = updated_user.email;
        self.created_at = updated_user.created_at;
        self.updated_at = updated_user.updated_at;

        let res: Option<UserModelSuperUser> = db
            .update(("user", &self.id.id.to_raw()))
            .merge(self.clone())
            .await?;
        res.ok_or(surrealdb::Error::Api(
            surrealdb::error::Api::InvalidRequest("Failed to update user".into()),
        ))
    }

    pub async fn update_self_profile_from_struct(
        &mut self,
        db: &Surreal<Client>,
        updated_user: UserProfileEdit,
    ) -> Result<UserModelSuperUser, surrealdb::Error> {
        self.name = updated_user.name;
        self.avatar = updated_user.avatar;
        self.description = updated_user.description;
        self.email = updated_user.email;
        self.updated_at = updated_user.updated_at;

        let res: Option<UserModelSuperUser> = db
            .update(("user", &self.id.id.to_raw()))
            .merge(self.clone())
            .await?;
        res.ok_or(surrealdb::Error::Api(
            surrealdb::error::Api::InvalidRequest("Failed to update user".into()),
        ))
    }

    pub async fn update_password(
        &mut self,
        db: &Surreal<Client>,
        new_password: String,
    ) -> Result<UserModel, surrealdb::Error> {
        self.password = new_password;
        self.updated_at = Some(chrono::Local::now().naive_local());
        let res: Option<UserModel> = db
            .update(("user", &self.id.id.to_raw()))
            .merge(self.clone())
            .await?;
        res.ok_or(surrealdb::Error::Api(
            surrealdb::error::Api::InvalidRequest("Failed to update user password".into()),
        ))
    }
}

impl From<UserModel> for UserProfileEdit {
    fn from(user: UserModel) -> Self {
        UserProfileEdit {
            id: user.id,
            name: user.name,
            password: user.password,
            avatar: user.avatar,
            description: user.description,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<UserModelSuperUser> for UserProfileEdit {
    fn from(user: UserModelSuperUser) -> Self {
        UserProfileEdit {
            id: user.id,
            name: user.name,
            password: user.password,
            avatar: user.avatar,
            description: user.description,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

// impl From<UserModel> for UserPublic {
//     fn from(user: UserModel) -> Self {
//         UserPublic::from(user)
//     }
// }
