use entity::user;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct WhoamiDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub name: String,
    pub email: String,
    pub roles: Vec<String>,
}

impl WhoamiDto {
    #[must_use]
    pub fn new(m: user::ModelEx) -> Option<Self> {
        if !m.roles.is_loaded() {
            return None;
        }

        Some(Self {
            id: m.id,
            created_at: m.created_at,
            modified_at: m.modified_at,
            name: m.name,
            email: m.email,
            roles: m.roles.into_iter().map(|r| r.name).collect(),
        })
    }
}
