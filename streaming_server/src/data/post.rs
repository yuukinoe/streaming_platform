use crate::mappers::{CreateEpisodesMapper, EpisodesStaff, UserPublic};
use crate::models::{
    Anime, CreateAnime, CreateEpisodes, CreateLogs, CreateNews, CreateNewsCategory, CreateRole,
    CreateStudio, CreateTags, Episodes, Logs, News, NewsCategory, Role, Studio, Tags, User,
};
use crate::serializers::{
    AddUpdateEpisodeForm, AddUpdateRoleForm, GenericResponse, MalAnimeData, MalEpisode,
    MalEpisodeData, NewsForm, Response, SingleMessage, UniversalUpdateForm,
    UpdateAnimeBackgroundPositionData, UpdateAnimeBackgroundPositionForm,
};

use crate::utils::{analytics, episodes_webhook, get_image_from_formdata, missing_fields};
use crate::{CONFIG, PermissionAction, PermissionModules, has_permission, request_client};
use ammonia::Builder;
use juli_orm_core::ForeignKey;
use regex::Regex;
use rocket::form::Form;
use rocket::http::{CookieJar, Status};
use rocket::serde::json;
use rocket::serde::json::Json;
use rocket::tokio::fs;
use slugify::slugify;
use std::sync::Arc;
use surrealdb::sql::Thing;
use uuid::Uuid;

#[post("/add_anime", data = "<data>")]
pub async fn add_anime(
    reqwest_client: &rocket::State<Arc<reqwest::Client>>,
    jar: &CookieJar<'_>,
    data: Json<SingleMessage>,
) -> (Status, Json<Response>) {
    let user = has_permission(jar, PermissionModules::ANIME, PermissionAction::Add).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    }

    let mal = &data.message.clone();
    let re: Regex = Regex::new(r"\d+").unwrap();
    let get_id: Vec<&str> = re.find_iter(mal).map(|mat| mat.as_str()).collect();

    if get_id.is_empty() {
        println!("Brak ID w instance_mal");
        return (
            Status::from_code(401).unwrap(),
            Json(Response {
                message: GenericResponse::Message("No numeric id found in mal field".to_string()),
                status: 401,
            }),
        );
    }

    let anime_id = get_id[0];
    let api_url = format!("https://api.jikan.moe/v4/anime/{}", anime_id);

    let res = request_client(reqwest_client.inner().clone(), api_url).await;

    return match res.json::<MalAnimeData>().await {
        Ok(mal_anime_data) => {
            // println!("Fetched anime data successfully");

            let mut tags = mal_anime_data
                .data
                .genres
                .iter()
                .map(|genre| CreateTags {
                    name: genre.name.clone(),
                    description: None,
                    date: Some(chrono::Local::now().naive_local()),
                    slug: slugify!(&genre.name, separator = "-"),
                })
                .collect::<Vec<CreateTags>>();
            let mut studios = mal_anime_data
                .data
                .studios
                .iter()
                .map(|studio| CreateStudio {
                    name: studio.name.clone(),
                    description: None,
                    date: Some(chrono::Local::now().naive_local()),
                    slug: slugify!(&studio.name, separator = "-"),
                })
                .collect::<Vec<CreateStudio>>();
            let mut selected_tags: Vec<ForeignKey<Tags>> = Vec::new();
            let mut selected_studios: Vec<ForeignKey<Studio>> = Vec::new();

            for tag in &mut tags {
                let tag_query = Tags::query().where_slug_eq(tag.slug.clone()).fetch().await;
                match tag_query {
                    Ok(t) => {
                        if t.is_empty() {
                            let new_tag = Tags::manage()
                                .create(tag.clone())
                                .await
                                .expect("Failed to create tag");
                            selected_tags.push(ForeignKey::with_data(new_tag));
                        } else {
                            selected_tags
                                .push(ForeignKey::with_data(t.into_iter().next().unwrap()));
                        }
                    }
                    Err(e) => panic!("Failed to check if tag exists: {}", e),
                }
            }

            for studio in &mut studios {
                let studio_query = Studio::query()
                    .where_slug_eq(studio.slug.clone())
                    .fetch()
                    .await;
                match studio_query {
                    Ok(s) => {
                        if s.is_empty() {
                            let new_studio = Studio::manage()
                                .create(studio.clone())
                                .await
                                .expect("Failed to create studio");

                            selected_studios.push(ForeignKey::with_data(new_studio));
                        } else {
                            selected_studios
                                .push(ForeignKey::with_data(s.into_iter().next().unwrap()));
                        }
                    }
                    Err(e) => panic!("Failed to check if studio exists: {}", e),
                }
            }

            // Determine image URL
            let image_url = match mal_anime_data.data.images {
                Some(ref images) => match images.webp {
                    Some(ref webp) => webp
                        .large_image_url
                        .clone()
                        .or_else(|| webp.small_image_url.clone())
                        .or_else(|| {
                            images.jpg.as_ref().and_then(|jpg| {
                                jpg.large_image_url
                                    .clone()
                                    .or_else(|| jpg.small_image_url.clone())
                            })
                        }),
                    None => images.jpg.as_ref().and_then(|jpg| {
                        jpg.large_image_url
                            .clone()
                            .or_else(|| jpg.small_image_url.clone())
                    }),
                },
                None => None,
            };
            let default_imgur = "https://i.imgur.com/RDwu9ax.png";
            let image = if let Some(ref url) = image_url {
                if url != default_imgur {
                    match crate::utils::download_and_save_image(
                        url,
                        format!(
                            "{}{}/{}",
                            CONFIG.directories.media_dir,
                            CONFIG.directories.anime_folder,
                            CONFIG.directories.anime_thumbnails_folder
                        )
                        .as_str(),
                    )
                    .await
                    {
                        Ok(local_path) => local_path,
                        Err(e) => {
                            println!("Failed to download image: {}", e);
                            default_imgur.to_string()
                        }
                    }
                } else {
                    default_imgur.to_string()
                }
            } else {
                default_imgur.to_string()
            };

            let anime = CreateAnime {
                mal: Some(mal.clone()),
                shinden: None,
                discord_role_id: None,
                background_position: 7,
                title: mal_anime_data.clone().data.title,
                description: None,
                slug: slugify!(&mal_anime_data.clone().data.title, separator = "-"),
                image,
                episodes: Some(mal_anime_data.data.episodes.unwrap_or(99999999) as i32),
                status: mal_anime_data.data.status,
                in_progress: false,
                release_time: Some(format!(
                    "{} {}",
                    mal_anime_data.data.season.unwrap_or(String::from("NULL")),
                    mal_anime_data.data.year.unwrap_or(0)
                )), // Optional field
                aired: Some(
                    mal_anime_data
                        .data
                        .aired
                        .as_ref()
                        .and_then(|aired| aired.from.as_ref())
                        .and_then(|from_str| {
                            chrono::NaiveDateTime::parse_from_str(from_str, "%Y-%m-%dT%H:%M:%S%z")
                                .ok()
                        })
                        .unwrap_or_else(|| chrono::Utc::now().naive_utc()),
                ),
                source: Some(mal_anime_data.data.source.unwrap_or(String::new())),
                tags: Some(selected_tags),
                studio: Some(selected_studios),
                date: Some(chrono::Local::now().naive_local()),
                alternative_title: mal_anime_data.data.title_japanese,
                anime_type: mal_anime_data.data.type_,
            };
            let all_anime = Anime::query()
                .where_mal_contains(anime_id.to_string())
                .fetch()
                .await;
            match all_anime {
                Ok(r) => {
                    if !r.is_empty() {
                        return (
                            Status::from_code(409).unwrap(),
                            Json(Response {
                                message: GenericResponse::Message(
                                    "Anime with this MAL ID already exists".to_string(),
                                ),
                                status: 409,
                            }),
                        );
                    }
                }
                Err(_e) => {}
            }

            let new_anime = Anime::manage().create(anime).await;
            match new_anime {
                Ok(anime) => {
                    let all_anime = Anime::query()
                        .where_mal_contains(anime_id.to_string())
                        .fetch()
                        .await;
                    match all_anime {
                        Ok(r) => {
                            if r.len() > 1 {
                                let _ = Anime::manage().delete(anime.id.id.to_raw()).await;
                                return (
                                    Status::from_code(409).unwrap(),
                                    Json(Response {
                                        message: GenericResponse::Message(
                                            "Anime with this MAL ID already exists".to_string(),
                                        ),
                                        status: 409,
                                    }),
                                );
                            }
                        }
                        Err(_e) => {}
                    }

                    let _ = Logs::manage()
                        .create(CreateLogs {
                            user_id: ForeignKey::new(user.0.id.clone()),
                            action: "Created this anime object".to_string(),
                            description: None,
                            object: "anime".to_string(),
                            object_id: anime.id.id.to_raw(),
                            date: chrono::Utc::now().naive_utc(),
                        })
                        .await;
                    (
                        Status::from_code(200).unwrap(),
                        Json(Response {
                            message: GenericResponse::Anime(anime),
                            status: 200,
                        }),
                    )
                }
                Err(e) => {
                    println!("Failed to add anime: {}", e);
                    (
                        Status::from_code(500).unwrap(),
                        Json(Response {
                            message: GenericResponse::Message("Failed to add anime".to_string()),
                            status: 500,
                        }),
                    )
                }
            }
        }
        Err(e) => {
            println!("Failed to fetch anime data: {}", e);
            return (
                Status::from_code(500).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Failed to fetch anime data".to_string()),
                    status: 500,
                }),
            );
        }
    };
}

#[post("/add_episode", data = "<form>")]
pub async fn add_episode(
    reqwest_client: &rocket::State<Arc<reqwest::Client>>,
    jar: &CookieJar<'_>,
    mut form: Form<AddUpdateEpisodeForm<'_>>,
) -> (Status, Json<Response>) {
    let user = has_permission(jar, PermissionModules::EPISODES, PermissionAction::Add).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    }

    let mut episode_form: CreateEpisodesMapper = match json::from_str(&form.data).map_err(|e| {
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

    let all_episodes = Episodes::query()
        .where_anime_slug_eq(
            episode_form
                .anime
                .load()
                .await
                .unwrap()
                .unwrap()
                .slug
                .clone(),
        )
        .where_episode_eq(episode_form.episode)
        .fetch()
        .await;
    match all_episodes {
        Ok(e) => {
            if !e.is_empty() {
                return (
                    Status::Conflict,
                    Json(Response {
                        message: GenericResponse::Message(
                            "Such episode already exists.".to_string(),
                        ),
                        status: 409,
                    }),
                );
            }
        }
        Err(_e) => {
            println!("Failed to check if episode exists: {}", _e);
        }
    }

    let anime_obj = episode_form.anime.clone();
    let mal = &anime_obj
        .load()
        .await
        .unwrap()
        .unwrap()
        .mal
        .clone()
        .unwrap();
    let re: Regex = Regex::new(r"\d+").unwrap();
    let get_id: Vec<&str> = re.find_iter(mal).map(|mat| mat.as_str()).collect();

    if get_id.is_empty() {
        println!("Brak ID w instance_mal");
        return (
            Status::from_code(401).unwrap(),
            Json(Response {
                message: GenericResponse::Message("No numeric id found in mal field".to_string()),
                status: 401,
            }),
        );
    }

    let anime_id = get_id[0];
    let api_url = format!(
        "https://api.jikan.moe/v4/anime/{}/episodes/{}",
        anime_id,
        if episode_form.episode.fract() == 0.5 {
            episode_form.episode
        } else {
            episode_form.episode as i64 as f64
        }
    );

    let res = request_client(reqwest_client.inner().clone(), api_url).await;

    let mut episode = match res.json::<MalEpisodeData>().await {
        Ok(mal_episode_data) => mal_episode_data.data,
        Err(_e) => MalEpisode::default(),
    };
    let mut episode_slug = episode.title.clone();
    if episode.mal_id == 0 {
        episode.title = episode_form
            .anime
            .load()
            .await
            .unwrap()
            .unwrap()
            .title
            .clone();
        episode_slug = Uuid::new_v4().to_string();
        episode.duration = Some(1420);
        println!("Failed to fetched episode data");
    };

    let anime_slug = anime_obj.load().await.unwrap().unwrap().slug.clone();
    // Handle file uploads

    let episode_num = episode_form.episode;
    // Ensure directories exist
    let _ = fs::create_dir_all(format!(
        "{}{}/{}",
        CONFIG.directories.media_dir,
        CONFIG.directories.episodes_folder,
        CONFIG.directories.episodes_images_folder
    ))
    .await;
    let _ = fs::create_dir_all(format!(
        "{}{}/{}",
        CONFIG.directories.media_dir,
        CONFIG.directories.episodes_folder,
        CONFIG.directories.torrents_folder
    ))
    .await;
    let _ = fs::create_dir_all(format!(
        "{}{}/{}",
        CONFIG.directories.media_dir,
        CONFIG.directories.episodes_folder,
        CONFIG.directories.subtitles_folder
    ))
    .await;

    if let Some(images) = &mut form.image {
        let path_and_name = format!(
            "{}{}/{}/{anime_slug}_{anime_id}_{episode_num}.webp",
            CONFIG.directories.media_dir,
            CONFIG.directories.episodes_folder,
            CONFIG.directories.episodes_images_folder
        );
        match get_image_from_formdata(path_and_name, images).await {
            Ok(path) => {
                episode_form.image = path;
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
    if let Some(torrent) = &mut form.torrent {
        let filename = format!(
            "{}{}/{}/{anime_slug}_{anime_id}_{episode_num}.torrent",
            CONFIG.directories.media_dir,
            CONFIG.directories.episodes_folder,
            CONFIG.directories.torrents_folder
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
        println!("{:?}", subtitles.name());
        let filename = format!(
            "{}{}/{}/{anime_slug}_{anime_id}_{episode_num}.{ext}",
            CONFIG.directories.media_dir,
            CONFIG.directories.episodes_folder,
            CONFIG.directories.subtitles_folder
        );
        if let Err(e) = subtitles.persist_to(&filename).await {
            println!("Failed to save subtitles: {}", e);
        } else {
            episode_form.subtitles = filename;
        }
    }

    let translators: Option<Vec<ForeignKey<User>>> = match episode_form.translators.clone() {
        Some(vec) if !vec.is_empty() => {
            async fn validate_translators(
                vec: Vec<ForeignKey<UserPublic>>,
            ) -> Option<Vec<ForeignKey<User>>> {
                let mut new_translators_vec = Vec::new();
                for t in vec {
                    match User::query().where_id_eq(t.id).fetch().await {
                        Ok(user) if !user.is_empty() => {
                            new_translators_vec
                                .push(ForeignKey::new(user.into_iter().next().unwrap().id));
                        }
                        Ok(_) => {
                            eprintln!("Translator not found");
                            return None;
                        }
                        Err(e) => {
                            eprintln!("Error loading translator: {}", e);
                            return None;
                        }
                    }
                }
                Some(new_translators_vec)
            }

            validate_translators(vec).await
        }
        _ => None,
    };

    let proofreaders: Option<Vec<ForeignKey<User>>> = match episode_form.proofreaders.clone() {
        Some(vec) if !vec.is_empty() => {
            async fn validate_proofreaders(
                vec: Vec<ForeignKey<UserPublic>>,
            ) -> Option<Vec<ForeignKey<User>>> {
                let mut new_proofreaders_vec = Vec::new();
                for p in vec {
                    match User::query().where_id_eq(p.id).fetch().await {
                        Ok(user) if !user.is_empty() => {
                            new_proofreaders_vec
                                .push(ForeignKey::new(user.into_iter().next().unwrap().id));
                        }
                        Ok(_) => {
                            eprintln!("Proofreader not found");
                            return None;
                        }
                        Err(e) => {
                            eprintln!("Error loading proofreader: {}", e);
                            return None;
                        }
                    }
                }
                Some(new_proofreaders_vec)
            }

            validate_proofreaders(vec).await
        }
        _ => None,
    };

    let uploaders = match episode_form.uploaders.clone() {
        Some(vec) if !vec.is_empty() => {
            async fn validate_uploaders(
                vec: Vec<ForeignKey<UserPublic>>,
            ) -> Option<Vec<ForeignKey<User>>> {
                let mut new_uploaders_vec = Vec::new();
                for u in vec {
                    match User::query().where_id_eq(u.id).fetch().await {
                        Ok(user) if !user.is_empty() => {
                            new_uploaders_vec
                                .push(ForeignKey::new(user.into_iter().next().unwrap().id));
                        }
                        Ok(_) => {
                            eprintln!("Uploader not found");
                            return None;
                        }
                        Err(e) => {
                            eprintln!("Error loading uploader: {}", e);
                            return None;
                        }
                    }
                }
                Some(new_uploaders_vec)
            }

            validate_uploaders(vec).await
        }
        _ => None,
    };

    let typesetters = match episode_form.typesetters.clone() {
        Some(vec) if !vec.is_empty() => {
            async fn validate_typesetters(
                vec: Vec<ForeignKey<UserPublic>>,
            ) -> Option<Vec<ForeignKey<User>>> {
                let mut new_typesetters_vec = Vec::new();
                for t in vec {
                    match User::query().where_id_eq(t.id).fetch().await {
                        Ok(user) if !user.is_empty() => {
                            new_typesetters_vec
                                .push(ForeignKey::new(user.into_iter().next().unwrap().id));
                        }
                        Ok(_) => {
                            eprintln!("Typesetter not found");
                            return None;
                        }
                        Err(e) => {
                            eprintln!("Error loading typesetter: {}", e);
                            return None;
                        }
                    }
                }
                Some(new_typesetters_vec)
            }

            validate_typesetters(vec).await
        }
        _ => None,
    };

    let new_episode = CreateEpisodes {
        anime: anime_obj,
        episode: episode_form.episode.clone(),
        title: Some(episode.title.clone()),
        discord_id: None,
        translators: translators,
        proofreaders: proofreaders,
        uploaders: uploaders,
        typesetters: typesetters,
        image: episode_form.image.clone(),
        description: episode_form.description.clone(),
        color: episode_form.color.clone(),
        subtitles: episode_form.subtitles.clone(),
        torrent: episode_form.torrent.clone(),
        video_players: episode_form.video_players.clone(),
        discord_ping: episode_form.discord_ping,
        webhook: episode_form.webhook,
        date: Some(chrono::Local::now().naive_local()),
        length: episode.duration.unwrap_or(1420) as i32 / 60,
        slug: slugify!(&episode_slug, separator = "-"),
    };

    return match Episodes::manage().create(new_episode).await {
        Ok(episode) => {
            // println!("Episode added successfully: {:?}", episode);
            let all_episodes = Episodes::query()
                .where_anime_slug_eq(anime_slug.clone())
                .where_episode_eq(episode_form.episode)
                .fetch()
                .await;
            match all_episodes {
                Ok(e) => {
                    if e.len() > 1 {
                        let _ = Episodes::manage().delete(episode.id.id.to_raw()).await;
                        return (
                            Status::from_code(409).unwrap(),
                            Json(Response {
                                message: GenericResponse::Message(
                                    "Episode with this episode number already exists".to_string(),
                                ),
                                status: 409,
                            }),
                        );
                    }
                }
                Err(_e) => {}
            }

            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(user.0.id.clone()),
                    action: "Created this episode object".to_string(),
                    description: None,
                    object: "episodes".to_string(),
                    object_id: episode.id.id.to_raw(),
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;
            let new_episode: EpisodesStaff = Episodes::query()
                .where_id_eq(Thing::from(("episodes", episode.id.id.to_raw().as_str())))
                .fetch_to_own_struct()
                .await
                .unwrap()
                .into_iter()
                .next()
                .unwrap();
            (
                Status::from_code(200).unwrap(),
                Json(Response {
                    message: GenericResponse::Episode(new_episode),
                    status: 200,
                }),
            )
        }
        Err(e) => {
            println!("Failed to add episode: {}", e);
            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(user.0.id.clone()),
                    action: "Failed to add episode".to_string(),
                    description: Some(e.to_string()),
                    object: "episodes".to_string(),
                    object_id: String::from("None"),
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;
            (
                Status::from_code(500).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Failed to add episode".to_string()),
                    status: 500,
                }),
            )
        }
    };
}

#[post("/add_role", data = "<form>")]
pub async fn add_role(
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

    // let mut role_form_dbg: serde_json::Value = json::from_str(&form.data)
    //     .map_err(|e| {
    //         eprintln!("Error parsing user json: {}", e);
    //         rocket::http::Status::BadRequest
    //     })
    //     .unwrap();
    // dbg!(role_form_dbg);
    #[allow(unused_mut)]
    let mut role_form: Result<CreateRole, _> = serde_json::from_str(&form.data);
    let mut role_form = match role_form {
        Ok(role) => role,
        Err(e) => {
            eprintln!("Error parsing role: {}", e);
            return (
                Status::BadRequest,
                Json(Response {
                    message: GenericResponse::Message("Not implemented yet".to_string()),
                    status: 500,
                }),
            );
        }
    };

    role_form.date = Some(chrono::Local::now().naive_local());
    let permissions_options = PermissionModules::all();
    for (permission_name, _) in role_form.permissions.clone().into_iter() {
        if !permissions_options.contains(&permission_name.as_str()) {
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
        let path_and_name: String = format!(
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

    let role = Role::manage().create(role_form).await;

    let role = match role {
        Ok(role) => role,
        Err(e) => {
            println!("Failed to add role: {}", e);
            return (
                Status::from_code(500).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Failed to add role".to_string()),
                    status: 500,
                }),
            );
        }
    };

    let _ = Logs::manage()
        .create(CreateLogs {
            user_id: ForeignKey::new(user.0.id.clone()),
            action: "Created this role object".to_string(),
            description: None,
            object: "role".to_string(),
            object_id: role.id.id.to_raw(),
            date: chrono::Utc::now().naive_utc(),
        })
        .await;

    return (
        Status::Ok,
        Json(Response {
            message: GenericResponse::Role(role),
            status: 200,
        }),
    );
}

#[post("/episode_webhook/<id>")]
pub async fn episode_webhook(
    reqwest_client: &rocket::State<Arc<reqwest::Client>>,
    jar: &CookieJar<'_>,
    id: String,
) -> (Status, Json<Response>) {
    let user = has_permission(jar, PermissionModules::EPISODES, PermissionAction::Edit).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    };

    let mut episode = match Episodes::query()
        .where_id_eq(Thing::from(("episodes", id.as_str())))
        .fetch()
        .await
        .unwrap()
        .into_iter()
        .next()
    {
        Some(r) => r,
        _ => {
            return (
                Status::Forbidden,
                Json(Response {
                    message: GenericResponse::Message("Episode not found".to_string()),
                    status: 500,
                }),
            );
        }
    };
    let response = episodes_webhook(episode.clone(), reqwest_client).await;

    if response == 0 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("Webhook error".to_string()),
                status: 500,
            }),
        );
    };

    episode.discord_id = Some(response);

    match Episodes::manage()
        .merge(episode.id.id.to_raw(), episode.clone())
        .await
    {
        Ok(_) => (
            Status::Ok,
            Json(Response {
                message: GenericResponse::Message("Webhook sent successfully".to_string()),
                status: 200,
            }),
        ),
        _ => (
            Status::from_code(500).unwrap(),
            Json(Response {
                message: GenericResponse::Message("Failed to save episode".to_string()),
                status: 500,
            }),
        ),
    }
}

#[post("/add_news_category", data = "<form>")]
pub async fn add_news_category(
    jar: &CookieJar<'_>,
    form: Form<UniversalUpdateForm<'_>>,
) -> (Status, Json<Response>) {
    let user = has_permission(
        jar,
        PermissionModules::NEWS_CATEGORIES,
        PermissionAction::Add,
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
    };
    let mut news_category: CreateNewsCategory = match serde_json::from_str(&form.data) {
        Ok(news_category) => news_category,
        Err(e) => {
            eprintln!("Error parsing news category: {}", e);
            return (
                Status::BadRequest,
                Json(Response {
                    message: GenericResponse::Message("Not implemented yet".to_string()),
                    status: 500,
                }),
            );
        }
    };
    news_category.date = Some(chrono::Local::now().naive_local());
    news_category.slug = slugify!(&news_category.name, separator = "-");

    let news_category = NewsCategory::manage().create(news_category).await;

    let news_category = match news_category {
        Ok(news_category) => {
            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(user.0.id.clone()),
                    action: "Created this news category object".to_string(),
                    description: None,
                    object: "newscategory".to_string(),
                    object_id: news_category.id.id.to_raw(),
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;
            news_category
        }
        Err(e) => {
            println!("Failed to add news category: {}", e);
            return (
                Status::from_code(500).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Failed to add news category".to_string()),
                    status: 500,
                }),
            );
        }
    };

    return (
        Status::Ok,
        Json(Response {
            message: GenericResponse::NewsCategory(news_category),
            status: 200,
        }),
    );
}

#[post("/add_news", data = "<form>")]
pub async fn add_news(
    jar: &CookieJar<'_>,
    mut form: Form<NewsForm<'_>>,
) -> (Status, Json<Response>) {
    let user = has_permission(jar, PermissionModules::NEWS, PermissionAction::Add).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    };

    let mut news: CreateNews = match serde_json::from_str(&form.data) {
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

    news.date = Some(chrono::Local::now().naive_local());
    news.slug = slugify!(&news.text_header, separator = "-");

    if let Some(images) = &mut form.image {
        let path_and_name = format!(
            "{}{}/{}/image.webp",
            CONFIG.directories.media_dir,
            CONFIG.directories.news_main_folder,
            CONFIG.directories.news_images_folder
        );
        match get_image_from_formdata(path_and_name, images).await {
            Ok(path) => {
                news.image = path;
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
                news.thumbnail = path;
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

    let news = News::manage().create(news).await;

    let news = match news {
        Ok(news) => {
            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(user.0.id.clone()),
                    action: "Created this news object".to_string(),
                    description: None,
                    object: "news".to_string(),
                    object_id: news.id.id.to_raw(),
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;
            news
        }
        Err(e) => {
            println!("Failed to add news: {}", e);
            return (
                Status::from_code(500).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Failed to add news".to_string()),
                    status: 500,
                }),
            );
        }
    };

    return (
        Status::Ok,
        Json(Response {
            message: GenericResponse::News(news),
            status: 200,
        }),
    );
}

#[post("/update_anime_background_position", data = "<form>")]
pub async fn update_anime_background_position(
    jar: &CookieJar<'_>,
    form: Form<UpdateAnimeBackgroundPositionForm>,
) -> (Status, Json<Response>) {
    let user = has_permission(jar, PermissionModules::ANIME, PermissionAction::Edit).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    }

    Anime::manage()
        .merge(form.id.to_string(), UpdateAnimeBackgroundPositionData {
            background_position: form.background_position,
        })
        .await
        .expect("Failed to update anime background position");

    return (
        Status::Ok,
        Json(Response {
            message: GenericResponse::Message("Anime background position updated".to_string()),
            status: 200,
        }),
    );
}

#[post("/create_news_category", data = "<form>")]
pub async fn create_news_category(
    jar: &CookieJar<'_>,
    form: Form<UniversalUpdateForm<'_>>,
) -> (Status, Json<Response>) {
    let user = has_permission(
        jar,
        PermissionModules::NEWS_CATEGORIES,
        PermissionAction::Add,
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

    let news_category: CreateNewsCategory = match serde_json::from_str(&form.data) {
        Ok(news_category) => news_category,
        Err(e) => {
            eprintln!("Error parsing news category: {}", e);
            return (
                Status::BadRequest,
                Json(Response {
                    message: GenericResponse::Message("Not implemented yet".to_string()),
                    status: 500,
                }),
            );
        }
    };

    // dbg!(&form.data);
    // return (
    //     Status::Ok,
    //     Json(Response {
    //         message: GenericResponse::Message("News category created".to_string()),
    //         status: 200,
    //     }),
    // );

    match NewsCategory::query()
        .where_slug_eq(news_category.slug.clone())
        .fetch()
        .await
    {
        Ok(news_category) => {
            if !news_category.is_empty() {
                return (
                    Status::Conflict,
                    Json(Response {
                        message: GenericResponse::Message(
                            "News category already exists".to_string(),
                        ),
                        status: 409,
                    }),
                );
            }
        }
        Err(_e) => {
            println!("Failed to check if news category exists: {}", _e);
        }
    }

    let news_category = NewsCategory::manage().create(news_category).await;

    let news_category = match news_category {
        Ok(news_category) => {
            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(user.0.id.clone()),
                    action: "Created this news category object".to_string(),
                    description: None,
                    object: "newscategory".to_string(),
                    object_id: news_category.id.id.to_raw(),
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;
            news_category
        }
        Err(e) => {
            println!("Failed to add news category: {}", e);
            return (
                Status::from_code(500).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Failed to add news category".to_string()),
                    status: 500,
                }),
            );
        }
    };

    return (
        Status::Ok,
        Json(Response {
            message: GenericResponse::NewsCategory(news_category),
            status: 200,
        }),
    );
}

#[post("/create_news", data = "<form>")]
pub async fn create_news(
    jar: &CookieJar<'_>,
    mut form: Form<NewsForm<'_>>,
) -> (Status, Json<Response>) {
    let user = has_permission(jar, PermissionModules::NEWS, PermissionAction::Add).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    }

    let mut news: CreateNews = match serde_json::from_str(&form.data) {
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
    // dbg!(&news);
    // return (
    //     Status::Ok,
    //     Json(Response {
    //         message: GenericResponse::Message("News created".to_string()),
    //         status: 200,
    //     }),
    // );

    // process image and thumbnail
    if let Some(images) = &mut form.image {
        let path_and_name = format!(
            "{}{}/{}/image.webp",
            CONFIG.directories.media_dir,
            CONFIG.directories.news_main_folder,
            CONFIG.directories.news_images_folder
        );
        match get_image_from_formdata(path_and_name, images).await {
            Ok(path) => {
                news.image = path;
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
                news.thumbnail = path;
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

    news.author = Some(ForeignKey::new(user.0.id.clone()));
    news.description = Builder::default()
        .add_generic_attributes(&["style"])
        .clean(news.description.clone().as_str())
        .to_string();
    news.date = Some(chrono::Local::now().naive_local());
    news.slug = slugify!(&news.text_header, separator = "-");

    let news = News::manage().create(news).await;

    let news = match news {
        Ok(news) => {
            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(user.0.id.clone()),
                    action: "Created this news object".to_string(),
                    description: None,
                    object: "news".to_string(),
                    object_id: news.id.id.to_raw(),
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;
            news
        }
        Err(e) => {
            println!("Failed to add news: {}", e);
            return (
                Status::from_code(500).unwrap(),
                Json(Response {
                    message: GenericResponse::Message("Failed to add news".to_string()),
                    status: 500,
                }),
            );
        }
    };

    return (
        Status::Ok,
        Json(Response {
            message: GenericResponse::News(news),
            status: 200,
        }),
    );
}

#[post("/analytics_anime/<slug>")]
pub async fn analytics_anime(slug: String) {
    let anime = Anime::query().where_slug_eq(slug.to_string()).fetch().await;
    if let Ok(anime) = anime {
        if !anime.is_empty() {
            let _ = analytics(None, "anime", anime.first().unwrap().id.clone()).await;
        }
    }
}

#[post("/analytics_episode/<slug>/<number>")]
pub async fn analytics_episode(slug: String, number: String) {
    let episode = Episodes::query()
        .where_anime_slug_eq(slug.to_string())
        .where_episode_eq(number.parse::<f64>().unwrap_or(0.0))
        .fetch()
        .await;
    if let Ok(episode) = episode {
        if !episode.is_empty() {
            let _ = analytics(None, "episodes", episode.first().unwrap().id.clone()).await;
        }
    }
}

#[post("/analytics_main_page")]
pub async fn analytics_main_page() {
    let _ = analytics(None, "main_page", Thing::from(("main_page", "mainpage"))).await;
}
