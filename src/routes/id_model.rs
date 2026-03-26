use sea_orm::FromQueryResult;

#[derive(Debug, Clone, FromQueryResult)]
pub struct IdModel {
    pub id: i32,
}
