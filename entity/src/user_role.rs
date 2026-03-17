use sea_orm::{ActiveValue::Set, entity::prelude::*, sea_query::prelude::Utc};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_role")]
pub struct Model {
    pub created_at: DateTime,
    pub modified_at: DateTime,
    #[sea_orm(indexed)]
    pub deleted_at: Option<DateTime>,

    // composite key
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: i32,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[sea_orm(belongs_to, from = "role_id", to = "id")]
    pub role: HasOne<super::role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = Utc::now().naive_local();
        Self {
            created_at: Set(now),
            modified_at: Set(now),
            ..ActiveModelTrait::default()
        }
    }
}

crate::active_actions!(ActiveModelEx);
