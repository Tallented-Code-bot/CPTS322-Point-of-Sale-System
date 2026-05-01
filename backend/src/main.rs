#[macro_use]
extern crate rocket;

mod api;
mod auth;
mod models;
mod repository;

use rocket::fs::NamedFile;
use rocket::shield::{Hsts, Shield};
use std::path::{Path, PathBuf};

use crate::api::{
    create_transaction, get_all_products, get_all_transactions, get_product_by_upc,
    get_transaction_by_id, push,
    admin_create_product, admin_delete_product, admin_get_products, admin_patch_product,
    admin_get_settings, admin_put_settings,
    admin_create_staff_user, admin_get_staff_users, admin_patch_staff_user,
};
use crate::repository::MongoRepo;

// Serve static files (JS, CSS, images, etc.)
#[get("/<file..>", rank = 5)]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("../frontend/build").join(&file);
    if path.is_dir() {
        return NamedFile::open(path.join("index.html")).await.ok();
    }
    NamedFile::open(path).await.ok()
}

// Fallback to index.html for SPA routing (lowest priority)
#[get("/<_path..>", rank = 20)]
async fn spa_fallback(_path: PathBuf) -> Option<NamedFile> {
    let index_path = Path::new("../frontend/build").join("index.html");
    NamedFile::open(index_path).await.ok()
}

// Serve index.html for root route
#[get("/", rank = 1)]
async fn index() -> Option<NamedFile> {
    let index_path = Path::new("../frontend/build/").join("index.html");
    NamedFile::open(index_path).await.ok()
}

#[launch]
async fn rocket() -> _ {
    let db = MongoRepo::init().await;

    rocket::build()
        .attach(Shield::default().disable::<Hsts>())
        .manage(db)
        // API routes mounted under /api to avoid collisions with static files
        .mount(
            "/api",
            routes![
                get_all_products,
                get_product_by_upc,
                push,
                get_all_transactions,
                get_transaction_by_id,
                create_transaction,
                // Admin API
                admin_get_products,
                admin_create_product,
                admin_patch_product,
                admin_delete_product,
                admin_get_staff_users,
                admin_create_staff_user,
                admin_patch_staff_user,
                admin_get_settings,
                admin_put_settings,
            ],
        )
        // Frontend and static files mounted at root
        .mount("/", routes![index, static_files, spa_fallback])
}
