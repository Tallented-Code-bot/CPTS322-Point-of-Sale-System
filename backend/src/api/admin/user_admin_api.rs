use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::auth::AdminUser;
use crate::models::{StaffRole, StaffStatus, StaffUser};
use crate::repository::MongoRepo;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffUserRow {
    id: String,
    email: String,
    role: StaffRole,
    status: StaffStatus,
}

impl TryFrom<StaffUser> for StaffUserRow {
    type Error = Status;

    fn try_from(value: StaffUser) -> Result<Self, Self::Error> {
        let id = value.id.ok_or(Status::InternalServerError)?.to_hex();
        Ok(Self {
            id,
            email: value.email,
            role: value.role,
            status: value.status,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStaffUserRequest {
    email: String,
    role: StaffRole,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchStaffUserRequest {
    role: Option<StaffRole>,
    status: Option<StaffStatus>,
}

#[get("/admin/users?<query>")]
pub async fn admin_get_staff_users(
    db: &State<MongoRepo>,
    _admin: AdminUser,
    query: Option<String>,
) -> Result<Json<Vec<StaffUserRow>>, Status> {
    let mut users = db
        .list_staff_users()
        .await
        .map_err(|_| Status::InternalServerError)?;

    if let Some(q) = query {
        let q = q.trim().to_lowercase();
        if !q.is_empty() {
            users = users
                .into_iter()
                .filter(|u| {
                    u.email.to_lowercase().contains(&q)
                        || format!("{:?}", u.role).to_lowercase().contains(&q)
                        || format!("{:?}", u.status).to_lowercase().contains(&q)
                })
                .collect();
        }
    }

    let rows = users
        .into_iter()
        .map(StaffUserRow::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Json(rows))
}

#[post("/admin/users", data = "<payload>")]
pub async fn admin_create_staff_user(
    db: &State<MongoRepo>,
    _admin: AdminUser,
    payload: Json<CreateStaffUserRequest>,
) -> Result<Json<StaffUserRow>, Status> {
    let data = payload.into_inner();
    let email = data.email.trim().to_lowercase();
    if email.is_empty() || !email.contains('@') {
        return Err(Status::BadRequest);
    }

    let id = db.create_staff_user(&email, data.role).await.map_err(|e| {
        // Repo uses a generic error type; inspect the message for duplicates.
        let msg = e.to_string();
        if msg.contains("staff user already exists") {
            Status::Conflict
        } else {
            Status::InternalServerError
        }
    })?;

    let created = db
        .get_staff_user_by_email(&email)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::InternalServerError)?;

    Ok(Json(StaffUserRow {
        id: id.to_hex(),
        email: created.email,
        role: created.role,
        status: created.status,
    }))
}

#[patch("/admin/users/<id>", data = "<payload>")]
pub async fn admin_patch_staff_user(
    db: &State<MongoRepo>,
    _admin: AdminUser,
    id: String,
    payload: Json<PatchStaffUserRequest>,
) -> Result<Status, Status> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| Status::BadRequest)?;
    let data = payload.into_inner();
    if data.role.is_none() && data.status.is_none() {
        return Err(Status::BadRequest);
    }

    let updated = db
        .update_staff_user(object_id, data.role, data.status)
        .await
        .map_err(|_| Status::InternalServerError)?;
    if !updated {
        return Err(Status::NotFound);
    }
    Ok(Status::NoContent)
}
