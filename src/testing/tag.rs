use entity::{active_action::ActiveAction, tag};
use sea_orm::{ActiveValue::Set, DbConn, DbErr};

pub async fn seed_db(db: &DbConn) -> Result<(), DbErr> {
    tag::ActiveModel {
        name: Set("Tag 1".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .insert(db)
    .await?;

    tag::ActiveModel {
        name: Set("Tag 2".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .insert(db)
    .await?;

    tag::ActiveModel {
        name: Set("Tag 3".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .insert(db)
    .await?;

    tag::ActiveModel {
        name: Set("Tag deleted 1".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .deleting()
    .insert(db)
    .await?;

    tag::ActiveModel {
        name: Set("Tag deleted 2".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .deleting()
    .insert(db)
    .await?;

    tag::ActiveModel {
        name: Set("Tag deleted 3".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .deleting()
    .insert(db)
    .await?;

    Ok(())
}
