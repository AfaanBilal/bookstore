/**
 * 📕 BookStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/bookstore
 */
use super::Response;
use rocket::State;
use sea_orm::DatabaseConnection;

#[post("/sign-in")]
pub async fn sign_in(db: &State<DatabaseConnection>) -> Response<String> {
    let db = db as &DatabaseConnection;
    todo!()
}

#[post("/sign-up")]
pub async fn sign_up() -> Response<String> {
    todo!()
}
