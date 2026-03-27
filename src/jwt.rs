use chrono::{Duration, Local};
use entity::{prelude::*, user};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use rocket::{
    Request, async_trait,
    http::{CookieJar, Status},
    request::{FromRequest, Outcome},
};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbConn, DbErr, EntityLoaderTrait, EntityTrait, QueryFilter,
    SelectExt,
};
use serde::{Deserialize, Serialize};
use services::{
    service_trait::{ServiceFilter, ServiceTrait},
    user_service::UserService,
};
use thiserror::Error;

use crate::constants::{JWT_KEY, get_jwt_key};

/// This is the struct used inside the JWT
/// It implements FromRequest, so you can check if a user is signed in with the following:
/// ```no_run
/// use rocket::get;
/// use vinting_server::jwt::JwtClaims;
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

impl JwtClaims {
    pub fn new(uid: i32) -> Self {
        let now = Local::now();
        let iat = now.timestamp();
        let exp = (now + Duration::days(30)).timestamp();

        JwtClaims { exp, iat, uid }
    }

    pub async fn has_role<C>(&self, db: &C, role: &str) -> Result<bool, DbErr>
    where
        C: ConnectionTrait + Send,
    {
        Role::find_by_name(role)
            .inner_join(User)
            .filter(user::Column::Id.eq(self.uid))
            .exists(db)
            .await
    }

    pub fn remove_from(&self, jar: &CookieJar<'_>) {
        jar.remove(JWT_KEY);
    }

    pub fn encode(self) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(get_jwt_key().as_ref()),
        )
    }

    pub async fn exists_or_remove<C>(&self, db: &C, jar: &CookieJar<'_>) -> Result<bool, DbErr>
    where
        C: ConnectionTrait + Send,
    {
        let exists = self.exists(db).await?;
        if !exists {
            self.remove_from(jar);
        }
        Ok(exists)
    }

    pub async fn exists<C>(&self, db: &C) -> Result<bool, DbErr>
    where
        C: ConnectionTrait + Send,
    {
        User::find_by_id(self.uid)
            .service_filter::<UserService<DbConn>>()
            .exists(db)
            .await
    }

    /// fetches the user from the db using the filters defined in `UserService`
    pub async fn fetch<C>(&self, db: &C) -> Result<Option<user::Model>, DbErr>
    where
        C: ConnectionTrait + Send,
    {
        User::find_by_id(self.uid)
            .service_filter::<UserService<DbConn>>()
            .one(db)
            .await
    }

    /// fetches the user from the db without any filters
    pub async fn fetch_always(&self, db: &DbConn) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_id(self.uid).one(db).await
    }

    /// fetches the user from the db using the filters defined in `UserService`
    pub async fn load(
        &self,
        db: &DbConn,
        mut with: impl FnMut(user::EntityLoader) -> user::EntityLoader,
    ) -> Result<Option<user::ModelEx>, DbErr> {
        with(user::Entity::load().filter_by_id(self.uid))
            .filter(UserService::<DbConn>::default_filters())
            .one(db)
            .await
    }

    /// fetches the user from the db without any filters
    pub async fn load_always(
        &self,
        db: &DbConn,
        mut with: impl FnMut(user::EntityLoader) -> user::EntityLoader,
    ) -> Result<Option<user::ModelEx>, DbErr> {
        with(user::Entity::load().filter_by_id(self.uid))
            .one(db)
            .await
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for JwtClaims {
    type Error = JwtError;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let jar = request.cookies();
        let Some(jwt) = jar.get(JWT_KEY) else {
            return Outcome::Error((Status::Unauthorized, JwtError::Missing));
        };
        let jwt = jwt.to_string();

        let Some((_key, jwt)) = jwt.split_once("=") else {
            jar.remove(JWT_KEY);
            return Outcome::Error((
                Status::Unauthorized,
                JwtError::Malformed("The JWT cookie has no value".to_string()),
            ));
        };

        let res = decode::<JwtClaims>(
            jwt,
            &DecodingKey::from_secret(get_jwt_key().as_ref()),
            &Validation::default(),
        )
        .map_err(|err| match err.clone().into_kind() {
            ErrorKind::ExpiredSignature => (Status::Unauthorized, JwtError::Expired),
            _ => (Status::Unauthorized, JwtError::Malformed(err.to_string())),
        });

        let jwt = match res {
            Ok(val) => val,
            Err(err) => {
                jar.remove(JWT_KEY);
                return Outcome::Error(err);
            }
        };

        Outcome::Success(jwt.claims)
    }
}
