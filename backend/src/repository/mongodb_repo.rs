use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    Client, Collection,
};
use std::{env, str::FromStr};
use upc_a::UpcA;

use crate::models::{Product, Transaction, UPC};

pub struct MongoRepo {
    col: Collection<Product>,
    transaction_col: Collection<Transaction>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv::dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("POS");
        let col: Collection<Product> = db.collection("products");
        let transaction_col: Collection<Transaction> = db.collection("transactions");
        MongoRepo {
            col,
            transaction_col,
        }
    }

    pub async fn get_product_by_upc(&self, upc: UPC) -> Result<Option<Product>, Error> {
        let mut cursor = self
            .col
            .find(doc! {"upc": upc.0.to_string()})
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;

        if cursor
            .advance()
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?
        {
            match cursor.deserialize_current() {
                Ok(product) => return Ok(Some(product)),
                Err(e) => eprintln!(
                    "Warning: Skipping document due to deserialization error: {}",
                    e
                ),
            }
        }

        Ok(None)
    }

    pub async fn get_all_products(&self) -> Result<Vec<Product>, Error> {
        let mut cursor = self
            .col
            .find(doc! {})
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;

        let mut products: Vec<Product> = Vec::new();
        while cursor
            .advance()
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?
        {
            match cursor.deserialize_current() {
                Ok(product) => products.push(product),
                Err(e) => {
                    eprintln!(
                        "Warning: Skipping document due to deserialization error: {}",
                        e
                    );
                    continue;
                }
            }
        }

        Ok(products)
    }

    pub async fn insert_test_product(&self) -> Result<(), Error> {
        let test_product = Product {
            id: None, // Let MongoDB generate the ObjectId
            name: Some("Test Product".to_string()),
            price: Some(9.99),
            upc: Some(UPC(UpcA::from_str("462479070265").unwrap())), // Example UPC-A code
        };

        self.col
            .insert_one(test_product)
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;

        println!("Test product inserted successfully.");
        Ok(())
    }

    pub async fn create_transaction(&self, transaction: Transaction) -> Result<ObjectId, Error> {
        let result = self
            .transaction_col
            .insert_one(transaction)
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;

        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| Error::DeserializationError {
                message: "Unable to extract inserted ObjectId".to_string(),
            })
    }

    pub async fn get_all_transactions(&self) -> Result<Vec<Transaction>, Error> {
        let mut cursor =
            self.transaction_col
                .find(doc! {})
                .await
                .map_err(|e| Error::DeserializationError {
                    message: e.to_string(),
                })?;

        let mut transactions: Vec<Transaction> = Vec::new();
        while cursor
            .advance()
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?
        {
            match cursor.deserialize_current() {
                Ok(transaction) => transactions.push(transaction),
                Err(e) => {
                    eprintln!(
                        "Warning: Skipping document due to deserialization error: {}",
                        e
                    );
                    continue;
                }
            }
        }
        Ok(transactions)
    }

    pub async fn get_transaction_by_id(&self, id: ObjectId) -> Result<Option<Transaction>, Error> {
        let mut cursor = self
            .transaction_col
            .find(doc! {"_id": id})
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;

        if cursor
            .advance()
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?
        {
            match cursor.deserialize_current() {
                Ok(transaction) => return Ok(Some(transaction)),
                Err(e) => {
                    eprintln!(
                        "Warning: Skipping document due to deserialization error: {}",
                        e
                    );
                }
            }
        }

        Ok(None)
    }
}
