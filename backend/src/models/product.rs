use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct UPC(pub UpcA);

impl<'a> FromParam<'a> for UPC {
    type Error = UpcAParseError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(Self(UpcA::from_str(param)?))
    }
}
