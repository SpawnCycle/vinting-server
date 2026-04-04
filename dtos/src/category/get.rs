use entity::category;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::from_models;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub name: String,
}

from_models!(category, CategoryGetDto, m, {
    Self {
        id: m.id,
        created_at: m.created_at,
        modified_at: m.modified_at,
        name: m.name,
    }
});
