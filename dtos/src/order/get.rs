use entity::order;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

use crate::product::get::ProductGetDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderGetDto {
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,

    pub user_id: i32,
    pub amount: u32,
    pub arrived_at: Option<DateTime>,

    pub product: ProductGetDto,
}

impl OrderGetDto {
    #[must_use]
    pub fn with_product(m: order::ModelEx, host: &str) -> Option<Self> {
        let product = m.product.into_option()?;

        Some(Self {
            id: m.id,
            created_at: m.created_at,
            modified_at: m.modified_at,

            user_id: m.user_id,
            amount: m.amount,
            arrived_at: m.arrived_at,

            product: ProductGetDto::from_model_with_host(product, host)?,
        })
    }
}
