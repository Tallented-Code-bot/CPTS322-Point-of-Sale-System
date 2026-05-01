use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use rocket::request::FromParam;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use upc_a::{UpcA, UpcAParseError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub price: Option<f32>,
    pub upc: Option<UPC>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UPC(pub UpcA);

impl Serialize for UPC {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for UPC {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpcVisitor;

        impl<'de> de::Visitor<'de> for UpcVisitor {
            type Value = UPC;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a UPC-A string (12 digits) or an integer")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let parsed = UpcA::from_str(v).map_err(E::custom)?;
                Ok(UPC(parsed))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(&v)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(E::custom("UPC cannot be negative"));
                }
                self.visit_u64(v as u64)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                // Older DB entries may have been stored as an int64, which drops leading zeros.
                // Pad back to 12 digits.
                let s = format!("{v:012}");
                let parsed = UpcA::from_str(&s).map_err(E::custom)?;
                Ok(UPC(parsed))
            }
        }

        deserializer.deserialize_any(UpcVisitor)
    }
}

impl<'a> FromParam<'a> for UPC {
    type Error = UpcAParseError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(Self(UpcA::from_str(param)?))
    }
}
