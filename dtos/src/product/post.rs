use entity::product::{self, ProductCondition, ProductSex};
use serde::Deserialize;

/// DOESN'T SET THE FOREIGN STUFF
#[derive(Debug, Clone, Deserialize)]
pub struct ProductPostDto {
    pub name: String,
    pub description: String,
    pub price: u32,
    pub size: String,
    pub color: String,
    pub brand: Option<String>,

    pub condition: ProductCondition,
    pub sex: ProductSex,

    /// List of category ids
    pub categories: Vec<i32>,
    /// List of image ids
    pub images: Vec<i32>,
    /// List of tag ids
    pub tags: Vec<i32>,
}

impl ProductPostDto {
    #[must_use]
    pub fn into_active_model(self, uid: i32) -> product::ActiveModelEx {
        product::ActiveModelEx::from(self).set_seller_id(uid)
    }
}

impl From<ProductPostDto> for product::ActiveModelEx {
    // Doesn't set the foreign stuff
    fn from(d: ProductPostDto) -> product::ActiveModelEx {
        // user id is set outside of this function, because we get it from auth
        product::ActiveModel::builder()
            .set_name(d.name)
            .set_condition(d.condition)
            .set_price(d.price)
            .set_sex(d.sex)
            .set_size(d.size)
            .set_color(d.color)
            .set_brand(d.brand)
            .set_description(d.description)
            .set_has_stock(true)
    }
}
