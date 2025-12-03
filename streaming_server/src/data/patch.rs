use crate::CONFIG;
use crate::data::auth::{is_staff, is_superuser};
use crate::mappers::{EpisodesStaffUpdate, NewsPublic, UserProfileEdit, UserStaff};
use crate::models::{Anime, CreateLogs, Episodes, Logs, News, NewsCategory, Role, User};
use crate::serializers::{
    AddUpdateEpisodeForm, AddUpdateRoleForm, GenericResponse, NewsForm, Password, PasswordChange,
    Response, UniversalUpdateForm,
};
use crate::utils::{diff_structs, get_image_from_formdata, missing_fields};
use crate::{PermissionAction, PermissionModules, has_permission, utils};
use ammonia::Builder;
use juli_orm_core::ForeignKey;
use rocket::form::Form;
use rocket::http::{CookieJar, Status};
use rocket::serde::json;
use rocket::serde::json::Json;
use slugify::slugify;
use surrealdb::sql::Thing;

#[patch("/patch_anime_secured", data = "<form>")]
pub async fn patch_anime_secured(
    jar: &CookieJar<'_>,
    mut form: Form<UniversalUpdateForm<'_>>,
) -> (Status, Json<Response>) {
    let staff = has_permission(jar, PermissionModules::ANIME, PermissionAction::Edit).await;
    if !staff.1 {
        return (
            Status::Forbidden,
            Json(Response {
                status: 403,
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
            }),
        );
    }
    let mut anime_form: Anime = json::from_str(&form.data)
        .map_err(|e| {
            eprintln!("Error parsing anime json: {}", e);
            rocket::http::Status::BadRequest
        })
        .unwrap();

    let id = Thing::from(("anime", anime_form.id.id.to_raw().as_str()));
    let anime = Anime::query()
        .where_id_eq(id)
        .fetch()
        .await
        .expect("Failed to query anime");

    let anime = if anime.is_empty() {
        return (
            Status::NotFound,
            Json(Response {
                status: 404,
                message: GenericResponse::Message("Anime not found".to_string()),
            }),
        );
    } else {
        anime.into_iter().next().unwrap()
    };

    
    
    if let Some(images) = &mut form.image {
        let path_and_name = format!(
            "{}{}/{}/{}_{}.webp",
            CONFIG.directories.media_dir,
            CONFIG.directories.anime_folder,
            CONFIG.directories.anime_thumbnails_folder,
            &anime_form.slug,
            &anime_form.id.id.to_raw()
        );
        match get_image_from_formdata(path_and_name, images).await {
            Ok(image_path) => {
                anime_form.image = image_path;
            }
            Err(e) => {
                eprintln!("Error processing image: {}", e);
                return (
                    Status::InternalServerError,
                    Json(Response {
                        status: 500,
                        message: GenericResponse::Message("Failed to process image".to_string()),
                    }),
                );
            }
        }
    }

    diff_structs(
        &anime,
        &anime_form,
        staff.0.id.id.to_raw(),
        "anime",
        anime.id.id.to_raw(),
    )
    .await;
    let anime_updated = match Anime::manage()
        .merge(anime_form.id.id.to_raw(), anime_form)
        .await
    {
        Ok(anime) => anime,
        Err(e) => {
            eprintln!("Error updating anime: {}", e);
            return (
                Status::InternalServerError,
                Json(Response {
                    status: 500,
                    message: GenericResponse::Message("Failed to update anime".to_string()),
                }),
            );
        }
    };
    (
        Status::Ok,
        Json(Response {
            status: 200,
            message: GenericResponse::Anime(anime_updated),
        }),
    )
}

#[patch("/patch_user_secured", data = "<form>")]
pub async fn patch_user_secured(
    jar: &CookieJar<'_>,
    mut form: Form<UniversalUpdateForm<'_>>,
) -> (Status, Json<Response>) {
    let superuser = is_superuser(jar).await;
    if !superuser.1 {
        return (
            Status::Forbidden,
            Json(Response {
                status: 403,
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
            }),
        );
    }
    let mut user_form: UserStaff = match json::from_str(&form.data) {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Error parsing user json: {}", e);
            return (
                Status::BadRequest,
                Json(Response {
                    status: 400,
                    message: GenericResponse::Message("Invalid user data".to_string()),
                }),
            );
        }
    };

    let user = User::query()
        .where_id_eq(Thing::from(("user", user_form.id.id.to_raw().as_str())))
        .fetch()
        .await
        .expect("Failed to query user");
    let user = if user.is_empty() {
        return (
            Status::NotFound,
            Json(Response {
                status: 404,
                message: GenericResponse::Message("User not found".to_string()),
            }),
        );
    } else {
        user.into_iter().next().unwrap()
    };
    
    if let Some(images) = &mut form.image {
        let path_and_name = format!(
            "{}{}/{}/{}_{}.webp",
            CONFIG.directories.media_dir,
            CONFIG.directories.users_folder,
            CONFIG.directories.avatars_folder,
            &user_form.slug,
            &user_form.id.id.to_raw()
        );
        match get_image_from_formdata(path_and_name, images).await {
            Ok(image_path) => {
                user_form.avatar = Some(image_path);
            }
            Err(e) => {
                eprintln!("Error processing image: {}", e);
                return (
                    Status::InternalServerError,
                    Json(Response {
                        status: 500,
                        message: GenericResponse::Message("Failed to process image".to_string()),
                    }),
                );
            }
        }
    }
    user_form.updated_at = Some(chrono::Utc::now().naive_utc());
    diff_structs(
        &UserStaff::from_user(user.clone()).await,
        &user_form,
        superuser.0.id.id.to_raw(),
        "users",
        user.id.id.to_raw(),
    )
    .await;

    let updated_user = match User::manage()
        .merge(user_form.id.id.to_raw(), user_form)
        .await
    {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Error updating user: {}", e);
            return (
                Status::InternalServerError,
                Json(Response {
                    status: 500,
                    message: GenericResponse::Message("Failed to update user".to_string()),
                }),
            );
        }
    };

    (
        Status::Ok,
        Json(Response {
            status: 200,
            message: GenericResponse::User(UserStaff::from_user(updated_user).await),
        }),
    )
}

#[patch("/patch_episode_secured", data = "<form>")]
pub async fn patch_episode_secured(
    jar: &CookieJar<'_>,
    mut form: Form<AddUpdateEpisodeForm<'_>>,
) -> (Status, Json<Response>) {
    let staff = has_permission(jar, PermissionModules::EPISODES, PermissionAction::Edit).await;
    if !staff.1 {
        return (
            Status::Forbidden,
            Json(Response {
                status: 403,
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
            }),
        );
    }
    let mut episode_form: EpisodesStaffUpdate = match json::from_str(&form.data).map_err(|e| {
        eprintln!("Error parsing episodes json: {}", e);
        rocket::http::Status::BadRequest
    }) {
        Ok(e) => e,
        Err(_e) => {
            return (
                Status::BadRequest,
                Json(Response {
                    message: GenericResponse::Message(
                        "Required fields contains incorrect data".to_string(),
                    ),
                    status: 400,
                }),
            );
        }
    };

    let mut required_fields = Vec::new();

    if episode_form.translators.is_none() {
        required_fields.push("translators");
    }
    if episode_form.proofreaders.is_none() {
        required_fields.push("proofreaders");
    }
    let (validate, status) = missing_fields(required_fields).await;
    if validate {
        return status;
    }
    let episode: Result<Vec<EpisodesStaffUpdate>, juli_orm_core::JuliError> = Episodes::query()
        .where_id_eq(Thing::from((
            "episodes",
            episode_form.id.id.to_raw().as_str(),
        )))
        .fetch_to_own_struct()
        .await;

    let episode = match episode {
        Ok(e) => {
            if e.is_empty() {
                return (
                    Status::NotFound,
                    Json(Response {
                        status: 404,
                        message: GenericResponse::Message("Episode not found".to_string()),
                    }),
                );
            } else {
                e.into_iter().next().unwrap()
            }
        }
        Err(_) => {
            return (
                Status::NotFound,
                Json(Response {
                    status: 404,
                    message: GenericResponse::Message("Episode not found".to_string()),
                }),
            );
        }
    };

    if let Some(images) = &mut form.image {
        let path_and_name = format!(
            "{}{}/{}/{}_{}.webp",
            CONFIG.directories.media_dir,
            CONFIG.directories.episodes_folder,
            CONFIG.directories.episodes_images_folder,
            &episode_form.slug,
            &episode_form.id.id.to_raw()
        );
        match get_image_from_formdata(path_and_name, images).await {
            Ok(image_path) => {
                episode_form.image = image_path;
            }
            Err(e) => {
                eprintln!("Error processing image: {}", e);
                return (
                    Status::InternalServerError,
                    Json(Response {
                        status: 500,
                        message: GenericResponse::Message("Failed to process image".to_string()),
                    }),
                );
            }
        }
    }
    if let Some(torrent) = &mut form.torrent {
        let filename = format!(
            "{}{}/{}/{}_{}_{}.torrent",
            CONFIG.directories.media_dir,
            CONFIG.directories.episodes_folder,
            CONFIG.directories.torrents_folder,
            &episode_form.anime.load().await.unwrap().unwrap().slug,
            &episode_form.anime.id.id.to_raw(),
            &episode_form.episode
        );
        if let Err(e) = torrent.persist_to(&filename).await {
            println!("Failed to save torrent: {}", e);
        } else {
            episode_form.torrent = filename;
        }
    }
    if let Some(subtitles) = &mut form.subtitles {
        let ext = subtitles
            .name()
            .and_then(|name| {
                std::path::Path::new(name)
                    .extension()
                    .and_then(|e| e.to_str())
            })
            .unwrap_or_else(|| {
                println!("Failed to get subtitles extension");
                "ass"
            });
        
        let filename = format!(
            "{}{}/{}/{}_{}_{}.{}",
            CONFIG.directories.media_dir,
            CONFIG.directories.episodes_folder,
            CONFIG.directories.subtitles_folder,
            &episode_form.anime.load().await.unwrap().unwrap().slug,
            &episode_form.anime.id.id.to_raw(),
            &episode_form.episode,
            ext
        );
        if let Err(e) = subtitles.persist_to(&filename).await {
            println!("Failed to save subtitles: {}", e);
        } else {
            episode_form.subtitles = filename;
        }
    }
    diff_structs(
        &episode,
        &episode_form,
        staff.0.id.id.to_raw(),
        "episodes",
        episode.id.id.to_raw(),
    )
    .await;
    match Episodes::manage()
        .merge(episode_form.id.id.to_raw(), episode_form.clone())
        .await
    {
        Ok(episode) => episode,
        Err(e) => {
            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(staff.0.id.clone()),
                    action: format!("Episode updated: {}", episode_form.id.id.to_raw()),
                    description: None,
                    object: "error".to_string(),
                    object_id: episode_form.id.id.to_raw(),
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;
            eprintln!("Error updating episode: {}", e);
            return (
                Status::InternalServerError,
                Json(Response {
                    status: 500,
                    message: GenericResponse::Message("Failed to update episode".to_string()),
                }),
            );
        }
    };

    (
        Status::Ok,
        Json(Response {
            status: 200,
            message: GenericResponse::EpisodeUpdate(episode_form),
        }),
    )
}

#[patch("/patch_user_profile_secured", data = "<form>")]
pub async fn patch_user_profile_secured(
    jar: &CookieJar<'_>,
    mut form: Form<UniversalUpdateForm<'_>>,
) -> (Status, Json<Response>) {
    let user = is_staff(jar).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                status: 403,
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
            }),
        );
    }
    let mut user_form: UserProfileEdit = json::from_str(&form.data)
        .map_err(|e| {
            eprintln!("Error parsing user profile json: {}", e);
            rocket::http::Status::BadRequest
        })
        .unwrap();

    if user.0.id != user_form.id && user.0.password != user_form.password {
        return (
            Status::Forbidden,
            Json(Response {
                status: 403,
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
            }),
        );
    }

    if let Some(images) = &mut form.image {
        let path_and_name = format!(
            "{}{}/{}/{}_{}.webp",
            CONFIG.directories.media_dir,
            CONFIG.directories.users_folder,
            CONFIG.directories.avatars_folder,
            &user.0.slug,
            &user.0.id.id.to_raw()
        );
        match get_image_from_formdata(path_and_name, images).await {
            Ok(image_path) => {
                user_form.avatar = Some(image_path);
            }
            Err(e) => {
                eprintln!("Error processing image: {}", e);
                return (
                    Status::InternalServerError,
                    Json(Response {
                        status: 500,
                        message: GenericResponse::Message("Failed to process image".to_string()),
                    }),
                );
            }
        }
    }
    user_form.updated_at = Some(chrono::Utc::now().naive_utc());
    diff_structs(
        &UserProfileEdit::from(user.0.clone()),
        &user_form,
        user.0.id.id.to_raw(),
        "users",
        user.0.id.id.to_raw(),
    )
    .await;

    let updated_user = match User::manage()
        .merge(user_form.id.id.to_raw(), user_form)
        .await
    {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Error updating user profile: {}", e);
            return (
                Status::InternalServerError,
                Json(Response {
                    status: 500,
                    message: GenericResponse::Message("Failed to update user profile".to_string()),
                }),
            );
        }
    };

    (
        Status::Ok,
        Json(Response {
            status: 200,
            message: GenericResponse::UserProfile(UserProfileEdit::from(updated_user)),
        }),
    )
}

#[patch("/patch_change_password_secured", data = "<form>")]
pub async fn patch_change_password_secured(
    jar: &CookieJar<'_>,
    #[allow(unused_mut)] mut form: Form<UniversalUpdateForm<'_>>,
) -> (Status, Json<Response>) {
    let user = is_staff(jar).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                status: 403,
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
            }),
        );
    }
    let password_form: PasswordChange = json::from_str(&form.data)
        .map_err(|e| {
            eprintln!("Error parsing password json: {}", e);
            rocket::http::Status::BadRequest
        })
        .unwrap();

    if !utils::password_verifier(
        &password_form.old_password.as_bytes(),
        user.0.clone().password,
    ) {
        return (
            Status::Unauthorized,
            Json(Response {
                status: 401,
                message: GenericResponse::Message("Current password is incorrect".to_string()),
            }),
        );
    }

    if password_form.new_password.is_empty() || password_form.new_password.len() < 8 {
        return (
            Status::BadRequest,
            Json(Response {
                status: 400,
                message: GenericResponse::Message(
                    "Password must be at least 8 characters long".to_string(),
                ),
            }),
        );
    }

    let hashed_password = Password {
        password: utils::hash_password(password_form.new_password.as_bytes()),
    };
    let updated_user = match User::manage()
        .merge(user.0.id.id.to_raw(), hashed_password)
        .await
    {
        Ok(user) => {
            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(user.id.clone()),
                    action: format!("User password changed for user: {}", user.name),
                    description: None,
                    object: "users".to_string(),
                    object_id: user.id.id.to_raw(),
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;

            user
        }
        Err(e) => {
            eprintln!("Error updating user password: {}", e);
            return (
                Status::InternalServerError,
                Json(Response {
                    status: 500,
                    message: GenericResponse::Message("Failed to update user password".to_string()),
                }),
            );
        }
    };

    (
        Status::Ok,
        Json(Response {
            status: 200,
            message: GenericResponse::UserProfile(UserProfileEdit::from(updated_user)),
        }),
    )
}

#[patch("/patch_change_password_secured_as_superuser/<id>", data = "<form>")]
pub async fn patch_change_password_secured_as_superuser(
    jar: &CookieJar<'_>,
    id: String,
    #[allow(unused_mut)] mut form: Form<UniversalUpdateForm<'_>>,
) -> (Status, Json<Response>) {
    let superuser = is_superuser(jar).await;
    if !superuser.1 {
        return (
            Status::Forbidden,
            Json(Response {
                status: 403,
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
            }),
        );
    };

    let password = &form.data.clone();

    if password.is_empty() || password.len() < 8 {
        return (
            Status::BadRequest,
            Json(Response {
                status: 400,
                message: GenericResponse::Message(
                    "Password must be at least 8 characters long".to_string(),
                ),
            }),
        );
    };
    let hashed_password = Password {
        password: utils::hash_password(password.as_bytes()),
    };
    let updated_user = match User::manage().merge(id.clone(), hashed_password).await {
        Ok(user) => {
            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(superuser.0.id.clone()),
                    action: format!("User password changed for user: {}", user.name),
                    description: None,
                    object: "users".to_string(),
                    object_id: user.id.id.to_raw(),
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;
            user
        }
        Err(e) => {
            eprintln!("Error updating user password: {}", e);
            return (
                Status::InternalServerError,
                Json(Response {
                    status: 500,
                    message: GenericResponse::Message("Failed to update user password".to_string()),
                }),
            );
        }
    };

    (
        Status::Ok,
        Json(Response {
            status: 200,
            message: GenericResponse::UserProfile(UserProfileEdit::from(updated_user)),
        }),
    )
}

#[patch("/patch_role_secured", data = "<form>")]
pub async fn update_role(
    jar: &CookieJar<'_>,
    mut form: Form<AddUpdateRoleForm<'_>>,
) -> (Status, Json<Response>) {
    let user = has_permission(jar, PermissionModules::ROLES, PermissionAction::Add).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    };
    let mut role_form: Role = json::from_str(&form.data)
        .map_err(|e| {
            eprintln!("Error parsing user json: {}", e);
            rocket::http::Status::BadRequest
        })
        .unwrap();

    for (permission_name, _) in role_form.permissions.clone().iter() {
        if !PermissionModules::all().contains(&permission_name.as_str()) {
            return (
                Status::Forbidden,
                Json(Response {
                    message: GenericResponse::Message("You are not allowed to do this".to_string()),
                    status: 403,
                }),
            );
        }
    }

    if let Some(images) = &mut form.icon {
        let path_and_name = format!(
            "{}{}/{}/icon.webp",
            CONFIG.directories.media_dir,
            CONFIG.directories.users_folder,
            CONFIG.directories.roles_icons_folder
        );
        match get_image_from_formdata(path_and_name, images).await {
            Ok(path) => {
                role_form.icon = path;
            }
            Err(e) => {
                println!("Failed to save image: {}", e);
                return (
                    Status::from_code(500).unwrap(),
                    Json(Response {
                        message: GenericResponse::Message("Failed to save image".to_string()),
                        status: 500,
                    }),
                );
            }
        };
    }
    let role: Result<Vec<Role>, juli_orm_core::JuliError> = Role::query()
        .where_id_eq(Thing::from(("role", role_form.id.id.to_raw().as_str())))
        .fetch()
        .await;
    match role {
        Ok(r) => {
            if r.is_empty() {
                return (
                    Status::NotFound,
                    Json(Response {
                        message: GenericResponse::Message("Role not found".to_string()),
                        status: 404,
                    }),
                );
            } else {
                let mut r = r.into_iter().next().unwrap();
                let log_clone = r.clone();
                r.name = role_form.name.clone();
                r.permissions = role_form.permissions.clone();
                r.administrative_role = role_form.administrative_role.clone();
                r.color = role_form.color.clone();
                r.hierarchy = role_form.hierarchy.clone();
                r.visible = role_form.visible.clone();
                r.icon = role_form.icon.clone();
                let updated_role = Role::manage().merge(r.id.id.to_raw(), r.clone()).await;
                diff_structs(
                    &log_clone,
                    &role_form,
                    user.0.id.id.to_raw(),
                    "role",
                    r.id.id.to_raw(),
                )
                .await;
                
                return (
                    Status::Ok,
                    Json(Response {
                        message: GenericResponse::Role(updated_role.unwrap()),
                        status: 200,
                    }),
                );
            }
        }
        Err(_) => {
            return (
                Status::from_code(500).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Failed to save image".to_string()),
                    status: 500,
                }),
            );
        }
    };
}

#[patch("/patch_news_category_secured", data = "<form>")]
pub async fn patch_news_category_secured(
    jar: &CookieJar<'_>,
    form: Form<UniversalUpdateForm<'_>>,
) -> (Status, Json<Response>) {
    let user = has_permission(
        jar,
        PermissionModules::NEWS_CATEGORIES,
        PermissionAction::Edit,
    )
    .await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    }
    let news_category_form: NewsCategory = json::from_str(&form.data)
        .map_err(|e| {
            eprintln!("Error parsing news category json: {}", e);
            rocket::http::Status::BadRequest
        })
        .unwrap();
    let news_category = NewsCategory::query()
        .where_id_eq(Thing::from((
            "newscategory",
            news_category_form.id.id.to_raw().as_str(),
        )))
        .fetch()
        .await
        .expect("Failed to query news category");
    let news_category = if news_category.is_empty() {
        return (
            Status::NotFound,
            Json(Response {
                message: GenericResponse::Message("News category not found".to_string()),
                status: 404,
            }),
        );
    } else {
        news_category.into_iter().next().unwrap()
    };
    diff_structs(
        &news_category,
        &news_category_form,
        user.0.id.id.to_raw(),
        "newscategory",
        news_category.id.id.to_raw(),
    )
    .await;
    let updated_news_category = match NewsCategory::manage()
        .merge(news_category.id.id.to_raw(), news_category_form)
        .await
    {
        Ok(news_category) => news_category,
        Err(e) => {
            eprintln!("Error updating news category: {}", e);
            return (
                Status::InternalServerError,
                Json(Response {
                    message: GenericResponse::Message("Failed to update news category".to_string()),
                    status: 500,
                }),
            );
        }
    };
    (
        Status::Ok,
        Json(Response {
            status: 200,
            message: GenericResponse::NewsCategory(updated_news_category),
        }),
    )
}

#[patch("/patch_news_secured", data = "<form>")]
pub async fn patch_news_secured(
    jar: &CookieJar<'_>,
    mut form: Form<NewsForm<'_>>,
) -> (Status, Json<Response>) {
    let user = has_permission(jar, PermissionModules::NEWS, PermissionAction::Edit).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    }

    let mut news_form: NewsPublic = match serde_json::from_str(&form.data) {
        Ok(news) => news,
        Err(e) => {
            eprintln!("Error parsing news: {}", e);
            return (
                Status::BadRequest,
                Json(Response {
                    message: GenericResponse::Message("Not implemented yet".to_string()),
                    status: 500,
                }),
            );
        }
    };

    let news: Vec<NewsPublic> = News::query()
        .where_id_eq(Thing::from(("news", news_form.id.id.to_raw().as_str())))
        .fetch_to_own_struct()
        .await
        .expect("Failed to query news");
    let news = if news.is_empty() {
        return (
            Status::NotFound,
            Json(Response {
                message: GenericResponse::Message("News not found".to_string()),
                status: 404,
            }),
        );
    } else {
        news.into_iter().next().unwrap()
    };
    news_form.author = news.author.clone();
    news_form.date = news.date.clone();
    news_form.slug = slugify!(&news_form.text_header, separator = "-");
    news_form.description = Builder::default()
        .add_generic_attributes(&["style"])
        .clean(news_form.description.clone().as_str())
        .to_string();

    

    
    if let Some(images) = &mut form.image {
        let path_and_name = format!(
            "{}{}/{}/image.webp",
            CONFIG.directories.media_dir,
            CONFIG.directories.news_main_folder,
            CONFIG.directories.news_images_folder
        );
        match get_image_from_formdata(path_and_name, images).await {
            Ok(path) => {
                news_form.image = path;
            }
            Err(e) => {
                println!("Failed to save image: {}", e);
                return (
                    Status::from_code(500).unwrap(),
                    Json(Response {
                        message: GenericResponse::Message("Failed to save image".to_string()),
                        status: 500,
                    }),
                );
            }
        }
    }

    if let Some(images) = &mut form.thumbnail {
        let path_and_name = format!(
            "{}{}/{}/thumbnail.webp",
            CONFIG.directories.media_dir,
            CONFIG.directories.news_main_folder,
            CONFIG.directories.news_thumbnails_folder
        );
        match get_image_from_formdata(path_and_name, images).await {
            Ok(path) => {
                news_form.thumbnail = path;
            }
            Err(e) => {
                println!("Failed to save thumbnail: {}", e);
                return (
                    Status::from_code(500).unwrap(),
                    Json(Response {
                        message: GenericResponse::Message("Failed to save thumbnail".to_string()),
                        status: 500,
                    }),
                );
            }
        }
    }

    diff_structs(
        &news,
        &news_form,
        user.0.id.id.to_raw(),
        "news",
        news.id.id.to_raw(),
    )
    .await;

    let _updated_news = match News::manage().merge(news.id.id.to_raw(), news_form).await {
        Ok(news) => news,
        Err(e) => {
            eprintln!("Error updating news: {}", e);
            return (
                Status::InternalServerError,
                Json(Response {
                    message: GenericResponse::Message("Failed to update news".to_string()),
                    status: 500,
                }),
            );
        }
    };

    (
        Status::Ok,
        Json(Response {
            status: 200,
            message: GenericResponse::Message("News updated".to_string()),
        }),
    )
}
