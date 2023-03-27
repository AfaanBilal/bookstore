/**
 * 📕 BookStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/bookstore
 */
use super::{Response, SuccessResponse};
use crate::auth::{AuthenticatedUser, Claims};
use crate::controllers::ErrorResponse;
use crate::entities::{prelude::*, user};
use crate::AppConfig;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::*;
use std::time::SystemTime;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignIn {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResSignIn {
    token: String,
}

#[post("/sign-in", data = "<req_sign_in>")]
pub async fn sign_in(
    db: &State<DatabaseConnection>,
    config: &State<AppConfig>,
    req_sign_in: Json<ReqSignIn>,
) -> Response<Json<ResSignIn>> {
    let db = db as &DatabaseConnection;
    let config = config as &AppConfig;

    let u: user::Model = match User::find()
        .filter(user::Column::Email.eq(&req_sign_in.email))
        .one(db)
        .await?
    {
        Some(u) => u,
        None => {
            return Err(ErrorResponse((
                Status::Unauthorized,
                "Invalid credentials".to_string(),
            )))
        }
    };

    if !verify(&req_sign_in.password, &u.password).unwrap() {
        return Err(ErrorResponse((
            Status::Unauthorized,
            "Invalid credentials".to_string(),
        )));
    }

    let claims = Claims {
        sub: u.id,
        role: "user".to_string(),
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 4 * 60 * 60,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .unwrap();

    Ok(SuccessResponse((Status::Ok, Json(ResSignIn { token }))))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignUp {
    email: String,
    password: String,
    firstname: Option<String>,
    lastname: Option<String>,
}

#[post("/sign-up", data = "<req_sign_up>")]
pub async fn sign_up(
    db: &State<DatabaseConnection>,
    req_sign_up: Json<ReqSignUp>,
) -> Response<String> {
    let db = db as &DatabaseConnection;

    if User::find()
        .filter(user::Column::Email.eq(&req_sign_up.email))
        .one(db)
        .await?
        .is_some()
    {
        return Err(ErrorResponse((
            Status::UnprocessableEntity,
            "An account exists with that email address.".to_string(),
        )));
    }

    User::insert(user::ActiveModel {
        email: Set(req_sign_up.email.to_owned()),
        password: Set(hash(req_sign_up.password.to_owned(), DEFAULT_COST).unwrap()),
        firstname: Set(req_sign_up.firstname.to_owned()),
        lastname: Set(req_sign_up.lastname.to_owned()),
        ..Default::default()
    })
    .exec(db)
    .await?;

    Ok(SuccessResponse((
        Status::Created,
        "Account created!".to_string(),
    )))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResMe {
    id: i32,
    email: String,
    firstname: Option<String>,
    lastname: Option<String>,
}

#[get("/me")]
pub async fn me(db: &State<DatabaseConnection>, user: AuthenticatedUser) -> Response<Json<ResMe>> {
    let db = db as &DatabaseConnection;

    let u: user::Model = User::find_by_id(user.id).one(db).await?.unwrap();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResMe {
            id: u.id,
            email: u.email,
            firstname: u.firstname,
            lastname: u.lastname,
        }),
    )))
}
