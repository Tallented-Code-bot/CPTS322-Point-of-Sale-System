use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    Client, Collection,
};
use std::{env, str::FromStr};
use upc_a::UpcA;

use chrono::Utc;
use crate::models::{Product, Settings, StaffRole, StaffStatus, StaffUser, Transaction, UPC};

pub struct MongoRepo {
    col: Collection<Product>,
    transaction_col: Collection<Transaction>,
    staff_col: Collection<StaffUser>,
    settings_col: Collection<Settings>,
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
        let staff_col: Collection<StaffUser> = db.collection("staff_users");
        let settings_col: Collection<Settings> = db.collection("settings");
        MongoRepo {
            col,
            transaction_col,
            staff_col,
            settings_col,
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
            quantity: Some(10),
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

    // --- Staff users (admin) ---
    pub async fn get_staff_user_by_email(&self, email: &str) -> Result<Option<StaffUser>, Error> {
        self.staff_col
            .find_one(doc! {"email": email.to_lowercase()})
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })
    }

    pub async fn list_staff_users(&self) -> Result<Vec<StaffUser>, Error> {
        let mut cursor = self
            .staff_col
            .find(doc! {})
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;

        let mut users: Vec<StaffUser> = Vec::new();
        while cursor
            .advance()
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?
        {
            match cursor.deserialize_current() {
                Ok(u) => users.push(u),
                Err(e) => {
                    eprintln!(
                        "Warning: Skipping document due to deserialization error: {}",
                        e
                    );
                    continue;
                }
            }
        }
        Ok(users)
    }

    pub async fn create_staff_user(
        &self,
        email: &str,
        role: StaffRole,
    ) -> Result<ObjectId, Error> {
        let now = Utc::now();
        let doc = StaffUser {
            id: None,
            email: email.to_lowercase(),
            role,
            status: StaffStatus::Invited,
            created_at: now,
            updated_at: now,
        };

        // Enforce uniqueness by checking first.
        if self
            .get_staff_user_by_email(&doc.email)
            .await?
            .is_some()
        {
            return Err(Error::DeserializationError {
                message: "staff user already exists".to_string(),
            });
        }

        let result = self
            .staff_col
            .insert_one(doc)
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

    pub async fn bootstrap_admin_if_missing(&self, email: &str) -> Result<StaffUser, Error> {
        let email = email.to_lowercase();
        if let Some(existing) = self.get_staff_user_by_email(&email).await? {
            return Ok(existing);
        }

        let now = Utc::now();
        let admin = StaffUser {
            id: None,
            email: email.clone(),
            role: StaffRole::Admin,
            status: StaffStatus::Active,
            created_at: now,
            updated_at: now,
        };

        let _ = self
            .staff_col
            .insert_one(admin)
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;

        self.get_staff_user_by_email(&email)
            .await?
            .ok_or_else(|| Error::DeserializationError {
                message: "bootstrap admin insert failed".to_string(),
            })
    }

    pub async fn update_staff_user(
        &self,
        id: ObjectId,
        role: Option<StaffRole>,
        status: Option<StaffStatus>,
    ) -> Result<bool, Error> {
        let mut set_doc = mongodb::bson::Document::new();
        if let Some(r) = role {
            let b = mongodb::bson::to_bson(&r).map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;
            set_doc.insert("role", b);
        }
        if let Some(s) = status {
            let b = mongodb::bson::to_bson(&s).map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;
            set_doc.insert("status", b);
        }
        set_doc.insert(
            "updatedAt",
            mongodb::bson::DateTime::from_millis(Utc::now().timestamp_millis()),
        );

        let result = self.staff_col
            .update_one(doc! {"_id": id}, doc! {"$set": set_doc})
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;
        Ok(result.matched_count > 0)
    }

    // --- Settings (admin) ---
    pub async fn get_settings_singleton(&self) -> Result<Settings, Error> {
        let existing = self
            .settings_col
            .find_one(doc! {"key": "singleton"})
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;
        if let Some(s) = existing {
            return Ok(s);
        }

        let now = Utc::now();
        let settings = Settings::singleton_default(now);
        let _ = self
            .settings_col
            .insert_one(settings.clone())
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;
        Ok(settings)
    }

    pub async fn set_tax_rate(&self, tax_rate: f32) -> Result<(), Error> {
        let now = Utc::now();
        let now_bson = mongodb::bson::DateTime::from_millis(now.timestamp_millis());
        self.settings_col
            .update_one(
                doc! {"key": "singleton"},
                doc! {"$set": {"taxRate": tax_rate, "updatedAt": now_bson}, "$setOnInsert": {"key": "singleton", "createdAt": now_bson}},
            )
            .upsert(true)
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;
        Ok(())
    }

    // --- Products (admin) ---
    pub async fn create_product(&self, product: Product) -> Result<ObjectId, Error> {
        let result = self
            .col
            .insert_one(product)
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

    pub async fn update_product(
        &self,
        id: ObjectId,
        set_doc: mongodb::bson::Document,
    ) -> Result<bool, Error> {
        let result = self.col
            .update_one(doc! {"_id": id}, doc! {"$set": set_doc})
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;
        Ok(result.matched_count > 0)
    }

    pub async fn delete_product(&self, id: ObjectId) -> Result<bool, Error> {
        let result = self.col
            .delete_one(doc! {"_id": id})
            .await
            .map_err(|e| Error::DeserializationError {
                message: e.to_string(),
            })?;
        Ok(result.deleted_count > 0)
    }
}
