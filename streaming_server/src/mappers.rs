use crate::models::{Anime, Role, User};
use crate::serializers::File;
use chrono::NaiveDateTime;
use juli_orm_core::ForeignKey;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

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
pub struct UserStaff {
    pub id: Thing,
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
    pub roles: Option<Vec<ForeignKey<Role>>>,
    pub slug: String,
}

impl UserStaff {
    pub async fn from_user(user: User) -> Self {
        Self {
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
            roles: if user.roles.is_none() {
                None
            } else {
                if user.roles.clone().unwrap().is_empty() {
                    None
                } else {
                    let r = user.roles.unwrap();
                    let mut roles = vec![];
                    for role in r {
                        match role.load().await {
                            Ok(role) => match role {
                                Some(role) => {
                                    roles.push(ForeignKey::new(role.id));
                                }
                                None => {
                                    eprintln!("Role not found");
                                }
                            },
                            Err(e) => {
                                eprintln!("Error loading role: {}", e);
                            }
                        }
                    }
                    Some(roles)
                }
            },
            slug: user.slug,
        }
    }
    pub fn dummy() -> Self {
        Self {
            id: Thing::from(("user", "1")),
            name: "Dummy".to_string(),
            password: "Dummy".to_string(),
            avatar: None,
            email: None,
            description: None,
            translator: false,
            proofreader: false,
            uploader: false,
            editor: false,
            published: false,
            created_at: None,
            updated_at: None,
            is_superuser: false,
            is_staff: false,
            is_active: false,
            is_banned: false,
            roles: None,
            slug: "dummy".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicEpisodes {
    pub id: surrealdb::sql::Thing,
    pub title: Option<String>,
    pub translators: Option<Vec<ForeignKey<UserPublic>>>,
    pub proofreaders: Option<Vec<ForeignKey<UserPublic>>>,
    pub uploaders: Option<Vec<ForeignKey<UserPublic>>>,
    pub typesetters: Option<Vec<ForeignKey<UserPublic>>>,
    pub anime: ForeignKey<Anime>,
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
pub struct EpisodesStaff {
    pub id: Thing,
    pub title: Option<String>,
    pub discord_id: Option<i64>,
    pub translators: Option<Vec<ForeignKey<UserPublic>>>,
    pub proofreaders: Option<Vec<ForeignKey<UserPublic>>>,
    pub uploaders: Option<Vec<ForeignKey<UserPublic>>>,
    pub typesetters: Option<Vec<ForeignKey<UserPublic>>>,
    pub anime: ForeignKey<Anime>,
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
pub struct EpisodesStaffUpdate {
    pub id: Thing,
    pub title: Option<String>,
    pub translators: Option<Vec<ForeignKey<UserPublic>>>,
    pub proofreaders: Option<Vec<ForeignKey<UserPublic>>>,
    pub uploaders: Option<Vec<ForeignKey<UserPublic>>>,
    pub typesetters: Option<Vec<ForeignKey<UserPublic>>>,
    pub anime: ForeignKey<Anime>,
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
pub struct CreateEpisodesMapper {
    pub title: Option<String>,
    pub discord_id: Option<i64>,
    pub translators: Option<Vec<ForeignKey<UserPublic>>>,
    pub proofreaders: Option<Vec<ForeignKey<UserPublic>>>,
    pub uploaders: Option<Vec<ForeignKey<UserPublic>>>,
    pub typesetters: Option<Vec<ForeignKey<UserPublic>>>,
    pub anime: ForeignKey<Anime>,
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
pub struct EpisodesUpdate {
    pub title: Option<String>,
    pub discord_id: Option<i64>,
    pub translators: Option<Vec<Thing>>,
    pub proofreaders: Option<Vec<Thing>>,
    pub uploaders: Option<Vec<Thing>>,
    pub typesetters: Option<Vec<Thing>>,
    pub anime: Thing,
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

impl From<User> for UserProfileEdit {
    fn from(user: User) -> Self {
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

impl From<UserStaff> for UserProfileEdit {
    fn from(user: UserStaff) -> Self {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewsCategoryPublic {
    pub id: Thing,
    pub name: String,
    pub allowed_everyone: bool,
    pub visible: bool,
    pub slug: String,
    pub date: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewsPublic {
    pub id: Thing,
    pub discord_id: Option<i64>,
    pub author: Option<ForeignKey<UserPublic>>,
    pub website: bool,
    pub category: ForeignKey<NewsCategoryPublic>,
    pub hyperlink: String,
    pub pinned: bool,
    pub color: String,
    pub text_header: String,
    pub thumbnail: File,
    pub image: File,
    pub description: String,
    pub slug: String,
    pub date: Option<NaiveDateTime>,
}
