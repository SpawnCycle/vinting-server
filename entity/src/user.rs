use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    #[sea_orm(indexed)]
    pub deleted_at: Option<DateTime>,

    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,

    #[sea_orm(has_many)]
    pub images: HasMany<super::image::Entity>,
    #[sea_orm(has_many)]
    pub products: HasMany<super::product::Entity>,
    #[sea_orm(has_many)]
    pub orders: HasMany<super::order::Entity>,

    #[sea_orm(has_many)]
    pub user_roles: HasMany<super::user_role::Entity>,
    #[sea_orm(has_many, via = "user_role")]
    pub roles: HasMany<super::role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

crate::active_actions!(ActiveModelEx);
