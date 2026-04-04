mod delete;
mod get;
mod post;
mod put;

use dtos::product::{post::ProductPostDto, put::ProductPutDto};
use entity::{image, prelude::*, product};
use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    routes,
};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseTransaction, DbConn, EntityTrait, IntoActiveModel,
    QueryFilter, TransactionTrait,
};
use services::{
    category_service::CategoryService,
    image_service::ImageService,
    service_trait::{ServiceFilter, ServiceTrait},
    tag_service::TagService,
};

use crate::responder::Responder;

pub struct ProductFairing;

#[async_trait]
impl Fairing for ProductFairing {
    fn info(&self) -> Info {
        Info {
            name: "Product Fairing",
            kind: Kind::Ignite | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r.mount(
            "/api/products",
            routes![
                get::all,
                get::one,
                post::one,
                post::form,
                post::order_product,
                put::one,
                delete::one,
            ],
        );

        Ok(r)
    }
}

// so this is the way to go, very sad
pub async fn product_post_dto_to_am_with_associations<C>(
    dto: &ProductPostDto,
    uid: i32,
    db: &C,
) -> Result<product::ActiveModelEx, Responder>
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>,
{
    let mut am = dto.clone().into_active_model(uid);
    am.categories.replace_all([]);
    am.tags.replace_all([]);
    am.images.replace_all([]);
    let c_service = CategoryService(db);
    let t_service = TagService(db);

    for c_id in dto.categories.iter() {
        let c = c_service
            .get_by_id(*c_id)
            .await?
            .ok_or(Responder::not_found(format!(
                "There is no category with the id of {c_id}"
            )))?;
        am = am.add_category(c.into_active_model());
    }

    for t_id in dto.tags.iter() {
        let t = t_service
            .get_by_id(*t_id)
            .await?
            .ok_or(Responder::not_found(format!(
                "There is no tag with the id of {t_id}"
            )))?;
        am = am.add_tag(t.into_active_model());
    }

    for i_id in dto.images.iter() {
        let i = Image::find_by_id(*i_id)
            .filter(image::Column::UserId.eq(uid))
            .service_filter::<ImageService<DbConn>>()
            .one(db)
            .await?
            .ok_or(Responder::not_found(format!(
                "You did not post an image with the id of {i_id}"
            )))?;
        am = am.add_image(i.into_active_model());
    }

    Ok(am)
}

// insane name
pub async fn product_put_dto_to_am_with_associations<C>(
    dto: &ProductPutDto,
    uid: i32,
    db: &C,
) -> Result<product::ActiveModelEx, Responder>
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>,
{
    Ok(product_post_dto_to_am_with_associations(&dto.data, uid, db)
        .await?
        .set_id(dto.id)
        .set_has_stock(dto.has_stock))
}

#[cfg(test)]
mod tests {
    use sea_orm::Database;

    #[tokio::test]
    async fn ignites_successfully() -> anyhow::Result<()> {
        let db = Database::connect("sqlite::memory:").await?;
        let r = rocket::build().manage(db).attach(super::ProductFairing);

        r.ignite().await?;

        Ok(())
    }
}
