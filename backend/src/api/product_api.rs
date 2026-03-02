use rocket::{http::Status, serde::json::Json, State};
use upc_a::UpcA;

use crate::{
    models::{Product, UPC},
    repository::MongoRepo,
};

#[get("/products")]
pub async fn get_all_products(db: &State<MongoRepo>) -> Result<Json<Vec<Product>>, Status> {
    let products = db.get_all_products().await;
    match products {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/products/<upc>")]
pub async fn get_product_by_upc(
    db: &State<MongoRepo>,
    upc: UPC,
) -> Result<Json<Option<Product>>, Status> {
    let product = db.get_product_by_upc(upc).await;
    match product {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/push")]
pub async fn push(db: &State<MongoRepo>) {
    db.insert_test_product().await.unwrap();
}
