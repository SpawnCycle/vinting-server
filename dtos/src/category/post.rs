use entity::category;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryPostDto {
    pub name: String,
}

impl From<CategoryPostDto> for category::ActiveModelEx {
    fn from(c: CategoryPostDto) -> category::ActiveModelEx {
        category::ActiveModel::builder().set_name(c.name)
    }
}
