use std::collections::HashMap;
use chrono::{NaiveDate, NaiveDateTime};
use juli_orm::{juli_model, juli_register_fields};
use juli_orm_core::ForeignKey;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;


type File = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[juli_register_fields]
#[juli_model(foreign_keys(tags = "Tags", studio = "Studio"))]
pub struct Anime {
    pub id: Thing,
    pub mal: Option<String>,
    pub shinden: Option<String>,
    pub discord_role_id: Option<String>,
    pub background_position: i8,
    pub title: String,
    pub description: Option<String>,
    pub anime_type: Option<String>,
    pub episodes: Option<i32>,
    pub status: Option<String>,
    pub in_progress: bool,
    pub release_time: Option<String>,
    pub aired: Option<NaiveDateTime>,
    pub source: Option<String>,
    pub tags: Option<Vec<ForeignKey<Tags>>>,
    pub studio: Option<Vec<ForeignKey<Studio>>>,
    pub image: crate::serializers::File,
    pub date: Option<NaiveDateTime>,
    pub slug: String,
    pub alternative_title: Option<String>,
}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[juli_register_fields]
#[juli_model(
    table_name = "episodes",
    foreign_keys(
        translators = "User",
        proofreaders = "User",
        uploaders = "User",
        typesetters = "User",
        anime = "Anime"
    )
)]
pub struct Episodes {
    pub id: Thing,
    pub title: Option<String>,
    pub discord_id: Option<i64>,
    pub translators: Option<Vec<ForeignKey<User>>>,
    pub proofreaders: Option<Vec<ForeignKey<User>>>,
    pub uploaders: Option<Vec<ForeignKey<User>>>,
    pub typesetters: Option<Vec<ForeignKey<User>>>,
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
#[juli_register_fields]
#[juli_model(foreign_keys(user_id = "User"))]
pub struct Logs {
    pub id: Thing,
    pub user_id: ForeignKey<User>,
    pub action: String,
    pub object: String,
    pub object_id: String,
    pub description: Option<String>,
    pub date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[juli_register_fields]
#[juli_model()]
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
#[juli_register_fields]
#[juli_model()]
pub struct Studio {
    pub id: Thing,
    pub name: String,
    pub description: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[juli_register_fields]
#[juli_model()]
pub struct Tags {
    pub id: Thing,
    pub name: String,
    pub description: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub slug: String,
}






// IMPORTANT !!
// MOVE USER SESSIONS TO A DIFFERENT TABLE CURRENTLY ITS UNSAFE 
// (i could do it but no time for it for now )
//
//
//
//
//
//




#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[juli_register_fields]
#[juli_model(foreign_keys(roles = "Role"))]
pub struct User {
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
    pub session_id: Option<String>,
    pub session_expiration: Option<NaiveDateTime>,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[juli_register_fields]
#[juli_model()]
pub struct NewsCategory {
    pub id: Thing,
    pub name: String,
    pub discord_webhook: Option<String>,
    pub allowed_everyone: bool,
    pub visible: bool,
    pub slug: String,
    pub date: Option<NaiveDateTime>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[juli_register_fields]
#[juli_model(foreign_keys(category = "NewsCategory"))]
pub struct News {
    pub id: Thing,
    pub discord_id: Option<i64>,
    pub author: Option<ForeignKey<User>>,
    pub website: bool,
    pub category: ForeignKey<NewsCategory>,
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




#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[juli_register_fields]
#[juli_model()]
pub struct Analytics {
    pub id: Thing,             
    pub kind: String,          
    pub object_id: Thing,
    pub date: NaiveDate,      
    pub views: i32,             
    pub ip: Option<String>,
}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PermissionsMap(pub HashMap<String, PermissionFlags>);

impl std::fmt::Display for PermissionsMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for (key, flags) in &self.0 {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key, flags)?;
            first = false;
        }
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PermissionFlags {
    pub add: bool,
    pub read: bool,
    pub edit: bool,
    pub self_action: bool,
    pub post_requests: bool,
    pub delete: bool,
}

impl PermissionsMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: PermissionFlags) -> Option<PermissionFlags> {
        self.0.insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<&PermissionFlags> {
        self.0.get(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, PermissionFlags> {
        self.0.iter()
    }

    pub fn into_iter(self) -> std::collections::hash_map::IntoIter<String, PermissionFlags> {
        self.0.into_iter()
    }
}

impl std::fmt::Display for PermissionFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut flags = Vec::new();
        if self.add {
            flags.push("add");
        }
        if self.read {
            flags.push("read");
        }
        if self.edit {
            flags.push("edit");
        }
        if self.self_action {
            flags.push("self_action");
        }
        if self.post_requests {
            flags.push("post_requests");
        }
        if self.delete {
            flags.push("delete");
        }

        if flags.is_empty() {
            write!(f, "none")
        } else {
            write!(f, "{}", flags.join(", "))
        }
    }
}
