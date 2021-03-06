#![allow(dead_code)]
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod schema;
mod models;
mod errors;
mod invitation_handler;
mod register_handler;
mod auth_handler;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool:models::Pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");
    let domain:String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new("1234".repeat(8).as_bytes())
                        .name("auth")
                        .path("/")
                        .domain(domain.as_str())
                        .max_age_time(chrono::Duration::days(1))
                        .secure(false),
                ))
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/invitation")
                            .route(web::post().to_async(invitation_handler::post_invitation))
                    )
                    .service(
                        web::resource("/register/{invitation_id}")
                            .route(web::post().to_async(register_handler::register_user))
                    )
                    .service(
                        web::resource("/auth")
                            .route(web::post().to_async(auth_handler::login))
                            .route(web::get().to_async(auth_handler::user_info))
                            .route(web::delete().to_async(auth_handler::logout))
                    )
            )
    })
    .bind("127.0.0.1:4000")?
    .run()
}
