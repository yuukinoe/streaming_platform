use crate::models::Anime;
use crate::models::Episodes;
use crate::models::User;
use crate::models::{CreateLogs, Logs};
use crate::serializers::{GenericResponse, Response};
use crate::{PermissionAction, PermissionModules, has_permission};
use juli_orm_core::ForeignKey;
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;

#[delete("/delete_anime/<id>")]
pub async fn delete_anime(
    jar: &CookieJar<'_>,
    id: &str,
) -> (Status, Json<Response>) {
    let user = has_permission(jar, PermissionModules::ANIME, PermissionAction::Delete).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    };

    match Anime::manage().delete(id.to_string()).await {
        Ok(_) => {
            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(user.0.id),
                    action: "Deleted this anime object".to_string(),
                    object: "anime".to_string(),
                    object_id: id.to_string(),
                    description: None,
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;
            (
                Status::Ok,
                Json(Response {
                    message: GenericResponse::Message("Successfully deleted anime".to_string()),
                    status: 200,
                }),
            )
        }
        Err(e) => {
            eprintln!("Error deleting anime: {}", e);
            (
                Status::BadRequest,
                Json(Response {
                    message: GenericResponse::Message(
                        "Error while trying to delete anime.".to_string(),
                    ),
                    status: 400,
                }),
            )
        }
    }
}

#[delete("/delete_episode/<id>")]
pub async fn delete_episode(
    jar: &CookieJar<'_>,
    id: &str,
) -> (Status, Json<Response>) {
    let user = has_permission(
        jar,
        PermissionModules::EPISODES,
        PermissionAction::Delete,
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

    return match Episodes::manage().delete(id.to_string()).await {
        Ok(_) => {
            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(user.0.id),
                    action: "Deleted this episode object".to_string(),
                    object: "episodes".to_string(),
                    object_id: id.to_string(),
                    description: None,
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;
            (
                Status::Ok,
                Json(Response {
                    message: GenericResponse::Message("Successfully deleted episode".to_string()),
                    status: 200,
                }),
            )
        }
        Err(e) => (
            Status::BadRequest,
            Json(Response {
                message: GenericResponse::Message(
                    format!("Error while trying to delete episode: {}", e),
                ),
                status: 400,
            }),
        ),
    };
}

#[delete("/delete_user/<id>")]
pub async fn delete_user(
    jar: &CookieJar<'_>,
    id: &str,
) -> (Status, Json<Response>) {
    let user = has_permission(jar, PermissionModules::USERS, PermissionAction::Delete).await;
    if !user.1 {
        return (
            Status::Forbidden,
            Json(Response {
                message: GenericResponse::Message("You are not allowed to do this".to_string()),
                status: 403,
            }),
        );
    };

    return match User::manage().delete(id.to_string()).await {
        Ok(_) => {
            let _ = Logs::manage()
                .create(CreateLogs {
                    user_id: ForeignKey::new(user.0.id),
                    action: "Deleted this users object".to_string(),
                    object: "users".to_string(),
                    object_id: id.to_string(),
                    description: None,
                    date: chrono::Utc::now().naive_utc(),
                })
                .await;
            (
                Status::Ok,
                Json(Response {
                    message: GenericResponse::Message("Successfully deleted user".to_string()),
                    status: 200,
                }),
            )
        }
        Err(e) => (
            Status::BadRequest,
            Json(Response {
                message: GenericResponse::Message(
                    format!("Error while trying to delete user: {}", e),
                ),
                status: 400,
            }),
        ),
    };
}
