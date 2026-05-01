use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::auth::AdminUser;
use crate::repository::MongoRepo;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsResponse {
    tax_rate: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutSettingsRequest {
    tax_rate: f32,
}

#[get("/admin/settings")]
pub async fn admin_get_settings(
    db: &State<MongoRepo>,
    _admin: AdminUser,
) -> Result<Json<SettingsResponse>, Status> {
    let settings = db
        .get_settings_singleton()
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(SettingsResponse {
        tax_rate: settings.tax_rate,
    }))
}

#[put("/admin/settings", data = "<payload>")]
pub async fn admin_put_settings(
    db: &State<MongoRepo>,
    _admin: AdminUser,
    payload: Json<PutSettingsRequest>,
) -> Result<Status, Status> {
    let data = payload.into_inner();
    if !(0.0..=1.0).contains(&data.tax_rate) {
        return Err(Status::BadRequest);
    }
    db.set_tax_rate(data.tax_rate)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Status::NoContent)
}
