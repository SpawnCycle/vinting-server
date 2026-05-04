use dtos::ProductGetDto;
use entity::{category, prelude::*, product};
use rocket::{FromForm, FromFormField, State, get, http::uri::Host, serde::json::Json};
use sea_orm::{
    ColumnTrait, Condition, DbConn, DbErr, EntityTrait, ExprTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde::Serialize;
use services::{
    category_service::CategoryService, product_service::ProductService,
    service_trait::ServiceFilter,
};

use crate::{constants::construct_host, responder::Responder, routes::id_model::IdModel};

#[derive(Debug, Clone, Copy, FromFormField)]
enum ProductSort {
    #[field(value = "date")]
    Date,
    #[field(value = "price")]
    Price,
}

#[derive(Debug, Clone, FromForm)]
pub struct ProductFilter {
    query: Option<String>,
    gender: Option<String>,
    sizes: Option<Vec<String>>,
    colors: Option<Vec<String>>,
    conditions: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    sort_by: Option<ProductSort>,
    #[field(default = false)]
    asc: bool,
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
    let service = ProductService(db);

    let ids = get_matching_ids(filters.clone(), db).await?;

    let mut products = Vec::new();

    // This results in more queries than I'd want, which is very sad,
    // but it's needed to maintain the order of the products
    for id in ids.data {
        products.push(
            service
                .load_by_id_mutating(id, |m| {
                    ProductGetDto::from_model_with_host(m, &host)
                        .expect("The model should be properly loaded")
                })
                .await?
                .expect("This id was just aquired from the db"),
        );
    }

    Ok(Json(ProductPagination {
        data: products,
        items: ids.items,
        pages: ids.pages,
    }))
}

async fn get_matching_ids(filters: ProductFilter, db: &DbConn) -> Result<IdPagination, DbErr> {
    let mut q = Product::find().service_filter::<ProductService>().filter(
        // filter the sold products
        product::Column::OverallStock
            .into_expr()
            .gt(product::Column::SoldStock.into_expr()),
    );

    if let Some(s) = filters.query {
        q = q.filter(
            Condition::any()
                .add(product::Column::Name.like(s.clone()))
                .add(product::Column::Description.like(s.clone()))
                .add(product::Column::Brand.like(s.clone())),
        );
    }

    if let Some(g) = filters.gender {
        q = q.filter(product::Column::Sex.eq(g));
    }

    if let Some(s) = filters.sizes {
        q = q.filter(product::Column::Size.is_in(s));
    }

    if let Some(c) = filters.colors {
        q = q.filter(product::Column::Color.is_in(c));
    }

    if let Some(c) = filters.conditions {
        q = q.filter(product::Column::Condition.is_in(c));
    }

    if let Some(c) = filters.categories {
        q = q
            .left_join(Category)
            .group_by(product::Column::Id)
            .service_filter::<CategoryService>()
            .filter(category::Column::Name.is_in(c));
    }

    let order_col = match filters.sort_by {
        None | Some(ProductSort::Date) => product::Column::CreatedAt,
        Some(ProductSort::Price) => product::Column::Price,
    };

    if filters.asc {
        q = q.order_by_asc(order_col);
    } else {
        q = q.order_by_desc(order_col);
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
