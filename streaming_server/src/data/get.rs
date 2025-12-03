use crate::data::auth::is_superuser;
use crate::is_logged_in;
use crate::models::Analytics;
use crate::models::Anime;
use crate::models::Episodes;
use crate::models::Logs;
use crate::models::News;
use crate::models::NewsCategory;
use crate::models::Role;
use crate::models::Studio;
use crate::models::Tags;
use crate::models::User;

use crate::utils::{analytics_dummy, get_random_date_and_number};
use crate::{PermissionAction, PermissionModules, has_permission};
use chrono::Utc;
use rocket::http::Status;
use rocket::serde::json::Json;

// IMPORTANT !!
// MOVE USER SESSIONS TO A DIFFERENT TABLE CURRENTLY ITS UNSAFE 
// (i could do it but no time for it for now )
//
//
//
//
//
//



#[get("/anime")]
pub async fn get_all_anime() -> Result<(Status, Json<Vec<Anime>>), String> {
    match Anime::query().order_by("title", "ASC").fetch().await {
        Ok(anime) => Ok((Status::Ok, Json(anime))),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/episodes_secured/<slug>")]
pub async fn get_episodes_by_slug(
    slug: &str,
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Vec<serde_json::Value>>), String> {
    if !has_permission(jar, PermissionModules::EPISODES, PermissionAction::Read)
        .await
        .1
    {
        return Ok((Status::Forbidden, Json(vec![])));
    }

    match Episodes::query()
        .where_slug_eq(slug.to_string())
        .omit(vec![
            "translators.roles.permissions",
            "translators.password",
            "translators.created_at",
            "translators.updated_at",
            "translators.is_superuser",
            "translators.is_staff",
            "translators.is_active",
            "translators.is_banned",
            "translators.session_id",
            "translators.session_expiration",
            "proofreaders.roles.permissions",
            "proofreaders.password",
            "proofreaders.created_at",
            "proofreaders.updated_at",
            "proofreaders.is_superuser",
            "proofreaders.is_staff",
            "proofreaders.is_active",
            "proofreaders.is_banned",
            "proofreaders.session_id",
            "proofreaders.session_expiration",
            "uploaders.roles.permissions",
            "uploaders.password",
            "uploaders.created_at",
            "uploaders.updated_at",
            "uploaders.is_superuser",
            "uploaders.is_staff",
            "uploaders.is_active",
            "uploaders.is_banned",
            "uploaders.session_id",
            "uploaders.session_expiration",
            "typesetters.roles.permissions",
            "typesetters.password",
            "typesetters.created_at",
            "typesetters.updated_at",
            "typesetters.is_superuser",
            "typesetters.is_staff",
            "typesetters.is_active",
            "typesetters.is_banned",
            "typesetters.session_id",
            "typesetters.session_expiration",
        ])
        .fetch_to_value()
        .await
    {
        Ok(episodes) => {
            if episodes.is_empty() {
                Ok((Status::NotFound, Json(vec![])))
            } else {
                Ok((Status::Ok, Json(episodes)))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[get("/episodes")]
pub async fn get_episodes(
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Vec<serde_json::Value>>), String> {
    if !has_permission(jar, PermissionModules::EPISODES, PermissionAction::Read)
        .await
        .1
    {
        match Episodes::query()
            .omit(vec![
                "torrent",
                "subtitles",
                "discord_id",
                "discord_ping",
                "webhook",
                "translators.roles.permissions",
                "translators.password",
                "translators.created_at",
                "translators.updated_at",
                "translators.is_superuser",
                "translators.is_staff",
                "translators.is_active",
                "translators.is_banned",
                "translators.session_id",
                "translators.session_expiration",
                "proofreaders.roles.permissions",
                "proofreaders.password",
                "proofreaders.created_at",
                "proofreaders.updated_at",
                "proofreaders.is_superuser",
                "proofreaders.is_staff",
                "proofreaders.is_active",
                "proofreaders.is_banned",
                "proofreaders.session_id",
                "proofreaders.session_expiration",
                "uploaders.roles.permissions",
                "uploaders.password",
                "uploaders.created_at",
                "uploaders.updated_at",
                "uploaders.is_superuser",
                "uploaders.is_staff",
                "uploaders.is_active",
                "uploaders.is_banned",
                "uploaders.session_id",
                "uploaders.session_expiration",
                "typesetters.roles.permissions",
                "typesetters.password",
                "typesetters.created_at",
                "typesetters.updated_at",
                "typesetters.is_superuser",
                "typesetters.is_staff",
                "typesetters.is_active",
                "typesetters.is_banned",
                "typesetters.session_id",
                "typesetters.session_expiration",
            ])
            .fetch_to_value()
            .await
        {
            Ok(episodes) => Ok((Status::Ok, Json(episodes))),
            Err(e) => Err(e.to_string()),
        }
    } else {
        match Episodes::query()
            .omit(vec![
                "translators.roles.permissions",
                "translators.password",
                "translators.created_at",
                "translators.updated_at",
                "translators.is_superuser",
                "translators.is_staff",
                "translators.is_active",
                "translators.is_banned",
                "translators.session_id",
                "translators.session_expiration",
                "proofreaders.roles.permissions",
                "proofreaders.password",
                "proofreaders.created_at",
                "proofreaders.updated_at",
                "proofreaders.is_superuser",
                "proofreaders.is_staff",
                "proofreaders.is_active",
                "proofreaders.is_banned",
                "proofreaders.session_id",
                "proofreaders.session_expiration",
                "uploaders.roles.permissions",
                "uploaders.password",
                "uploaders.created_at",
                "uploaders.updated_at",
                "uploaders.is_superuser",
                "uploaders.is_staff",
                "uploaders.is_active",
                "uploaders.is_banned",
                "uploaders.session_id",
                "uploaders.session_expiration",
                "typesetters.roles.permissions",
                "typesetters.password",
                "typesetters.created_at",
                "typesetters.updated_at",
                "typesetters.is_superuser",
                "typesetters.is_staff",
                "typesetters.is_active",
                "typesetters.is_banned",
                "typesetters.session_id",
                "typesetters.session_expiration",
            ])
            .fetch_to_value()
            .await
        {
            Ok(episodes) => Ok((Status::Ok, Json(episodes))),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[get("/users")]
pub async fn get_users(
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Vec<serde_json::Value>>), String> {
    if !is_superuser(jar).await.1 {
        match User::query()
            .where_is_active_eq(true)
            .omit(vec![
                "password",
                "session_id",
                "session_expiration",
                "is_superuser",
                "is_staff",
                "is_banned",
                "roles.permissions",
                "roles.created_at",
                "roles.updated_at",
            ])
            .fetch_to_value()
            .await
        {
            Ok(users) => Ok((Status::Ok, Json(users))),
            Err(e) => Err(e.to_string()),
        }
    } else {
        match User::query()
            .omit(vec!["session_id", "session_expiration"])
            .fetch_to_value()
            .await
        {
            Ok(users) => Ok((Status::Ok, Json(users))),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[get("/tags")]
pub async fn get_tags() -> Result<(Status, Json<Vec<Tags>>), String> {
    match Tags::query().fetch().await {
        Ok(tags) => Ok((Status::Ok, Json(tags))),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/studios")]
pub async fn get_studios() -> Result<(Status, Json<Vec<Studio>>), String> {
    match Studio::query().fetch().await {
        Ok(studios) => Ok((Status::Ok, Json(studios))),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/logs")]
pub async fn get_logs(
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Vec<serde_json::Value>>), String> {
    if !has_permission(jar, PermissionModules::LOGS, PermissionAction::Read)
        .await
        .1
    {
        return Ok((Status::Forbidden, Json(vec![])));
    }
    match Logs::query()
        .omit(vec![
            "user_id.session_id",
            "user_id.session_expiration",
            "user_id.password",
        ])
        .fetch_to_value()
        .await
    {
        Ok(logs) => Ok((Status::Ok, Json(logs))),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/permissions_options")]
pub async fn get_permissions_options(
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Vec<String>>), String> {
    let staff = has_permission(jar, PermissionModules::ROLES, PermissionAction::Read).await;
    if !staff.1 {
        return Ok((Status::Forbidden, Json(vec![String::new()])));
    };
    let mut permission_vec = Vec::new();
    for permission in PermissionModules::all() {
        permission_vec.push(permission.to_string());
    }
    Ok((Status::Ok, Json(permission_vec)))
}

#[get("/get_roles")]
pub async fn get_roles(
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Option<Vec<Role>>>), String> {
    let staff = has_permission(jar, PermissionModules::ROLES, PermissionAction::Read).await;
    if !staff.1 {
        return Ok((Status::Forbidden, Json(None)));
    };

    match Role::query().fetch().await {
        Ok(roles) => Ok((Status::Ok, Json(Some(roles)))),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/episodes/<slug>/<number>")]
pub async fn get_episode_by_slug_and_number(
    slug: &str,
    number: &str,
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Option<serde_json::Value>>), String> {
    if !has_permission(jar, PermissionModules::EPISODES, PermissionAction::Read)
        .await
        .1
    {
        return Ok((Status::Forbidden, Json(None)));
    }
    match Episodes::query()
        .where_anime_slug_eq(slug.to_string())
        .where_episode_eq(number.parse::<f64>().unwrap_or(0.0))
        .omit(vec![
            "translators.roles.permissions",
            "translators.password",
            "translators.created_at",
            "translators.updated_at",
            "translators.is_superuser",
            "translators.is_staff",
            "translators.is_active",
            "translators.is_banned",
            "translators.session_id",
            "translators.session_expiration",
            "proofreaders.roles.permissions",
            "proofreaders.password",
            "proofreaders.created_at",
            "proofreaders.updated_at",
            "proofreaders.is_superuser",
            "proofreaders.is_staff",
            "proofreaders.is_active",
            "proofreaders.is_banned",
            "proofreaders.session_id",
            "proofreaders.session_expiration",
            "uploaders.roles.permissions",
            "uploaders.password",
            "uploaders.created_at",
            "uploaders.updated_at",
            "uploaders.is_superuser",
            "uploaders.is_staff",
            "uploaders.is_active",
            "uploaders.is_banned",
            "uploaders.session_id",
            "uploaders.session_expiration",
            "typesetters.roles.permissions",
            "typesetters.password",
            "typesetters.created_at",
            "typesetters.updated_at",
            "typesetters.is_superuser",
            "typesetters.is_staff",
            "typesetters.is_active",
            "typesetters.is_banned",
            "typesetters.session_id",
            "typesetters.session_expiration",
        ])
        .fetch_to_value()
        .await
    {
        Ok(episode) => {
            if episode.is_empty() {
                Ok((Status::NotFound, Json(None)))
            } else {
                Ok((Status::Ok, Json(episode.into_iter().next())))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[get("/anime/<slug>")]
pub async fn get_anime_by_slug(
    slug: &str,
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Option<Anime>>), String> {
    if !has_permission(jar, PermissionModules::ANIME, PermissionAction::Read)
        .await
        .1
    {
        return Ok((Status::Forbidden, Json(None)));
    }
    match Anime::query().where_slug_eq(slug.to_string()).fetch().await {
        Ok(anime) => {
            if anime.is_empty() {
                Ok((Status::NotFound, Json(None)))
            } else {
                Ok((Status::Ok, Json(anime.into_iter().next())))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[get("/news_categories")]
pub async fn get_news_categories(
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Vec<serde_json::Value>>), String> {
    if !has_permission(
        jar,
        PermissionModules::NEWS_CATEGORIES,
        PermissionAction::Read,
    )
    .await
    .1
    {
        match NewsCategory::query()
            .omit(vec!["discord_webhook"])
            .fetch_to_value()
            .await
        {
            Ok(news_categories) => Ok((Status::Ok, Json(news_categories))),
            Err(e) => Err(e.to_string()),
        }
    } else {
        match NewsCategory::query().fetch_to_value().await {
            Ok(news_categories) => Ok((Status::Ok, Json(news_categories))),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[get("/news")]
pub async fn get_news(
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Vec<serde_json::Value>>), String> {
    if !has_permission(jar, PermissionModules::NEWS, PermissionAction::Read)
        .await
        .1
    {
        match News::query()
            .omit(vec![
                "author.password",
                "author.session_id",
                "author.session_expiration",
                "author.roles.permissions",
                "news_category.discord_webhook",
            ])
            .fetch_to_value()
            .await
        {
            Ok(news) => Ok((Status::Ok, Json(news))),
            Err(e) => Err(e.to_string()),
        }
    } else {
        match News::query()
            .omit(vec![
                "author.password",
                "author.session_id",
                "author.session_expiration",
                "author.roles.permissions",
            ])
            .fetch_to_value()
            .await
        {
            Ok(news) => Ok((Status::Ok, Json(news))),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[get("/analytics_anime")]
pub async fn get_analytics_anime(
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Vec<serde_json::Value>>), String> {
    match is_logged_in(jar).await {
        Ok((_, is_logged_in)) => {
            if !is_logged_in {
                return Ok((Status::Forbidden, Json(vec![])));
            }
        }
        Err(_) => return Ok((Status::InternalServerError, Json(vec![]))),
    }
    
    let today = Utc::now().naive_utc().date();
    let one_month_ago = today - chrono::Duration::days(30);
    match Analytics::query()
        .where_kind_eq("anime".to_string())
        .where_date_gte(one_month_ago)
        .fields_to_fetch(vec!["object_id"])
        .await
        .fetch_to_value()
        .await
    {
        Ok(analytics) => Ok((Status::Ok, Json(analytics))),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/analytics_episode")]
pub async fn get_analytics_episode(
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Vec<serde_json::Value>>), String> {
    match is_logged_in(jar).await {
        Ok((_, is_logged_in)) => {
            if !is_logged_in {
                return Ok((Status::Forbidden, Json(vec![])));
            }
        }
        Err(_) => return Ok((Status::InternalServerError, Json(vec![]))),
    }
    let today = Utc::now().naive_utc().date();
    let one_month_ago = today - chrono::Duration::days(30);
    match Analytics::query()
        .where_kind_eq("episodes".to_string())
        .where_date_gte(one_month_ago)
        .fields_to_fetch(vec!["object_id"])
        .await
        .fetch_to_value()
        .await
    {
        Ok(analytics) => Ok((Status::Ok, Json(analytics))),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/analytics_overall")]
pub async fn get_overall_analytics(
    jar: &rocket::http::CookieJar<'_>,
) -> Result<(Status, Json<Vec<serde_json::Value>>), String> {
    match is_logged_in(jar).await {
        Ok((_, is_logged_in)) => {
            if !is_logged_in {
                return Ok((Status::Forbidden, Json(vec![])));
            }
        }
        Err(_) => return Ok((Status::InternalServerError, Json(vec![]))),
    }
    let today = Utc::now().naive_utc().date();
    let one_month_ago = today - chrono::Duration::days(30);
    match Analytics::query()
        .where_kind_eq("main_page".to_string())
        .where_date_gte(one_month_ago)
        .fetch_to_value()
        .await
    {
        Ok(analytics) => Ok((Status::Ok, Json(analytics))),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/dummy_data_for_anime")]
pub async fn get_dummy_data_for_anime() -> Result<(Status, Json<Vec<serde_json::Value>>), String>
{
    let random_anime = Anime::query().fetch().await;
    match random_anime {
        Ok(anime) => {
            let random_anime = anime;
            for n in 0..30 {
                analytics_dummy(None, "anime", random_anime[n as usize].id.clone(), get_random_date_and_number().await.0).await;
            }
            Ok((Status::Ok, Json(vec![])))
        }
        Err(e) => Err(e.to_string()),
    }
}


#[get("/dummy_data_for_overall")]
pub async fn get_dummy_data_for_overall() -> Result<(Status, Json<Vec<serde_json::Value>>), String> {
    for _ in 0..30 {
        analytics_dummy(None, "main_page", surrealdb::sql::Thing::from(("main_page", "main_page")), get_random_date_and_number().await.0).await;
    }
    Ok((Status::Ok, Json(vec![])))
}

