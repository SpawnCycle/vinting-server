use entity::order;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OrderPostDto {
    pub user_id: i32,
}

impl From<OrderPostDto> for order::ActiveModelEx {
    fn from(c: OrderPostDto) -> order::ActiveModelEx {
        order::ActiveModel::builder().set_user_id(c.user_id)
    }
}
