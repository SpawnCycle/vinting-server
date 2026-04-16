use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    #[sea_orm(indexed)]
    pub deleted_at: Option<DateTime>,

    #[sea_orm(unique)]
    pub name: String,

    #[sea_orm(has_many)]
    pub product_tags: HasMany<super::product_tag::Entity>,

    #[sea_orm(has_many, via = "product_tag")]
    pub products: HasMany<super::product::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

crate::active_actions!(ActiveModelEx);
