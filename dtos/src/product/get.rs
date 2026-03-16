use entity::product::{self, ProductCondition, ProductSex};
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

    pub price: u64,
    pub size: String,
    pub brand: Option<String>,

    pub condition: ProductCondition,
    pub sex: ProductSex,

    pub user: UserGetDto,
    pub categories: Vec<CategoryGetDto>,
    pub tags: Vec<TagGetDto>,
    pub images: Vec<ImageGetDto>,
}

impl ProductGetDto {
    pub fn from_model_with_host(m: product::ModelEx, host: &str) -> Option<Self> {
        if !m.categories.is_loaded()
            || !m.tags.is_loaded()
            || !m.images.is_loaded()
            || !m.user.is_loaded()
        {
            return None;
        }

        let user = m.user.unwrap();

        Some(Self {
            id: m.id,
            created_at: m.created_at,
            modified_at: m.modified_at,

            name: m.name,
            description: m.description,

            price: m.price,
            size: m.size,
            brand: m.brand,
            condition: m.condition,
            sex: m.sex,

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
        })
    }
}
