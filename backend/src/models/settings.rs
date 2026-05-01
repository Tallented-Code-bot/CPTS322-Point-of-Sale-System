use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub key: String,
    pub tax_rate: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Settings {
    pub fn singleton_default(now: DateTime<Utc>) -> Self {
        Self {
            id: None,
            key: "singleton".to_string(),
            tax_rate: 0.0825,
            created_at: now,
            updated_at: now,
        }
    }
}
