use chrono::{Duration, Local};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use rocket::{
    Request, async_trait,
    http::Status,
    request::{FromRequest, Outcome},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::constants::{JWT_KEY, JWT_STR};

/// This is the struct used inside the JWT
/// It implements FromRequest, so you can check if a user is signed in with the following:
/// ```no_run
/// use rocket::get;
/// use vinting_server::routes::users::jwt::JwtClaims;
///
///
/// #[get("/")]
/// fn a_route(
///     user_claims: JwtClaims
/// ) -> &'static str {
///     "You are logged in"
/// }
/// ```
/// But since the db is not accessable from within the request,
/// you still need to verify if the user exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub exp: i64,
    pub iat: i64,
    pub uid: i32,
}

#[derive(Debug, Clone, Error)]
pub enum JwtError {
    #[error("Could not find a jwt")]
    Missing,
    #[error("JWT cookie could not be parsed: {0}")]
    Malformed(String),
    #[error("The jwt has expired")]
    Expired,
}

#[async_trait]
impl<'a> FromRequest<'a> for JwtClaims {
    type Error = JwtError;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let jar = request.cookies();
        let Some(jwt) = jar.get(JWT_STR) else {
            return Outcome::Error((Status::Unauthorized, JwtError::Missing));
        };
        let jwt = jwt.to_string();

        let Some((_key, jwt)) = jwt.split_once("=") else {
            return Outcome::Error((
                Status::Unauthorized,
                JwtError::Malformed("The JWT cookie has no value".to_string()),
            ));
        };

        let res = decode::<JwtClaims>(
            jwt,
            &DecodingKey::from_secret(JWT_KEY.as_ref()),
            &Validation::default(),
        )
        .map_err(|err| match err.clone().into_kind() {
            ErrorKind::ExpiredSignature => (Status::Unauthorized, JwtError::Expired),
            _ => (Status::Unauthorized, JwtError::Malformed(err.to_string())),
        });

        let jwt = match res {
            Ok(val) => val,
            Err(err) => return Outcome::Error(err),
        };

        Outcome::Success(jwt.claims)
    }
}

pub fn make_jwt(uid: i32, _secret: String) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Local::now();
    let iat = now.timestamp();
    let exp = (now + Duration::days(30)).timestamp();

    let claims = JwtClaims { exp, iat, uid };

    // TODO: key
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_KEY.as_ref()),
    )
}
