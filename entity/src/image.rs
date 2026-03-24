use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "image")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    #[sea_orm(indexed)]
    pub deleted_at: Option<DateTime>,

    // WARN: DO NOT ACCEPT THIS FROM THE USER
    pub path: String,

    #[sea_orm(indexed)]
    pub user_id: i32,

    #[sea_orm(has_many)]
    pub product_images: HasMany<super::product_image::Entity>,
    #[sea_orm(has_many, via = "product_image")]
    pub products: HasMany<super::product::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

crate::active_actions!(ActiveModelEx);
