#![allow(dead_code)]
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
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
                            .route(web::post().to(||{}))
                    )
                    .service(
                        web::resource("/register/{invitation_id}")
                            .route(web::post().to(||{}))
                    )
                    .service(
                        web::resource("/auth")
                            .route(web::get().to(||{}))
                            .route(web::post().to(||{}))
                            .route(web::delete().to(||{}))
                    )
            )
    })
    .bind("127.0.0.1:4000")?
    .run()
}
