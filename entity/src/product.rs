use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    #[sea_orm(indexed)]
    pub deleted_at: Option<DateTime>,

    pub name: String,
    pub description: String,

    pub price: u32,

    pub has_stock: bool,
    pub size: String,
    pub brand: Option<String>,

    pub condition: ProductCondition,
    pub sex: ProductSex,

    #[sea_orm(indexed)]
    pub seller_id: i32,

    #[sea_orm(belongs_to, from = "seller_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[sea_orm(has_many)]
    pub orders: HasMany<super::order::Entity>,

    #[sea_orm(has_many)]
    pub product_tags: HasMany<super::product_tag::Entity>,
    #[sea_orm(has_many, via = "product_tag")]
    pub tags: HasMany<super::tag::Entity>,

    #[sea_orm(has_many)]
    pub product_categories: HasMany<super::product_category::Entity>,
    #[sea_orm(has_many, via = "product_category")]
    pub categories: HasMany<super::category::Entity>,

    #[sea_orm(has_many)]
    pub product_images: HasMany<super::product_image::Entity>,
    #[sea_orm(has_many, via = "product_image")]
    pub images: HasMany<super::image::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

crate::active_actions!(ActiveModelEx);

// Unholy stuff below

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Display,
    EnumString,
    Serialize,
    Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(try_from = "&str", into = "String")]
pub enum ProductCondition {
    #[sea_orm(string_value = "New")]
    #[strum(to_string = "New")]
    New,
    #[sea_orm(string_value = "Like new")]
    #[strum(to_string = "Like new")]
    LikeNew,
    #[sea_orm(string_value = "Used")]
    #[strum(to_string = "Used")]
    Used,
    #[sea_orm(string_value = "Heavily used")]
    #[strum(to_string = "Heavily used")]
    HeavilyUsed,
}

impl From<ProductCondition> for String {
    fn from(v: ProductCondition) -> Self {
        v.to_string()
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    DeriveDisplay,
    EnumString,
    Serialize,
    Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(try_from = "&str", into = "String")]
pub enum ProductSex {
    #[sea_orm(string_value = "Male")]
    #[strum(to_string = "Male")]
    Male,
    #[sea_orm(string_value = "Female")]
    #[strum(to_string = "Female")]
    Female,
    #[sea_orm(string_value = "Unisex")]
    #[strum(to_string = "Unisex")]
    Unisex,
}

impl From<ProductSex> for String {
    fn from(v: ProductSex) -> Self {
        v.to_string()
    }
}
