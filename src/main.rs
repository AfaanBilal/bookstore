/**
 * 📕 BookStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/bookstore
 */
#[macro_use]
extern crate rocket;

use controllers::{Response, SuccessResponse};
use fairings::cors::{CORS, options};
use migrator::Migrator;
use rocket::http::Status;
use sea_orm_migration::prelude::*;

mod controllers;
mod db;
mod entities;
mod migrator;
mod fairings;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("BOOKSTORE_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("BOOKSTORE_DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("BOOKSTORE_DB_USERNAME").unwrap_or("root".to_string()),
            db_password: std::env::var("BOOKSTORE_DB_PASSWORD").unwrap_or("".to_string()),
            db_database: std::env::var("BOOKSTORE_DB_DATABASE").unwrap_or("bookstore".to_string()),
        }
    }
}

#[get("/")]
fn index() -> Response<String> {
    Ok(SuccessResponse((Status::Ok, "Hello, World".to_string())))
}

#[launch]
async fn rocket() -> _ {
    let config = AppConfig::default();

    let db = db::connect(&config).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    rocket::build()
        .attach(CORS)
        .manage(db)
        .mount("/", routes![options])
        .mount("/", routes![index])
        .mount(
            "/auth",
            routes![controllers::auth::sign_in, controllers::auth::sign_up,],
        )
        .mount(
            "/authors",
            routes![
                controllers::authors::index,
                controllers::authors::create,
                controllers::authors::show,
                controllers::authors::update,
                controllers::authors::delete,
            ],
        )
        .mount(
            "/books",
            routes![
                controllers::books::index,
                controllers::books::create,
                controllers::books::show,
                controllers::books::update,
                controllers::books::delete,
            ],
        )
}
