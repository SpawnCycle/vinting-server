use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product_category")]
pub struct Model {
    // composite key
    #[sea_orm(primary_key, auto_increment = false)]
    pub product_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: i32,

    #[sea_orm(belongs_to, from = "product_id", to = "id")]
    pub product: HasOne<super::product::Entity>,

    #[sea_orm(belongs_to, from = "category_id", to = "id")]
    pub tag: HasOne<super::category::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
