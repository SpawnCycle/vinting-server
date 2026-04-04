use argon2::{Argon2, PasswordHasher};
use entity::user;
use serde::{Deserialize, Serialize};

use crate::email_string::EmailString;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPostDto {
    pub name: String,
    pub email: EmailString,
    pub password: String,
}

impl From<UserPostDto> for user::ActiveModelEx {
    fn from(d: UserPostDto) -> Self {
        let argon = Argon2::from(crate::get_argon_params());
        let pwd_hash = argon
            .hash_password(d.password.as_bytes())
            .expect("Hashing should not error if configured properly")
            .to_string();

        user::ActiveModel::builder()
            .set_name(d.name)
            .set_email(d.email)
            .set_password_hash(pwd_hash)
    }
}
