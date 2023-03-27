/**
 * 📕 BookStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/bookstore
 */
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::*;

use super::{Response, SuccessResponse};
use crate::auth::AuthenticatedUser;
use crate::entities::{book, prelude::*};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBook {
    id: i32,
    author_id: i32,
    title: String,
    year: String,
    cover: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBookList {
    total: usize,
    books: Vec<ResBook>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqBook {
    author_id: i32,
    title: String,
    year: String,
    cover: String,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<ResBookList>> {
    let db = db as &DatabaseConnection;

    let books = Book::find()
        .order_by_desc(book::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(|b| ResBook {
            id: b.id,
            author_id: b.author_id,
            title: b.title.to_owned(),
            year: b.year.to_owned(),
            cover: b.cover.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBookList {
            total: books.len(),
            books,
        }),
    )))
}

#[post("/", data = "<req_book>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_book: Json<ReqBook>,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let book = book::ActiveModel {
        user_id: Set(user.id),
        author_id: Set(req_book.author_id.to_owned()),
        title: Set(req_book.title.to_owned()),
        year: Set(req_book.year.to_owned()),
        cover: Set(req_book.cover.to_owned()),
        ..Default::default()
    };

    let book = book.insert(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(ResBook {
            id: book.id,
            author_id: book.author_id,
            title: book.title,
            year: book.year,
            cover: book.cover,
        }),
    )))
}

#[get("/<id>")]
pub async fn show(id: u32) -> Response<String> {
    todo!()
}

#[put("/<id>")]
pub async fn update(id: u32) -> Response<String> {
    todo!()
}

#[delete("/<id>")]
pub async fn delete(id: u32) -> Response<String> {
    todo!()
}
