use sea_orm::sea_query::prelude::Utc;

use entity::order;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPutDto {
    pub id: i32,
    pub arrived: bool,
}

impl From<OrderPutDto> for order::ActiveModelEx {
    fn from(d: OrderPutDto) -> Self {
        let now = Utc::now().naive_local();
        let arrived = if d.arrived { Some(now) } else { None };

        order::ActiveModelEx::new()
            .set_id(d.id)
            .set_arrived_at(arrived)
    }
}
