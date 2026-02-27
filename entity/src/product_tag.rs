use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product_tag")]
pub struct Model {
    pub created_at: DateTime,
    pub modified_at: DateTime,
    #[sea_orm(indexed)]
    pub deleted_at: Option<DateTime>,

    // composite key
    #[sea_orm(primary_key, auto_increment = false)]
    pub product_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: i32,

    #[sea_orm(belongs_to, from = "product_id", to = "id")]
    pub product: HasOne<super::product::Entity>,

    #[sea_orm(belongs_to, from = "tag_id", to = "id")]
    pub tag: HasOne<super::tag::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

crate::active_actions!(ActiveModelEx);
