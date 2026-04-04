use entity::user;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPutDto {
    pub id: i32,
    #[serde(flatten)]
    pub data: super::post::UserPostDto,
}

impl From<UserPutDto> for user::ActiveModelEx {
    fn from(d: UserPutDto) -> Self {
        user::ActiveModelEx::from(d.data).set_id(d.id)
    }
}
