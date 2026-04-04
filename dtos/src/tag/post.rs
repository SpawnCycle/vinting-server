use entity::tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagPostDto {
    pub name: String,
}

impl From<TagPostDto> for tag::ActiveModelEx {
    fn from(t: TagPostDto) -> Self {
        tag::ActiveModel::builder().set_name(t.name)
    }
}
