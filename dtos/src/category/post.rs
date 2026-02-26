use entity::category;
use sea_orm::sea_query::prelude::Utc;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryPostDto {
    pub name: String,
}

impl From<CategoryPostDto> for category::ActiveModelEx {
    fn from(c: CategoryPostDto) -> category::ActiveModelEx {
        category::ActiveModel::builder()
            .set_name(c.name)
            .set_created_at(Utc::now().naive_local())
    }
}

crate::active_actions!(category::ActiveModelEx);
