use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    #[sea_orm(indexed)]
    pub deleted_at: Option<DateTime>,

    pub name: String,
    pub description: String,

    #[sea_orm(indexed)]
    pub user_id: i32,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[sea_orm(has_many)]
    pub orders: HasMany<super::order::Entity>,

    #[sea_orm(has_many)]
    pub product_tags: HasMany<super::product_tag::Entity>,
    #[sea_orm(has_many, via = "product_tag")]
    pub tags: HasMany<super::tag::Entity>,

    #[sea_orm(has_many)]
    pub product_categories: HasMany<super::product_category::Entity>,
    #[sea_orm(has_many, via = "product_category")]
    pub categories: HasMany<super::category::Entity>,

    #[sea_orm(has_many)]
    pub product_images: HasMany<super::product_image::Entity>,
    #[sea_orm(has_many, via = "product_image")]
    pub images: HasMany<super::image::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

crate::active_actions!(ActiveModelEx);
