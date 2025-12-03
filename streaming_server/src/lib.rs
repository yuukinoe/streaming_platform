#[macro_use]
extern crate rocket;

use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, CONNECTION, DNT, USER_AGENT};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::sync::Arc;



pub mod config;
pub mod data;
pub mod mappers;
pub mod models;
pub mod serializers;
pub mod settings;
pub mod utils;



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

























use once_cell::sync::Lazy;
use std::fs;
use serde_yaml;
use crate::config::Config;

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| load_config("../config.yml").expect("Failed to load config"));

pub use data::auth::*;
#[allow(unused_imports)]
pub use data::delete::*;
pub use data::get::*;
pub use data::patch::*;
pub use data::post::*;
#[allow(unused_imports)]
pub use data::update::*;




