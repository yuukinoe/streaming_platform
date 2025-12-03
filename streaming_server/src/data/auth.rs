use crate::CONFIG;
use crate::mappers::UserStaff;
use crate::models::{CreateUser, User};
use crate::serializers::GenericResponse;
use crate::{
    serializers::{Response, UserPostRequest},
    utils,
};
use chrono::Local;
use chrono::NaiveDateTime;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
use slugify::slugify;

pub async fn update_session_id(mut user: User, session_id: String) -> User {
    use chrono::{Duration, Local};
    user.session_id = Some(session_id);
    user.session_expiration = Some(
        Local::now().naive_local() + Duration::minutes(CONFIG.session.duration_minutes.clone()),
    );
    user.updated_at = Some(Local::now().naive_local());
    match User::manage()
        .merge(user.id.id.to_raw(), user.clone())
        .await
    {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Error updating session id: {}", e);
            user
        }
    }
}

pub fn get_session_id_expiration(user: User) -> Option<(String, NaiveDateTime)> {
    match (user.session_id, user.session_expiration) {
        (Some(id), Some(exp)) => Some((id.clone(), exp)),
        _ => None,
    }
}

#[post("/login", data = "<login>")]
pub async fn login(jar: &CookieJar<'_>, login: Json<UserPostRequest>) -> (Status, Json<Response>) {
    let user = match User::query()
        .where_name_eq(login.username.clone())
        .fetch()
        .await
    {
        Ok(users) => {
            if users.is_empty() {
                return (
                    Status::from_code(401).unwrap(),
                    Json(Response {
                        message: GenericResponse::Message(
                            "Invalid username or password".to_string(),
                        ),
                        status: 401,
                    }),
                );
            } else {
                users.into_iter().next().unwrap()
            }
        }
        Err(e) => {
            eprintln!("Error querying users: {}", e);
            return (
                Status::from_code(500).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Internal server error".to_string()),
                    status: 500,
                }),
            );
        }
    };

    if !utils::password_verifier(login.password.clone().as_bytes(), user.clone().password) {
        return (
            Status::from_code(401).unwrap(),
            Json(Response {
                message: GenericResponse::Message("Invalid username or password".to_string()),
                status: 401,
            }),
        );
    }
    use rocket::http::SameSite;
    use uuid::Uuid;
    
    let session_id = Uuid::new_v4().to_string();
    let user = update_session_id(user, session_id.clone()).await;

    
    if let Some((_, expiration)) = get_session_id_expiration(user) {
        let mut cookie = Cookie::new("session_id", session_id);
        cookie.set_expires(
            rocket::time::OffsetDateTime::from_unix_timestamp(expiration.and_utc().timestamp())
                .unwrap_or_else(|_| rocket::time::OffsetDateTime::now_utc()),
        );
        cookie.set_path("/");
        if CONFIG.debug.clone() {
            cookie.set_same_site(SameSite::Lax);
            println!("Session with expiration date");
        } else {
            cookie.set_same_site(SameSite::None);
        };
        cookie.set_secure(!CONFIG.debug.clone());
        cookie.set_http_only(!CONFIG.debug.clone());

        jar.add(cookie);
    } else {
        let mut cookie = Cookie::new("session_id", session_id);
        cookie.set_path("/");
        jar.add(cookie);
    }
    if CONFIG.debug.clone() {
        println!("success");
    }
    (
        Status::from_code(200).unwrap(),
        Json(Response {
            message: GenericResponse::Message("Login successful".to_string()),
            status: 200,
        }),
    )
}

#[post("/logout")]
pub async fn logout(jar: &CookieJar<'_>) -> (Status, Json<Response>) {
    jar.remove(Cookie::from("session_id"));
    (
        Status::from_code(200).unwrap(),
        Json(Response {
            message: GenericResponse::Message("Logged out successfully".to_string()),
            status: 200,
        }),
    )
}

#[post("/register", data = "<user>")]
pub async fn register(
    jar: &CookieJar<'_>,
    user: Json<UserPostRequest>,
) -> (Status, Json<Response>) {
    let perm = has_permission(jar, PermissionModules::USERS, PermissionAction::Add).await;
    if !perm.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    }

    let mut new_user = user.into_inner();
    if new_user.password.clone().len() < 8 {
        return (
            Status::from_code(400).unwrap(),
            Json(Response {
                message: GenericResponse::Message(
                    "Password must be at least eight characters long".to_string(),
                ),
                status: 400,
            }),
        );
    }
    if new_user.username.clone().len() < 3 {
        return (
            Status::from_code(400).unwrap(),
            Json(Response {
                message: GenericResponse::Message(
                    "Username must be longer than three characters.".to_string(),
                ),
                status: 400,
            }),
        );
    }
    new_user.password = utils::hash_password(new_user.password.as_bytes());
    match User::query()
        .where_name_eq(new_user.username.clone())
        .fetch()
        .await
    {
        Ok(user) => {
            if !user.is_empty() {
                return (
                    Status::from_code(409).unwrap(),
                    Json(Response {
                        message: GenericResponse::Message("Username already exists".to_string()),
                        status: 409,
                    }),
                );
            }
        }
        Err(e) => {
            if CONFIG.debug.clone() {
                eprintln!("Error checking if username exists: {}", e);
            }
        }
    };

    User::create(CreateUser {
        name: new_user.username.clone(),
        password: new_user.password,
        avatar: None,
        email: None,
        description: None,
        translator: false,
        proofreader: false,
        uploader: false,
        editor: false,
        published: true,
        roles: None,
        slug: slugify!(new_user.username.as_str(), separator = "-").to_string(),
        created_at: Some(chrono::Utc::now().naive_utc()),
        updated_at: Some(chrono::Utc::now().naive_utc()),
        is_superuser: false,
        is_staff: false,
        is_active: false,
        is_banned: false,
        session_id: None,
        session_expiration: None,
    })
    .await
    .expect("Failed to create user");
    (
        Status::from_code(200).unwrap(),
        Json(Response {
            message: GenericResponse::Message("success".to_string()),
            status: 200,
        }),
    )
}

#[get("/logged_in")]
pub async fn is_logged_in_get(jar: &CookieJar<'_>) -> (Status, Json<Response>) {
    
    match is_logged_in(jar).await {
        Ok(logged_in) => {
            if logged_in.1 {
                if logged_in.0.is_staff && logged_in.0.is_active && !logged_in.0.is_banned {
                    return (
                        Status::from_code(200).unwrap(),
                        Json(Response {
                            message: GenericResponse::UserStaff(logged_in.0),
                            status: 200,
                        }),
                    );
                }
            }
            return (
                Status::from_code(401).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Not logged in".to_string()),
                    status: 401,
                }),
            );
        }
        Err(e) => {
            if CONFIG.debug.clone() {
                eprintln!("Error checking if user is logged in1: {}", e);
            }
            return (
                Status::from_code(401).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Not logged in".to_string()),
                    status: 401,
                }),
            );
        }
    }
}

pub async fn is_logged_in(jar: &CookieJar<'_>) -> Result<(UserStaff, bool), String> {
    if CONFIG.debug.clone() {
        println!("Checking if user is logged in");
    }
    if let Some(cookie) = jar.get("session_id") {
        let session_id = cookie.value();
        if session_id == "" || session_id == "undefined" {
            return Err("User not logged in".to_string());
        }
        match User::query()
            .where_session_id_eq(Some(session_id.to_string()))
            .fetch()
            .await
        {
            Ok(user) => {
                if !user.is_empty() {
                    let user = user.into_iter().next().unwrap();
                    if let Some(exp) = user.session_expiration {
                        if exp > Local::now().naive_local() {
                            return Ok((
                                User::query()
                                    .where_id_eq(user.id)
                                    .fetch_to_own_struct::<UserStaff>()
                                    .await
                                    .unwrap()
                                    .into_iter()
                                    .next()
                                    .unwrap(),
                                true,
                            ));
                        }
                    }
                }
            }
            Err(e) => {
                if CONFIG.debug.clone() {
                    eprintln!("Error checking if user is logged in2: {}", e);
                }
            }
        }
    }
    Err("User not logged in".to_string())
}

pub async fn is_superuser(jar: &CookieJar<'_>) -> (UserStaff, bool) {
    match is_logged_in(jar).await {
        Ok(user) => {
            if user.1 && user.0.is_superuser {
                return user;
            } else {
                return (UserStaff::dummy(), false);
            }
        }
        Err(e) => {
            if CONFIG.debug.clone() {
                eprintln!("Error checking if user is superuser: {}", e);
            }
            return (UserStaff::dummy(), false);
        }
    }
}

pub async fn is_staff(jar: &CookieJar<'_>) -> (UserStaff, bool) {
    let user = is_logged_in(jar).await;
    match user {
        Ok(user) => {
            if user.1 && user.0.is_staff {
                return user;
            } else {
                return (UserStaff::dummy(), false);
            }
        }
        Err(e) => {
            if CONFIG.debug.clone() {
                eprintln!("Error checking if user is staff: {}", e);
            }
            return (UserStaff::dummy(), false);
        }
    }
}

pub async fn has_permission(
    jar: &CookieJar<'_>,
    module: &str,
    action: PermissionAction,
) -> (UserStaff, bool) {
    let user = is_logged_in(jar).await;
    match user {
        Ok(user) => {
            if user.1 && user.0.is_superuser {
                return (user.0, true);
            }

            if !user.1 {
                return (user.0, false);
            }

            let _roles = &user.0.roles;
            if _roles.is_none() {
                return (user.0, false);
            }

            match user.0.roles.clone() {
                Some(roles) => {
                    for role in roles {
                        if let Ok(Some(loaded_role)) = role.load().await {
                            if let Some(perms) = loaded_role.permissions.get(module) {
                                let allowed = match action {
                                    PermissionAction::Add => perms.add,
                                    PermissionAction::Read => perms.read,
                                    PermissionAction::Edit => perms.edit,
                                    PermissionAction::SelfAction => perms.self_action,
                                    PermissionAction::PostRequest => perms.post_requests,
                                    PermissionAction::Delete => perms.delete,
                                };

                                if allowed {
                                    return (user.0, true);
                                }
                            }
                        }
                    }
                    (user.0, false)
                }
                None => (user.0, false),
            }
        }
        Err(e) => {
            if CONFIG.debug.clone() {
                eprintln!("Error checking if user has permission: {}", e);
            }
            (UserStaff::dummy(), false)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionAction {
    Add,
    Read,
    Edit,
    SelfAction,
    PostRequest,
    Delete,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PermissionModules;

impl PermissionModules {
    pub const USERS: &'static str = "users";
    pub const ANIME: &'static str = "anime";
    pub const EPISODES: &'static str = "episodes";
    pub const TAGS: &'static str = "tags";
    pub const STUDIO: &'static str = "studio";
    pub const LOGS: &'static str = "logs";
    pub const ROLES: &'static str = "roles";
    pub const NEWS_CATEGORIES: &'static str = "news_categories";
    pub const NEWS: &'static str = "news";

    pub fn all() -> &'static [&'static str] {
        &[
            Self::USERS,
            Self::ANIME,
            Self::EPISODES,
            Self::TAGS,
            Self::STUDIO,
            Self::LOGS,
            Self::ROLES,
            Self::NEWS_CATEGORIES,
            Self::NEWS,
        ]
    }
}





























