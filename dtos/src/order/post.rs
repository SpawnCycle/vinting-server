use entity::order;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OrderPostDto {
    pub ammount: u32,
}

impl From<OrderPostDto> for order::ActiveModelEx {
    fn from(c: OrderPostDto) -> order::ActiveModelEx {
        order::ActiveModel::builder().set_ammount(c.ammount)
    }
}
