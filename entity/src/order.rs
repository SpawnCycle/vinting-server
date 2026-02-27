use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    #[sea_orm(indexed)]
    pub deleted_at: Option<DateTime>,

    #[sea_orm(indexed)]
    pub user_id: i32,
    #[sea_orm(indexed)]
    pub product_id: i32,

    pub arrived_at: Option<DateTime>,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,
    #[sea_orm(belongs_to, from = "product_id", to = "id")]
    pub product: HasOne<super::product::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

crate::active_actions!(ActiveModelEx);
