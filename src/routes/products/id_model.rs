use sea_orm::FromQueryResult;

#[derive(Debug, Clone, FromQueryResult)]
pub struct ProductIds {
    pub id: i32,
}
