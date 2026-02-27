use entity::tag;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TagPostDto {
    pub name: String,
}

impl From<TagPostDto> for tag::ActiveModelEx {
    fn from(t: TagPostDto) -> Self {
        tag::ActiveModel::builder().set_name(t.name)
    }
}
