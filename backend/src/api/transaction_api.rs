use crate::{
    models::{Product, Transaction, TransactionItem},
    repository::MongoRepo,
};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use upc_a::UpcA;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckoutRequest {
    items: Vec<CheckoutItemRequest>,
    total_price: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckoutItemRequest {
    upc: String,
    qty: u32,
    unit_price: f32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CheckoutResponse {
    receipt_id: String,
    timestamp: DateTime<Utc>,
    item_count: usize,
    total: f32,
}

#[post("/transactions", data = "<checkout>")]
pub async fn create_transaction(
    db: &State<MongoRepo>,
    checkout: Json<CheckoutRequest>,
) -> Result<Json<CheckoutResponse>, Status> {
    let data = checkout.into_inner();
    if data.items.is_empty() {
        return Err(Status::BadRequest);
    }

    let catalog = db
        .get_all_products()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let product_lookup = build_product_lookup(catalog);

    let mut transaction_items: Vec<TransactionItem> = Vec::with_capacity(data.items.len());
    let mut payload_total: f32 = 0.0;

    for item in data.items {
        if item.qty == 0 {
            return Err(Status::BadRequest);
        }

        if item.unit_price < 0.0 {
            return Err(Status::BadRequest);
        }

        let parsed_upc = UpcA::from_str(item.upc.trim()).map_err(|_| Status::BadRequest)?;
        let canonical_upc = parsed_upc.to_string();
        let product = product_lookup.get(&canonical_upc).ok_or(Status::NotFound)?;

        payload_total += item.unit_price * item.qty as f32;
        transaction_items.push(TransactionItem {
            product_id: product.id.clone(),
            name: product.name.clone(),
            price: item.unit_price,
            quantity: item.qty,
        });
    }

    let payload_total = (payload_total * 100.0).round() / 100.0;
    let provided_total = (data.total_price * 100.0).round() / 100.0;
    if (payload_total - provided_total).abs() > 0.05 {
        return Err(Status::BadRequest);
    }

    let timestamp = Utc::now();
    let item_count = transaction_items.len();
    let transaction = Transaction {
        id: None,
        items: transaction_items,
        total_price: payload_total,
        timestamp,
    };

    let receipt_id = db
        .create_transaction(transaction)
        .await
        .map_err(|_| Status::InternalServerError)?
        .to_hex();

    let response = CheckoutResponse {
        receipt_id,
        timestamp,
        item_count,
        total: payload_total,
    };

    Ok(Json(response))
}

#[get("/transactions")]
pub async fn get_all_transactions(db: &State<MongoRepo>) -> Result<Json<Vec<Transaction>>, Status> {
    let transactions = db.get_all_transactions().await;
    match transactions {
        Ok(transactions) => Ok(Json(transactions)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/transactions/<id>")]
pub async fn get_transaction_by_id(
    db: &State<MongoRepo>,
    id: String,
) -> Result<Json<Transaction>, Status> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| Status::BadRequest)?;
    let transaction = db.get_transaction_by_id(object_id).await;
    match transaction {
        Ok(Some(t)) => Ok(Json(t)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[derive(Clone)]
struct ProductSnapshot {
    id: ObjectId,
    name: String,
}

fn build_product_lookup(products: Vec<Product>) -> HashMap<String, ProductSnapshot> {
    let mut lookup = HashMap::new();
    for product in products {
        let Some(upc) = product.upc else {
            continue;
        };
        let Some(id) = product.id.clone() else {
            continue;
        };
        let name = product
            .name
            .unwrap_or_else(|| "Unnamed product".to_string());

        lookup.insert(upc.0.to_string(), ProductSnapshot { id, name });
    }
    lookup
}
