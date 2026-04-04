use entity::order;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPostDto {
    pub ammount: u32,
}

impl From<OrderPostDto> for order::ActiveModelEx {
    fn from(c: OrderPostDto) -> order::ActiveModelEx {
        order::ActiveModel::builder().set_ammount(c.ammount)
    }
}
