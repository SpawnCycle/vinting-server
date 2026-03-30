use sea_orm::prelude::*;

use entity::order;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OrderPutDto {
    pub id: i32,
    pub arrived_at: Option<DateTime>,
}

impl From<OrderPutDto> for order::ActiveModelEx {
    fn from(d: OrderPutDto) -> Self {
        order::ActiveModelEx::new()
            .set_id(d.id)
            .set_arrived_at(d.arrived_at)
    }
}
