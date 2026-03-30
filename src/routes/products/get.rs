use dtos::{order::get::OrderGetDto, product::get::ProductGetDto};
use entity::{category, prelude::*, product};
use rocket::{
    FromForm, State, get,
    http::{CookieJar, uri::Host},
    serde::json::Json,
};
use sea_orm::{
    ColumnTrait, Condition, DbConn, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use serde::Serialize;
use services::{
    category_service::CategoryService, order_service::OrderService,
    product_service::ProductService, service_trait::ServiceFilter,
};

use crate::{
    constants::construct_host, jwt::JwtClaims, responder::Responder, routes::id_model::IdModel,
};

#[derive(Debug, Clone, FromForm)]
pub struct ProductFilter {
    query: Option<String>,
    gender: Option<String>,
    size: Option<String>,
    color: Option<String>,
    condition: Option<String>,
    categories: Option<Vec<String>>,
    #[field(default = 0)]
    page: u64,
    #[field(default = 10)]
    page_size: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProductPagination {
    data: Vec<ProductGetDto>,
    items: u64,
    pages: u64,
}

#[derive(Debug, Clone)]
pub struct IdPagination {
    data: Vec<i32>,
    items: u64,
    pages: u64,
}

#[get("/<id>")]
pub async fn one(
    id: i32,
    host: &Host<'_>,
    db: &State<DbConn>,
) -> Result<Json<ProductGetDto>, Responder> {
    let db = db.inner();
    let service = ProductService(db);
    let host = construct_host(host);

    let product = service
        .load_by_id(id)
        .await?
        .map(|p| {
            ProductGetDto::from_model_with_host(p, &host)
                .ok_or(Responder::server_error("Could not properly load the model"))
        })
        .ok_or(Responder::not_found(
            "Couldn't find a product with the given id",
        ))??;

    Ok(Json(product))
}

#[get("/?<filters..>")]
pub async fn all(
    host: &Host<'_>,
    db: &State<DbConn>,
    filters: ProductFilter,
) -> Result<Json<ProductPagination>, Responder> {
    let db = db.inner();
    let host = construct_host(host);

    let ids = get_matching_ids(filters, db).await?;

    let products = Product::load()
        .with(User)
        .with(Category)
        .with(Tag)
        .with(Image)
        .filter(product::Column::Id.is_in(ids.data))
        .all(db)
        .await?
        .into_iter()
        .map(|p| {
            ProductGetDto::from_model_with_host(p, &host)
                .expect("The model should be properly loaded")
        })
        .collect::<Vec<_>>();

    Ok(Json(ProductPagination {
        data: products,
        items: ids.items,
        pages: ids.pages,
    }))
}

async fn get_matching_ids(filters: ProductFilter, db: &DbConn) -> Result<IdPagination, DbErr> {
    let mut q = Product::find().service_filter::<ProductService<DbConn>>();

    if let Some(s) = filters.query {
        q = q.filter(
            Condition::any()
                .add(product::Column::Name.like(s.to_owned()))
                .add(product::Column::Description.like(s.to_owned()))
                .add(product::Column::Brand.like(s.to_owned())),
        );
    }

    if let Some(g) = filters.gender {
        q = q.filter(product::Column::Sex.eq(g));
    }

    if let Some(s) = filters.size {
        q = q.filter(product::Column::Size.eq(s));
    }

    if let Some(c) = filters.color {
        q = q.filter(product::Column::Color.eq(c));
    }

    if let Some(c) = filters.condition {
        q = q.filter(product::Column::Condition.eq(c));
    }

    if let Some(c) = filters.categories {
        q = q
            .left_join(Category)
            .group_by(product::Column::Id)
            .service_filter::<CategoryService>()
            .filter(category::Column::Name.is_in(c));
    }

    // wish I could filter with a join inside of a loader, but alas it is not a thing,
    // so this is probably the best way
    let pagination = q
        .select_only()
        .column(product::Column::Id)
        .into_model::<IdModel>()
        .paginate(db, filters.page_size);
    let page = pagination.num_items_and_pages().await?;

    let products = pagination
        // -1 because it returns the first page on index 0
        .fetch_page(filters.page.saturating_sub(1))
        .await?;

    let items = products.into_iter().map(|p| p.id).collect::<Vec<_>>();

    Ok(IdPagination {
        data: items,
        items: page.number_of_items,
        pages: page.number_of_pages,
    })
}

#[get("/<id>/order")]
pub async fn product_order(
    id: i32,
    claims: JwtClaims,
    jar: &CookieJar<'_>,
    db: &State<DbConn>,
) -> Result<Json<OrderGetDto>, Responder> {
    let db = db.inner();
    let service = OrderService(db);
    if !claims.exists_or_remove(db, jar).await? {
        return Err(Responder::unauhorized(
            "You must be logged in to have orders",
        ));
    }

    let order = service
        .get_by_user(claims.uid, id)
        .await?
        .ok_or(Responder::not_found("You did not order this product"))?;

    Ok(Json(order.into()))
}
