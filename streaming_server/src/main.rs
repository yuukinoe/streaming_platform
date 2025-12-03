#[macro_use]
extern crate rocket;

use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, CONNECTION, DNT, USER_AGENT};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{CookieJar, Header};
use rocket::{Request, Response};
use std::sync::Arc;
pub mod settings;
pub mod utils;
use juli_orm_core::{DatabaseConfig, initialize_db_with_config};
use rocket::shield::{Frame, Shield};
use teacup_api::*;
pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        
        let origin = req.headers().get_one("Origin");
        let allowed_origin = if let Some(origin) = origin {
            if origin.contains("trycloudflare.com") {
                origin.to_string()
            } else {
                CONFIG.app.domain.clone()
            }
        } else {
            CONFIG.app.domain.clone()
        };
        
        res.set_header(Header::new(
            "Access-Control-Allow-Origin",
            allowed_origin,
        ));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS, UPDATE, DELETE",
        ));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        res.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
    }
}

#[options("/<_..>")]
fn cors_options() -> rocket::http::Status {
    rocket::http::Status::Ok
}

pub async fn request_client(
    reqwest_client: Arc<reqwest::Client>,
    api_url: String,
) -> reqwest::Response {
    reqwest_client
        .get(&api_url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.102 Safari/537.36 OPR/90.0.4480.117",
        )
        .header(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header(ACCEPT_LANGUAGE, "en-US,en;q=0.5")
        .header(DNT, "1")
        .header(CONNECTION, "close")
        .send().await.unwrap()
}

#[get("/me")]
fn me(jar: &CookieJar<'_>) -> Option<String> {
    jar.get("user_id")
        .map(|c| format!("Logged in as user: {}", c.value()))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let config = match load_config("../config.yml") {
        Ok(config) => {
            println!("Config loaded successfully");
            config
        }
        Err(e) => {
            println!("Error loading config: {}", e);
            panic!("Failed to load config");
        }
    };
    let db_config = DatabaseConfig {
        host: config.surrealdb.host,
        port: config.surrealdb.port,
        username: config.surrealdb.user,
        password: config.surrealdb.password,
        namespace: config.surrealdb.namespace,
        database: config.surrealdb.database,
    };
    initialize_db_with_config(db_config).await.unwrap();

    let reqwest_client = reqwest::Client::new();
    let shield = Shield::default().disable::<Frame>();
    rocket::build()
        .attach(CORS)
        .attach(shield)
        .manage(Arc::new(reqwest_client))
        .mount(
            String::from("/") + &format!("{}", CONFIG.directories.media_dir).replace("/", ""),
            rocket::fs::FileServer::from(
                format!("{}", CONFIG.directories.media_dir).replace("/", ""),
            ),
        )
        .mount("/", routes![
            cors_options,
            me,
            index,
            add_anime,
            add_episode,
            get_anime_by_slug,
            update_anime_background_position,
            get_all_anime,
            get_episodes_by_slug,
            get_episodes,
            patch_episode_secured,
            get_episode_by_slug_and_number,
            patch_anime_secured,
            get_users,
            get_studios,
            get_tags,
            get_logs,
            patch_user_secured,
            create_news_category,
            create_news,
            analytics_anime,
            analytics_episode,
            analytics_main_page,
            get_analytics_anime,
            get_analytics_episode,
            get_overall_analytics,
            get_dummy_data_for_overall,
            get_dummy_data_for_anime,
            
            
            patch_user_profile_secured,
            patch_change_password_secured_as_superuser,
            patch_change_password_secured,
            get_news_categories,
            get_news,
            patch_news_secured,
            patch_news_category_secured,
            get_permissions_options,
            get_roles,
            add_role,
            update_role,
            episode_webhook,
            delete_anime,
            delete_user,
            delete_episode,
            
            data::auth::is_logged_in_get,
            data::auth::logout,
            data::auth::register,
            data::auth::login,
        ])
}
