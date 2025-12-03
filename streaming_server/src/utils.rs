use crate::CONFIG;
use crate::models::{Analytics, CreateAnalytics, CreateLogs, Episodes, Logs, User};
use crate::serializers::{Embed, EmbedFooter, EmbedImage, EmbedThumbnail, WebhookJson};
use crate::serializers::{GenericResponse, Response};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::Utc;
use image::ImageFormat;
use juli_orm_core::ForeignKey;
use rand::Rng;
use reqwest::Client;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::io::AsyncReadExt;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::pin::Pin;
use surrealdb::sql::Thing;
use tokio::io::AsyncWriteExt;
use url::Url;
use webp::Encoder;

pub fn hash_password(password: &[u8]) -> String {
    // let password = passwo; // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    parsed_hash.to_string()
}

pub fn password_verifier(password: &[u8], hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    let status = Argon2::default()
        .verify_password(password, &parsed_hash)
        .is_ok();
    status
}

pub async fn download_and_save_image(
    url: &str,
    folder: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client.get(url).send().await?;
    if !resp.status().is_success() {
        return Err(format!("Failed to download image: {}", resp.status()).into());
    }
    let content_disposition = resp
        .headers()
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split("filename=").nth(1))
        .map(|v| v.trim_matches('"'));
    let url_path = url.split('/').last().unwrap_or("image");
    let ext = Path::new(url_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");
    let mut filename = content_disposition.unwrap_or(url_path).to_string();
    if !filename.ends_with(ext) {
        filename = format!("{}.{}", filename, ext);
    }
    let folder_path = Path::new(folder);
    if !folder_path.exists() {
        fs::create_dir_all(folder_path)?;
    }
    let mut file_path = folder_path.join(&filename);
    let mut i = 1;
    while file_path.exists() {
        let stem = Path::new(&filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image");
        let new_filename = format!("{}_{}.{}", stem, i, ext);
        file_path = folder_path.join(&new_filename);
        i += 1;
    }
    let bytes = resp.bytes().await?;
    let mut file = tokio::fs::File::create(&file_path).await?;
    file.write_all(&bytes).await?;
    Ok(file_path.to_string_lossy().to_string().replace("\\", "/"))
}

pub async fn remove_duplicates(data: Vec<String>) -> Vec<String> {
    let set: HashSet<_> = data.into_iter().collect(); // Collect into a HashSet to remove duplicates
    set.into_iter().collect() // Convert back into a Vec
}

// pub async fn difference<T: PartialEq>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
//     vec1.into_iter()
//         .filter(|item| !vec2.contains(item))
//         .collect()
// }

pub async fn get_image_from_formdata(
    path_and_name: String,
    images: &mut TempFile<'_>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut image_bytes = Vec::new();
    if let Err(e) = images
        .open()
        .await
        .expect("REASON")
        .read_to_end(&mut image_bytes)
        .await
    {
        println!("Failed to read image bytes: {}", e);
    } else {
        let _format = image::guess_format(&image_bytes).unwrap_or(ImageFormat::Jpeg); // fallback
        let img = image::load_from_memory(&image_bytes);

        match img {
            Ok(decoded) => {
                let webp_data = Encoder::from_image(&decoded)
                    .expect("failed to encode to webp")
                    .encode(80f32);
                #[allow(unused_mut)]
                let mut path = Path::new(&path_and_name).to_path_buf();
                let parent = path
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
                    .to_path_buf();
                let stem = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let ext = path
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let mut final_path = path.clone();
                let mut counter = 1;

                while final_path.exists() {
                    let file_name = format!("{}-{}.{}", stem, counter, ext);
                    final_path = parent.join(file_name);
                    counter += 1;
                }

                // dbg!(&final_path);
                if let Err(e) = std::fs::write(&final_path, &*webp_data) {
                    println!("Failed to save webp image: {}", e);
                } else {
                    return Ok(final_path
                        .to_string_lossy()
                        .to_string()
                        .replace("\\", "/")
                        .replace("\\\\", "/"));
                }
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    // dbg!(&path_and_name);
    Ok(path_and_name.replace("\\", "/").replace("\\\\", "/"))
}

// / Function to compare two structs and print the differences
// / First old struct is `a`, second new struct is `b`
pub async fn diff_structs<T: Serialize>(a: &T, b: &T, id: String, object: &str, object_id: String) {
    let a_value = serde_json::to_value(a).unwrap();
    let b_value = serde_json::to_value(b).unwrap();

    diff_values(&a_value, &b_value, "", id, object, object_id).await;
}

pub fn diff_values<'a>(
    a: &'a Value,
    b: &'a Value,
    path: &'a str,
    id: String,
    object: &'a str,
    object_id: String,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
    Box::pin(async move {
        match (a, b) {
            (Value::Object(map_a), Value::Object(map_b)) => {
                for key in map_a
                    .keys()
                    .chain(map_b.keys())
                    .collect::<std::collections::BTreeSet<_>>()
                {
                    let val_a = map_a.get(key);
                    let val_b = map_b.get(key);
                    let new_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };

                    match (val_a, val_b) {
                        (Some(va), Some(vb)) => {
                            diff_values(va, vb, &new_path, id.clone(), object, object_id.clone())
                                .await
                        }
                        (Some(va), None) => {
                            let _ = Logs::manage()
                                .create(CreateLogs {
                                    user_id: ForeignKey::new(Thing::from((
                                        "user",
                                        id.clone().as_str(),
                                    ))),
                                    action: format!("Field `{}` removed: {:?}", new_path, va),
                                    object: object.to_string(),
                                    object_id: object_id.clone(),
                                    description: None,
                                    date: chrono::Utc::now().naive_utc(),
                                })
                                .await;
                        }
                        (None, Some(vb)) => {
                            let _ = Logs::manage()
                                .create(CreateLogs {
                                    user_id: ForeignKey::new(Thing::from((
                                        "user",
                                        id.clone().as_str(),
                                    ))),
                                    action: format!("Field `{}` added: {:?}", new_path, vb),
                                    object: object.to_string(),
                                    object_id: object_id.clone(),
                                    description: None,
                                    date: chrono::Utc::now().naive_utc(),
                                })
                                .await;
                            ()
                        }
                        _ => {}
                    }
                }
            }
            (Value::Array(arr_a), Value::Array(arr_b)) => {
                for i in 0..arr_a.len().max(arr_b.len()) {
                    let va = arr_a.get(i);
                    let vb = arr_b.get(i);
                    let new_path = format!("{}[{}]", path, i);

                    match (va, vb) {
                        (Some(va), Some(vb)) => {
                            diff_values(va, vb, &new_path, id.clone(), object, object_id.clone())
                                .await
                        }
                        (Some(va), None) => {
                            let _ = Logs::manage()
                                .create(CreateLogs {
                                    user_id: ForeignKey::new(Thing::from((
                                        "user",
                                        id.clone().as_str(),
                                    ))),
                                    action: format!("Element `{}` removed: {:?}", new_path, va),
                                    object: object.to_string(),
                                    object_id: object_id.clone(),
                                    description: None,
                                    date: chrono::Utc::now().naive_utc(),
                                })
                                .await;
                            ()
                        }
                        (None, Some(vb)) => {
                            let _ = Logs::manage()
                                .create(CreateLogs {
                                    user_id: ForeignKey::new(Thing::from((
                                        "user",
                                        id.clone().as_str(),
                                    ))),
                                    action: format!("Element `{}` added: {:?}", new_path, vb),
                                    object: object.to_string(),
                                    object_id: object_id.clone(),
                                    description: None,
                                    date: chrono::Utc::now().naive_utc(),
                                })
                                .await;

                            ()
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                if a != b {
                    let _ = Logs::manage()
                        .create(CreateLogs {
                            user_id: ForeignKey::new(Thing::from(("user", id.clone().as_str()))),
                            action: format!("Changes in `{}`: {:?} -> {:?}", path, a, b),
                            object: object.to_string(),
                            object_id: object_id.clone(),
                            description: None,
                            date: chrono::Utc::now().naive_utc(),
                        })
                        .await;
                }
            }
        }
    })
}

pub fn get_domain_name(url_str: &str) -> String {
    match Url::parse(url_str) {
        Ok(parsed_url) => {
            let hostname = parsed_url.host_str().unwrap_or("").replace("www.", "");

            let parts: Vec<&str> = hostname.split('.').collect();

            if parts.len() >= 2 {
                let domain = parts[parts.len() - 2]
                    .to_uppercase()
                    .replace("GOOGLE", "GDRIVE");
                return domain;
            }

            hostname
                .to_uppercase()
                .replace("GOOGLE", "GDRIVE")
                .replace("RPMVID", "RPMV")
                .replace("VIDTUBE", "VT")
                .replace("MOVEARNPRE", "MAP")
        }
        Err(err) => {
            eprintln!("Invalid URL: {}, error: {}", url_str, err);
            String::new()
        }
    }
}

pub async fn get_ip_address(req: &rocket::Request<'_>) -> Option<String> {
    let ip = req
        .client_ip()
        .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
    if ip.is_ipv4() {
        Some(ip.to_string())
    } else {
        None
    }
}

// pub async fn get_user_agent(req: &rocket::Request<'_>) -> Option<String> {
//     let user_agent = req.headers().get("User-Agent");
//     if user_agent.is_empty() {
//         None
//     } else {
//         Some(user_agent.to_string())
//     }
// }

// pub async fn get_referer(req: &rocket::Request<'_>) -> Option<String> {
//     let referer = req.headers().get("Referer");
//     if referer.is_empty() {
//         None
//     } else {
//         Some(referer.to_string())
//     }
// }

pub async fn get_random_date_and_number() -> (chrono::NaiveDate, i64) {
    let mut rng = rand::thread_rng();

    let today = Utc::now().naive_utc().date();

    let random_days = rng.gen_range(0..=30);

    let random_date = today - chrono::Duration::days(random_days);

    let random_number = rng.gen_range(1..=80);

    (random_date, random_number)
}

pub async fn analytics_dummy(
    req: Option<&rocket::Request<'_>>,
    kind: &str,
    object_id: Thing,
    date: chrono::NaiveDate,
) -> Option<String> {
    let analytics = Analytics::query()
        .where_kind_eq(kind.to_string())
        .where_object_id_eq(object_id.clone())
        .where_date_eq(date.into())
        .fetch()
        .await
        .unwrap();
    if analytics.is_empty() {
        let data = CreateAnalytics {
            kind: kind.to_string(),
            object_id: object_id,
            date: date.into(),
            views: get_random_date_and_number().await.1 as i32,
            ip: if let Some(req) = req {
                get_ip_address(req).await
            } else {
                None
            },
        };
        Analytics::manage().create(data).await.unwrap();
    } else {
        let analytics = analytics.first().unwrap();
        let data = CreateAnalytics {
            kind: kind.to_string(),
            object_id: object_id,
            date: date.into(),
            views: analytics.views + get_random_date_and_number().await.1 as i32,
            ip: if let Some(req) = req {
                get_ip_address(req).await
            } else {
                None
            },
        };
        Analytics::manage()
            .merge_force(analytics.id.id.to_raw(), data)
            .await
            .unwrap();
    }
    None
}

pub async fn analytics(
    req: Option<&rocket::Request<'_>>,
    kind: &str,
    object_id: Thing,
) -> Option<String> {
    let analytics = Analytics::query()
        .where_kind_eq(kind.to_string())
        .where_object_id_eq(object_id.clone())
        .where_date_eq(chrono::Utc::now().naive_utc().into())
        .fetch()
        .await
        .unwrap();
    if analytics.is_empty() {
        let data = CreateAnalytics {
            kind: kind.to_string(),
            object_id: object_id,
            date: chrono::Utc::now().naive_utc().into(),
            views: 1,
            ip: if let Some(req) = req {
                get_ip_address(req).await
            } else {
                None
            },
        };
        Analytics::manage().create(data).await.unwrap();
    } else {
        let analytics = analytics.first().unwrap();
        let data = CreateAnalytics {
            kind: kind.to_string(),
            object_id: object_id,
            date: chrono::Utc::now().naive_utc().into(),
            views: analytics.views + 1,
            ip: if let Some(req) = req {
                get_ip_address(req).await
            } else {
                None
            },
        };
        Analytics::manage()
            .merge_force(analytics.id.id.to_raw(), data)
            .await
            .unwrap();
    }
    None
}

fn is_numeric_string(s: String) -> bool {
    s.chars().all(|c| c.is_numeric())
}

pub fn hex_to_decimal_color(hex: &str) -> Option<u32> {
    let cleaned = hex.trim_start_matches('#');

    u32::from_str_radix(cleaned, 16).ok()
}

pub async fn build_webhook_credits(
    translators: Option<&Vec<ForeignKey<User>>>,
    proofreaders: Option<&Vec<ForeignKey<User>>>,
    uploaders: Option<&Vec<ForeignKey<User>>>,
    typesetters: Option<&Vec<ForeignKey<User>>>,
) -> Option<String> {
    let mut lines = Vec::new();

    let mut line1 = Vec::new();
    if let Some(ts) = translators {
        if !ts.is_empty() {
            let mut names = Vec::new();
            for u in ts.iter() {
                let user = u.load().await.unwrap().unwrap();
                names.push(user.name.clone());
            }
            line1.push(format!("Tłumaczenie: {}", names.join(", ")));
        }
    }
    if let Some(ps) = proofreaders {
        if !ps.is_empty() {
            let mut names = Vec::new();
            for u in ps.iter() {
                let user = u.load().await.unwrap().unwrap();
                names.push(user.name.clone());
            }
            line1.push(format!("Korekta: {}", names.join(", ")));
        }
    }
    if !line1.is_empty() {
        lines.push(line1.join(" | "));
    }

    let mut line2 = Vec::new();
    if let Some(us) = uploaders {
        if !us.is_empty() {
            let mut names = Vec::new();
            for u in us.iter() {
                let user = u.load().await.unwrap().unwrap();
                names.push(user.name.clone());
            }
            line2.push(format!("Upload: {}", names.join(", ")));
        }
    }
    if let Some(ts) = typesetters {
        if !ts.is_empty() {
            let mut names = Vec::new();
            for u in ts.iter() {
                let user = u.load().await.unwrap().unwrap();
                names.push(user.name.clone());
            }
            line2.push(format!("Typesetting: {}", names.join(", ")));
        }
    }
    if !line2.is_empty() {
        lines.push(line2.join(" | "));
    }

    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}
pub async fn episodes_webhook(episode: Episodes, reqwest_client: &reqwest::Client) -> i64 {
    let credits = build_webhook_credits(
        episode.translators.as_ref(),
        episode.proofreaders.as_ref(),
        episode.uploaders.as_ref(),
        episode.typesetters.as_ref(),
    )
    .await
    .unwrap_or(String::new());
    let mut video_players = Vec::new();
    video_players.push(format!(
        "{}/anime/{}/episode/{}",
        CONFIG.app.domain,
        episode.anime.load().await.unwrap().unwrap().slug,
        episode.episode as i32
    ));
    video_players.append(&mut episode.video_players.as_ref().unwrap_or(&vec![]).clone());
    let description = format!(
        r#"**{}**
        
**Opis odcinka:**
{}

**Linki do odcinka:**
{}
        "#,
        credits,
        episode.description.unwrap_or(String::new()),
        video_players
            .iter()
            .map(|s| format!("[{}]({})", get_domain_name(s), s))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    if CONFIG.debug {
        dbg!(&description);
        dbg!(&episode.color);
    }


    let embed = Embed {
        embed_type: String::from("rich"),
        url: Some(format!(
            "{}/anime/{}/episode/{}",
            CONFIG.app.domain,
            episode.anime.load().await.unwrap().unwrap().slug,
            episode.episode as i32
        )),
        title: Some(format!(
            "{} - {}",
            episode.anime.load().await.unwrap().unwrap().title,
            episode.episode as i32
        )),
        description: Some(description),
        color: hex_to_decimal_color(episode.color.as_str()).unwrap_or(111111),
        fields: None,
        image: Some(EmbedImage {
            url: format!("{}/{}", CONFIG.app.domain, episode.image).to_string(),
        }),
        thumbnail: Some(EmbedThumbnail {
            url: format!(
                "{}/{}",
                CONFIG.app.domain,
                episode.anime.load().await.unwrap().unwrap().image
            )
            .to_string(),
        }),
        author: None,
        footer: Some(EmbedFooter {
            text: CONFIG.group.name.to_string(),
            icon_url: format!("{}/media/logo.png", CONFIG.app.domain).to_string(),
            proxy_icon_url: format!("{}/media/logo.png", CONFIG.app.domain).to_string(),
        }),
    };
    let role_id = episode.anime.load().await.unwrap().unwrap().discord_role_id;

    let role_id = if role_id.is_some() {
        if role_id.clone().unwrap().len() > 0 {
            if is_numeric_string(role_id.clone().unwrap()) {
                format!("<@&{}>", role_id.clone().unwrap())
            } else {
                role_id.clone().unwrap()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let webhook_payload = WebhookJson {
        username: CONFIG.group.name.clone().into(),
        content: role_id,
        embeds: vec![embed],
    };
    if CONFIG.debug {
        dbg!(&episode.discord_id);
        dbg!(&webhook_payload);
    }
    if episode.discord_id.is_none() || episode.discord_id.unwrap_or(0) == 0 {
        if CONFIG.debug {
            dbg!("Sending webhook");
            dbg!(&CONFIG.discord.episodes_webhook_url);
        }

        let response = reqwest_client
            .post(format!("{}?wait=true", CONFIG.discord.episodes_webhook_url))
            .json(&webhook_payload)
            .send()
            .await;
        let response = match response {
            Ok(resp) => {
                if CONFIG.debug {
                    dbg!(&resp);
                }
                resp
            }
            Err(err) => {
                println!("Error when sending webhook: {}", err);
                return 0;
            }
        };
        let json_result = match response.json::<serde_json::Value>().await {
            Ok(json) => json,
            Err(err) => {
                println!("Error while parsing webhook: {}", err);
                return 0;
            }
        };

        if let Some(id_str) = json_result.get("id").and_then(|v| v.as_str()) {
            if let Ok(id) = id_str.parse::<i64>() {
                return id;
            }
        }
    } else {
        if CONFIG.debug {
            dbg!("Editing webhook");
        }

        let response = reqwest_client
            .patch(format!(
                "{}/messages/{}",
                CONFIG.discord.episodes_webhook_url,
                episode.discord_id.unwrap()
            ))
            .json(&webhook_payload)
            .send()
            .await;
        match response {
            Ok(resp) => resp,
            Err(err) => {
                println!("Error when editing webhook: {}", err);
                return 0;
            }
        };
        return episode.discord_id.unwrap();
    }

    0
}

pub async fn missing_fields(fields: Vec<&str>) -> (bool, (Status, Json<Response>)) {
    if !fields.is_empty() {
        return (
            true,
            (
                Status::BadRequest,
                Json(Response {
                    message: GenericResponse::Message(format!(
                        "Required field(s) missing or incorrect: {}",
                        fields.join(", ")
                    )),
                    status: 400,
                }),
            ),
        );
    };
    return (
        false,
        (
            Status::Ok,
            Json(Response {
                message: GenericResponse::Message("Everything is fine".to_string()),
                status: 200,
            }),
        ),
    );
}
