use entity::user;
use serde::{Deserialize, Serialize};

use crate::{email_string::EmailString, limited_string::LimitedString};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPutDto {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<EmailString>,
    pub password: Option<LimitedString<50, 8>>,
}

impl From<UserPutDto> for user::ActiveModelEx {
    fn from(d: UserPutDto) -> Self {
        let mut am = user::ActiveModelEx::new().set_id(d.id);
        if let Some(name) = d.name {
            am = am.set_name(name);
        }
        if let Some(email) = d.email {
            am = am.set_email(email);
        }
        if let Some(password) = d.password {
            am = am.set_password_hash(super::hash_password(&password));
        }
        am
    }
}
