use std::path::PathBuf;

use entity::image;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ImageGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub url: String,
    pub user_id: i32,
}

impl ImageGetDto {
    /// # Panics
    /// Panics if the model has a path that doesn't refer to a file
    pub fn from_model_with_host<M>(m: M, host: &str) -> Self
    where
        M: Into<image::Model>,
    {
        let m = m.into() as image::Model;
        let path = PathBuf::from(m.path);
        let file_name = path.file_name().expect("Malformed image path");

        ImageGetDto {
            id: m.id,
            created_at: m.created_at,
            modified_at: m.modified_at,
            user_id: m.user_id,
            url: format!("{host}/img/{}", file_name.display()),
        }
    }
}
