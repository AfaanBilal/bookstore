/**
 * 📕 BookStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/bookstore
 */
use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    serde::{Deserialize, Serialize},
};

use crate::AppConfig;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: u64,
}

pub struct AuthenticatedUser {
    pub id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(token) = req.headers().get_one("token") {
            let config = req.rocket().state::<AppConfig>().unwrap();

            let data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            );

            let claims = match data {
                Ok(p) => p.claims,
                Err(_) => {
                    return Outcome::Failure((Status::Unauthorized, "Invalid token".to_string()))
                }
            };

            Outcome::Success(AuthenticatedUser { id: claims.sub })
        } else {
            Outcome::Failure((Status::Unauthorized, "Token absent".to_string()))
        }
    }
}
