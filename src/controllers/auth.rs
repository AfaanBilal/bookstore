/**
 * 📕 BookStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/bookstore
 */
use super::Response;
use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::DatabaseConnection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignIn {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct ResSignIn {
    token: String,
}

#[post("/sign-in", data = "<req_sign_in>")]
pub async fn sign_in(
    db: &State<DatabaseConnection>,
    req_sign_in: Json<ReqSignIn>,
) -> Response<ResSignIn> {
    let db = db as &DatabaseConnection;
    todo!()
}

#[post("/sign-up")]
pub async fn sign_up() -> Response<String> {
    todo!()
}
