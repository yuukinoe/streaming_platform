use juli_orm_core::{DatabaseConfig, initialize_db_with_config};
#[allow(unused_imports)]
use std::io::{self, *};
use teacup_api::models::{CreateUser, User};
use teacup_api::load_config;
use teacup_api::utils::hash_password;
use slugify::slugify;
#[allow(unused_imports)]
use chrono::NaiveDateTime;

#[allow(unused_imports)]
#[allow(dead_code)]
#[tokio::main]
async fn main() {
    println!("--- Create Superuser ---");
    print!("Username: ");
    stdout().flush().unwrap();
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Password: ");
    stdout().flush().unwrap();
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = password.trim();

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

    let _ = initialize_db_with_config(DatabaseConfig {
        host: config.surrealdb.host,
        port: config.surrealdb.port,
        username: config.surrealdb.user,
        password: config.surrealdb.password,
        namespace: config.surrealdb.namespace,
        database: config.surrealdb.database,
    })
    .await;

    let hashed = hash_password(password.as_bytes());

    let created = User::manage()
        .create(CreateUser {
            name: username.to_string(),
            password: hashed,
            avatar: None,
            email: None,
            description: None,
            translator: false,
            proofreader: false,
            uploader: false,
            editor: false,
            published: true,
            created_at: Some(chrono::Utc::now().naive_utc()),
            updated_at: Some(chrono::Utc::now().naive_utc()),
            is_superuser: true,
            is_staff: true,
            is_active: true,
            is_banned: false,
            roles: None,
            session_id: None,
            session_expiration: None,
            slug: slugify!(format!("{}", username.to_string()).as_str()).to_string(),
        })
        .await;
    println!("Superuser '{}' created!", created.unwrap().name);
}
