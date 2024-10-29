use rocket::fs::FileServer;

pub mod encoders;
pub mod models;
pub mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let assets = static_router("static/assets");

    let mut rocket = rocket::build();

    if let Some(assets) = assets {
        rocket = rocket.mount("/assets", assets);
    }

    rocket = rocket.mount(
        "/api",
        rocket::routes![
            routes::api::handle_uuid,
            routes::api::handle_random,
            routes::api::handle_random_ranged
        ],
    );

    rocket = rocket.mount(
        "/",
        rocket::routes![
            routes::web::index,
            routes::web::app_js,
            routes::web::app_css
        ],
    );

    match rocket.launch().await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error {:?}", e);
            Err(e)
        }
    }
}

fn static_router(path: &str) -> Option<FileServer> {
    let root = std::env::current_dir().unwrap();
    let dir = root.join(path);

    if dir.exists() {
        Some(FileServer::from(dir))
    } else {
        None
    }
}
