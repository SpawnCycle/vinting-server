use argon2::{Argon2, PasswordVerifier};
use dtos::user::{get::UserGetDto, post::UserPostDto};
use entity::{active_action::ActiveAction, prelude::*, role, user};
use migrations::constants::{ADMIN_ROLE, USER_ROLE};
use rocket::{
    FromForm, State,
    form::Form,
    http::{Cookie, CookieJar, uri::Host},
    post,
    response::status::{Created, NoContent},
    serde::{Deserialize, json::Json},
};
use sea_orm::{DbConn, IntoActiveModel};
use services::{service_trait::ServiceTrait, user_service::UserService};

use crate::{
    constants::{ADMIN_EMAIL, JWT_KEY, construct_host},
    jwt::JwtClaims,
    responder::Responder,
};

#[derive(Debug, Clone, FromForm, Deserialize)]
pub struct LoginDetails<'a> {
    email: &'a str,
    password: &'a str,
}

#[post("/signup", format = "application/json", data = "<data>")]
pub async fn signup(
    host: &Host<'_>,
    jar: &CookieJar<'_>,
    db: &State<DbConn>,
    data: Json<UserPostDto>,
) -> Result<Created<Json<UserGetDto>>, Responder> {
    let db = db.inner();
    let user = data.into_inner();
    let host = construct_host(host);
    let email = user.email.to_owned();
    let service = UserService(db);

    if service.exists_by_email_all(user.email.to_owned()).await? {
        return Err(Responder::conflict("A user with that email already exists"));
    }

    let default_role = Role::find_by_name(USER_ROLE).one(db).await?.map_or(
        role::ActiveModelEx::new().creating().set_name(USER_ROLE),
        |m| m.into_active_model().into(),
    );

    let user = user::ActiveModelEx::from(user).add_role(default_role);
    let user = if let Some(admin_email) = *ADMIN_EMAIL
        && *email == admin_email
    {
        let admin_role = Role::find_by_name(ADMIN_ROLE).one(db).await?.map_or(
            role::ActiveModelEx::new().creating().set_name(ADMIN_ROLE),
            |m| m.into_active_model().into(),
        );
        user.add_role(admin_role)
    } else {
        user
    };

    let user = service.insert(user).await?;

    add_jwt_to_jar(user.id, jar)?;

    Ok(Created::new(format!("{host}/api/users/{}", user.id)).body(Json(user.into())))
}

#[post("/login", format = "application/json", data = "<data>")]
pub async fn login_json(
    db: &State<DbConn>,
    data: Json<LoginDetails<'_>>,
    jar: &CookieJar<'_>,
) -> Result<NoContent, Responder> {
    let db = db.inner();

    let user = verify_user(db, data.into_inner()).await?;

    add_jwt_to_jar(user.id, jar)?;

    Ok(NoContent)
}

#[post("/login", data = "<data>", rank = 2)]
pub async fn login_form(
    db: &State<DbConn>,
    data: Form<LoginDetails<'_>>,
    jar: &CookieJar<'_>,
) -> Result<NoContent, Responder> {
    let db = db.inner();

    let user = verify_user(db, data.into_inner()).await?;

    add_jwt_to_jar(user.id, jar)?;

    Ok(NoContent)
}

pub async fn verify_user(db: &DbConn, data: LoginDetails<'_>) -> Result<user::Model, Responder> {
    let service = UserService(db);

    let user = service
        .get_by_email(data.email)
        .await?
        .ok_or(Responder::not_found(
            "There is no user with the given email",
        ))?;

    let argon2 = Argon2::from(dtos::get_argon_params());

    if argon2
        .verify_password(data.password.as_bytes(), user.password_hash.as_str())
        .is_err()
    {
        return Err(Responder::unauhorized("Wrong password"));
    }

    Ok(user)
}

#[post("/logout")]
pub fn logout(claims: JwtClaims, jar: &CookieJar<'_>) -> NoContent {
    claims.remove_from(jar);

    NoContent
}

fn add_jwt_to_jar(uid: i32, jar: &CookieJar<'_>) -> Result<(), jsonwebtoken::errors::Error> {
    let jwt = JwtClaims::new(uid);

    let cookie = Cookie::build((JWT_KEY, jwt.encode()?)).http_only(true);

    jar.add(cookie);

    Ok(())
}
