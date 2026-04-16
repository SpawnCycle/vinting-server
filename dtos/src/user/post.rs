use entity::user;
use serde::{Deserialize, Serialize};

use crate::{email_string::EmailString, limited_string::LimitedString};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPostDto {
    pub name: String,
    pub email: EmailString,
    pub password: LimitedString<50, 8>,
}

impl From<UserPostDto> for user::ActiveModelEx {
    fn from(d: UserPostDto) -> Self {
        let pwd_hash = super::hash_password(&d.password);

        user::ActiveModel::builder()
            .set_name(d.name)
            .set_email(d.email)
            .set_password_hash(pwd_hash)
    }
}
