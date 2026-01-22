#[macro_use]
extern crate rocket;

use rocket::fs::{relative, NamedFile};
use rocket::http::Status;
use std::path::{Path, PathBuf};

// Serve static files (JS, CSS, images, etc.)
#[get("/<file..>", rank = 2)]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("../frontend/build")).join(&file);

    // Only serve if the file exists and is not a directory
    if path.is_file() {
        NamedFile::open(path).await.ok()
    } else {
        None
    }
}

// Fallback to index.html for SPA routing (lowest priority)
#[get("/<_path..>", rank = 10)]
async fn spa_fallback(_path: PathBuf) -> Result<NamedFile, Status> {
    let index_path = Path::new(relative!("../frontend/build")).join("index.html");
    NamedFile::open(index_path)
        .await
        .map_err(|_| Status::NotFound)
}

// Serve index.html for root route
#[get("/", rank = 1)]
async fn index() -> Result<NamedFile, Status> {
    let index_path = Path::new(relative!("../frontend/build")).join("index.html");
    NamedFile::open(index_path)
        .await
        .map_err(|_| Status::NotFound)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // Mount API routes here with rank 1 (they get priority)
        // .mount("/api", routes![health_check])
        // Mount static file serving and SPA fallback
        .mount("/", routes![index, static_files, spa_fallback])
}
