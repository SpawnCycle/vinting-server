use entity::tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagPutDto {
    pub id: i32,
    #[serde(flatten)]
    pub data: super::post::TagPostDto,
}

impl From<TagPutDto> for tag::ActiveModelEx {
    fn from(d: TagPutDto) -> Self {
        tag::ActiveModelEx::from(d.data).set_id(d.id)
    }
}
