use entity::user;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

use crate::from_models;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub name: String,
    pub email: String,
}

from_models!(user, UserGetDto, m, {
    Self {
        id: m.id,
        created_at: m.created_at,
        modified_at: m.modified_at,
        name: m.name,
        email: m.email,
    }
});
