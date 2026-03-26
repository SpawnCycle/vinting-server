use sea_orm::FromQueryResult;

#[derive(Debug, Clone, FromQueryResult)]
pub struct ProductId {
    pub id: i32,
}
