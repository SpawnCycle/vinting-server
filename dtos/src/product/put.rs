use entity::product;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ProductPutDto {
    pub id: i32,
    #[serde(flatten)]
    pub data: super::post::ProductPostDto,
}

impl ProductPutDto {
    #[must_use]
    pub fn into_active_model(self, uid: i32) -> product::ActiveModelEx {
        product::ActiveModelEx::from(self).set_seller_id(uid)
    }
}

impl From<ProductPutDto> for product::ActiveModelEx {
    fn from(d: ProductPutDto) -> Self {
        product::ActiveModelEx::from(d.data).set_id(d.id)
    }
}
