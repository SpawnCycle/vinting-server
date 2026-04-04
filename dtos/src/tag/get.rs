use entity::tag;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

use crate::from_models;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub name: String,
}

from_models!(tag, TagGetDto, d, {
    Self {
        id: d.id,
        created_at: d.created_at,
        modified_at: d.modified_at,
        name: d.name,
    }
});
