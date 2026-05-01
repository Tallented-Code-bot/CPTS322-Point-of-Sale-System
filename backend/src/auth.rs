use std::env;
use std::time::{Duration, Instant};

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use once_cell::sync::{Lazy, OnceCell};
use reqwest::Client;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::Deserialize;

use crate::models::{StaffRole, StaffStatus};
use crate::repository::MongoRepo;

#[derive(Debug, Clone)]
pub struct AdminUser {
    pub email: String,
}

#[derive(Debug, Deserialize)]
struct Auth0Claims {
    exp: usize,
    iss: String,
    aud: serde_json::Value,
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Clone, Deserialize)]
struct Jwk {
    kid: String,
    kty: String,
    n: String,
    e: String,
    alg: Option<String>,
    use_: Option<String>,
}

struct JwksCache {
    fetched_at: Instant,
    jwks: Jwks,
}

static JWKS_CACHE: OnceCell<tokio::sync::RwLock<Option<JwksCache>>> = OnceCell::new();

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(8))
        .build()
        .expect("reqwest client build")
});

fn cache() -> &'static tokio::sync::RwLock<Option<JwksCache>> {
    JWKS_CACHE.get_or_init(|| tokio::sync::RwLock::new(None))
}

fn env_required(key: &str) -> Result<String, Status> {
    env::var(key).map_err(|_| Status::InternalServerError)
}

fn issuer_from_domain(domain: &str) -> String {
    format!("https://{}/", domain.trim())
}

async fn fetch_jwks(domain: &str) -> Result<Jwks, Status> {
    let url = format!("https://{}/.well-known/jwks.json", domain.trim());
    let res = HTTP_CLIENT
        .get(url)
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?;
    if !res.status().is_success() {
        return Err(Status::InternalServerError);
    }
    res.json::<Jwks>()
        .await
        .map_err(|_| Status::InternalServerError)
}

async fn fetch_userinfo_email(domain: &str, token: &str) -> Result<Option<String>, Status> {
    let url = format!("https://{}/userinfo", domain.trim());
    let res = HTTP_CLIENT
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?;
    if res.status() == reqwest::StatusCode::UNAUTHORIZED {
        return Err(Status::Unauthorized);
    }
    if !res.status().is_success() {
        return Err(Status::InternalServerError);
    }
    let info = res
        .json::<UserInfo>()
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(info.email)
}

async fn get_jwks(domain: &str) -> Result<Jwks, Status> {
    let ttl = Duration::from_secs(60 * 10);
    {
        let read = cache().read().await;
        if let Some(cached) = read.as_ref() {
            if cached.fetched_at.elapsed() < ttl {
                return Ok(Jwks {
                    keys: cached.jwks.keys.clone(),
                });
            }
        }
    }

    let jwks = fetch_jwks(domain).await?;
    let mut write = cache().write().await;
    *write = Some(JwksCache {
        fetched_at: Instant::now(),
        jwks: Jwks {
            keys: jwks.keys.clone(),
        },
    });
    Ok(jwks)
}

fn aud_matches(aud: &serde_json::Value, expected: &str) -> bool {
    match aud {
        serde_json::Value::String(s) => s == expected,
        serde_json::Value::Array(arr) => arr.iter().any(|v| v.as_str() == Some(expected)),
        _ => false,
    }
}

async fn decode_auth0_jwt(token: &str) -> Result<TokenData<Auth0Claims>, Status> {
    let domain = env_required("AUTH0_DOMAIN")?;
    let audience = env_required("AUTH0_AUDIENCE")?;

    let header = decode_header(token).map_err(|_| Status::Unauthorized)?;
    let kid = header.kid.ok_or(Status::Unauthorized)?;

    let jwks = get_jwks(&domain).await?;
    let jwk = jwks
        .keys
        .iter()
        .find(|k| k.kid == kid && k.kty == "RSA")
        .ok_or(Status::Unauthorized)?;

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|_| Status::Unauthorized)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[issuer_from_domain(&domain)]);
    // We'll verify audience manually because Auth0 may provide aud as array.
    validation.validate_aud = false;

    let data = decode::<Auth0Claims>(token, &decoding_key, &validation)
        .map_err(|_| Status::Unauthorized)?;

    if !aud_matches(&data.claims.aud, &audience) {
        return Err(Status::Unauthorized);
    }

    Ok(data)
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth = match req.headers().get_one("Authorization") {
            Some(v) => v,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let token = auth
            .strip_prefix("Bearer ")
            .or_else(|| auth.strip_prefix("bearer "));
        let Some(token) = token else {
            return Outcome::Error((Status::Unauthorized, ()));
        };

    let token_data = match decode_auth0_jwt(token).await {
        Ok(d) => d,
        Err(s) => return Outcome::Error((s, ())),
    };

        let domain = match std::env::var("AUTH0_DOMAIN") {
            Ok(d) => d,
            Err(_) => return Outcome::Error((Status::InternalServerError, ())),
        };

        let email = match token_data.claims.email {
            Some(e) if !e.trim().is_empty() => e,
            _ => match fetch_userinfo_email(&domain, token).await {
                Ok(Some(e)) if !e.trim().is_empty() => e,
                Ok(_) => return Outcome::Error((Status::Unauthorized, ())),
                Err(s) => return Outcome::Error((s, ())),
            },
        };

        // Enforce role via our Mongo staff record.
        let db = match req.guard::<&rocket::State<MongoRepo>>().await {
            Outcome::Success(db) => db,
            _ => return Outcome::Error((Status::InternalServerError, ())),
        };

        let staff = match db.get_staff_user_by_email(&email).await {
            Ok(Some(u)) => u,
            Ok(None) => {
                // Bootstrap path: allow one configured email to become the first admin.
                if let Ok(bootstrap) = std::env::var("BOOTSTRAP_ADMIN_EMAIL") {
                    if bootstrap.trim().eq_ignore_ascii_case(&email) {
                        match db.bootstrap_admin_if_missing(&email).await {
                            Ok(u) => u,
                            Err(_) => return Outcome::Error((Status::InternalServerError, ())),
                        }
                    } else {
                        return Outcome::Error((Status::Forbidden, ()));
                    }
                } else {
                    return Outcome::Error((Status::Forbidden, ()));
                }
            }
            Err(_) => return Outcome::Error((Status::InternalServerError, ())),
        };

        if staff.status != StaffStatus::Active {
            return Outcome::Error((Status::Forbidden, ()));
        }
        if staff.role != StaffRole::Admin {
            return Outcome::Error((Status::Forbidden, ()));
        }

        Outcome::Success(AdminUser { email })
    }
}
