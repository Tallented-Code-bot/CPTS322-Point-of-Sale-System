use mongodb::bson::{doc, oid::ObjectId, Document};
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use upc_a::UpcA;

use crate::auth::AdminUser;
use crate::models::{Product, UPC};
use crate::repository::MongoRepo;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminProductRow {
    id: String,
    upc: String,
    name: String,
    price: f32,
    quantity: u32,
}

fn to_row(p: Product) -> Result<AdminProductRow, Status> {
    let id = p.id.ok_or(Status::InternalServerError)?.to_hex();
    let upc = p
        .upc
        .map(|u| u.0.to_string())
        .ok_or(Status::InternalServerError)?;
    let name = p.name.unwrap_or_else(|| "Unnamed product".to_string());
    let price = p.price.ok_or(Status::InternalServerError)?;
    let quantity = p.quantity.unwrap_or(0);
    Ok(AdminProductRow {
        id,
        upc,
        name,
        price,
        quantity,
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductRequest {
    upc: String,
    name: String,
    price: f32,
    quantity: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchProductRequest {
    upc: Option<String>,
    name: Option<String>,
    price: Option<f32>,
    quantity: Option<u32>,
}

#[get("/admin/products?<query>")]
pub async fn admin_get_products(
    db: &State<MongoRepo>,
    _admin: AdminUser,
    query: Option<String>,
) -> Result<Json<Vec<AdminProductRow>>, Status> {
    let mut products = db
        .get_all_products()
        .await
        .map_err(|_| Status::InternalServerError)?;

    if let Some(q) = query {
        let q = q.trim().to_lowercase();
        if !q.is_empty() {
            products = products
                .into_iter()
                .filter(|p| {
                    let upc = p.upc.as_ref().map(|u| u.0.to_string()).unwrap_or_default();
                    let name = p.name.clone().unwrap_or_default();
                    let price = p.price.unwrap_or(0.0).to_string();
                    let qty = p.quantity.unwrap_or(0).to_string();
                    upc.to_lowercase().contains(&q)
                        || name.to_lowercase().contains(&q)
                        || price.contains(&q)
                        || qty.contains(&q)
                })
                .collect();
        }
    }

    let rows = products
        .into_iter()
        .map(to_row)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Json(rows))
}

#[post("/admin/products", data = "<payload>")]
pub async fn admin_create_product(
    db: &State<MongoRepo>,
    _admin: AdminUser,
    payload: Json<CreateProductRequest>,
) -> Result<Json<AdminProductRow>, (Status, Json<ApiError>)> {
    let data = payload.into_inner();

    let upc_raw = data.upc.trim();
    let parsed_upc = UpcA::from_str(upc_raw).map_err(|_| {
        (
            Status::BadRequest,
            Json(ApiError {
                message: "Invalid UPC-A (must be 12 digits with a valid check digit).".to_string(),
            }),
        )
    })?;
    if data.price < 0.0 {
        return Err((
            Status::BadRequest,
            Json(ApiError {
                message: "Price must be >= 0.".to_string(),
            }),
        ));
    }
    if data.name.trim().is_empty() {
        return Err((
            Status::BadRequest,
            Json(ApiError {
                message: "Name is required.".to_string(),
            }),
        ));
    }

    let product = Product {
        id: None,
        name: Some(data.name.trim().to_string()),
        price: Some(data.price),
        upc: Some(UPC(parsed_upc)),
        quantity: Some(data.quantity),
    };

    let _id = db.create_product(product).await.map_err(|_| {
        (
            Status::InternalServerError,
            Json(ApiError {
                message: "Failed to create product.".to_string(),
            }),
        )
    })?;

    // Return the created product by listing and finding it (small dataset assumption).
    let created = db
        .get_all_products()
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiError {
                    message: "Failed to fetch products.".to_string(),
                }),
            )
        })?
        .into_iter()
        .find(|p| p.id.as_ref().is_some_and(|oid| oid == &_id))
        .ok_or((
            Status::InternalServerError,
            Json(ApiError {
                message: "Created product could not be loaded.".to_string(),
            }),
        ))?;

    let row = to_row(created).map_err(|_| {
        (
            Status::InternalServerError,
            Json(ApiError {
                message: "Failed to serialize product.".to_string(),
            }),
        )
    })?;
    Ok(Json(row))
}

#[patch("/admin/products/<id>", data = "<payload>")]
pub async fn admin_patch_product(
    db: &State<MongoRepo>,
    _admin: AdminUser,
    id: String,
    payload: Json<PatchProductRequest>,
) -> Result<Status, (Status, Json<ApiError>)> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| {
        (
            Status::BadRequest,
            Json(ApiError {
                message: "Invalid product id.".to_string(),
            }),
        )
    })?;
    let data = payload.into_inner();
    if data.upc.is_none() && data.name.is_none() && data.price.is_none() && data.quantity.is_none()
    {
        return Err((
            Status::BadRequest,
            Json(ApiError {
                message: "No fields to update.".to_string(),
            }),
        ));
    }

    let mut set_doc = Document::new();

    if let Some(upc) = data.upc {
        let parsed = UpcA::from_str(upc.trim()).map_err(|_| {
            (
                Status::BadRequest,
                Json(ApiError {
                    message: "Invalid UPC-A (must be 12 digits with a valid check digit)."
                        .to_string(),
                }),
            )
        })?;
        set_doc.insert("upc", parsed.to_string());
    }
    if let Some(name) = data.name {
        if name.trim().is_empty() {
            return Err((
                Status::BadRequest,
                Json(ApiError {
                    message: "Name cannot be empty.".to_string(),
                }),
            ));
        }
        set_doc.insert("name", name.trim().to_string());
    }
    if let Some(price) = data.price {
        if price < 0.0 {
            return Err((
                Status::BadRequest,
                Json(ApiError {
                    message: "Price must be >= 0.".to_string(),
                }),
            ));
        }
        set_doc.insert("price", price);
    }
    if let Some(qty) = data.quantity {
        set_doc.insert("quantity", qty);
    }

    let updated = db.update_product(object_id, set_doc).await.map_err(|_| {
        (
            Status::InternalServerError,
            Json(ApiError {
                message: "Failed to update product.".to_string(),
            }),
        )
    })?;
    if !updated {
        return Err((
            Status::NotFound,
            Json(ApiError {
                message: "Product not found.".to_string(),
            }),
        ));
    }
    Ok(Status::NoContent)
}

#[delete("/admin/products/<id>")]
pub async fn admin_delete_product(
    db: &State<MongoRepo>,
    _admin: AdminUser,
    id: String,
) -> Result<Status, (Status, Json<ApiError>)> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| {
        (
            Status::BadRequest,
            Json(ApiError {
                message: "Invalid product id.".to_string(),
            }),
        )
    })?;
    let deleted = db.delete_product(object_id).await.map_err(|_| {
        (
            Status::InternalServerError,
            Json(ApiError {
                message: "Failed to delete product.".to_string(),
            }),
        )
    })?;
    if !deleted {
        return Err((
            Status::NotFound,
            Json(ApiError {
                message: "Product not found.".to_string(),
            }),
        ));
    }
    Ok(Status::NoContent)
}
