use entity::product;
use sea_orm::prelude::DateTime;
use serde::Serialize;
use services::{
    category_service::CategoryService, image_service::ImageService, service_trait::ServiceTrait,
    tag_service::TagService,
};

use crate::{
    category::get::CategoryGetDto, image::get::ImageGetDto, tag::get::TagGetDto,
    user::get::UserGetDto,
};

/// Can only convert from `ModelEx` with `user`, `categories`, `tags`, and `images` loaded
#[derive(Debug, Clone, Serialize)]
pub struct ProductGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub name: String,
    pub description: String,

    pub user: UserGetDto,
    pub categories: Vec<CategoryGetDto>,
    pub tags: Vec<TagGetDto>,
    pub images: Vec<ImageGetDto>,
}

impl ProductGetDto {
    // TODO: remove the allow(unused)
    #[allow(unused)]
    fn from_model_with_host(m: product::ModelEx, host: &str) -> Self {
        // TODO: Write tests for endpoints so we don't find out in prod that these are not set
        assert!(m.categories.is_loaded());
        assert!(m.tags.is_loaded());
        assert!(m.images.is_loaded());
        assert!(m.user.is_loaded());

        let user = m.user.unwrap();

        Self {
            id: m.id,
            created_at: m.created_at,
            modified_at: m.modified_at,

            name: m.name,
            description: m.description,

            user: UserGetDto::from(user),

            categories: m
                .categories
                .into_iter()
                .filter(|m| CategoryService::iter_filter(m.clone()))
                .map(CategoryGetDto::from)
                .collect::<Vec<_>>(),
            images: m
                .images
                .into_iter()
                .filter(|m| ImageService::iter_filter(m.clone()))
                .map(|m| ImageGetDto::from_model_with_host(m, host))
                .collect::<Vec<_>>(),
            tags: m
                .tags
                .into_iter()
                .filter(|m| TagService::iter_filter(m.clone()))
                .map(TagGetDto::from)
                .collect::<Vec<_>>(),
        }
    }
}
