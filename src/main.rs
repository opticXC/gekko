use encoders::GzipResponder;
use rand::{self, Rng};
use rocket::http::Header;
use rocket::http::{Cookie, CookieJar};
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncReadExt;
use rocket::{fs::FileServer, get, post, routes};
use rocket::{Request, Response};
use serde::Deserialize;

pub mod encoders;

const RAND_MAX: i32 = 1000;

fn random(count: usize, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 0..count {
        vec.push(rng.gen_range(min..max));
    }
    vec
}

#[get("/random/<count>")]
fn handle_random(count: usize) -> String {
    let v = random(count, 0, RAND_MAX);
    format!("{:?}", v)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RandomRange {
    min: i32,
    max: i32,
}

#[post("/random/<count>", format = "json", data = "<range>")]
fn handle_random_ranged(count: usize, range: Json<RandomRange>) -> String {
    let v = random(count, range.min, range.max);
    format!("{:?}", v)
}

fn gen_uuid() -> uuid::Uuid {
    let r = rand::thread_rng().gen_range(1..u128::MAX);

    let uuid = uuid::Uuid::from_u128(r);
    return uuid;
}

#[get("/uuid")]
fn handle_uuid() -> String {
    let uuid = gen_uuid();
    format!("{:?}", uuid)
}

#[get("/")]
async fn index(cookies: &CookieJar<'_>) -> RawHtml<Vec<u8>> {
    let _c = cookies.get("uuid");
    match _c {
        Some(c) => (),
        None => {
            cookies.add(Cookie::build(("uuid", gen_uuid().to_string())));
        }
    }
    let html = rocket::tokio::fs::read("static/index.html")
        .await
        .expect("Failed to read file");
    RawHtml(html)
}

#[get("/app.js")]
async fn app_js_gzip() -> GzipResponder {
    let mut file = File::open("static/app.js.gz").await.unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();
    GzipResponder { body: buf }
}

#[get("/app.css")]
async fn app_css_gzip() -> GzipResponder {
    let mut file = File::open("static/app.css.gz").await.unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();
    GzipResponder { body: buf }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let root = std::env::current_dir().unwrap();
    let assets = root.join("static/assets");
    let rocket = rocket::build()
        .mount(
            "/api",
            routes![handle_uuid, handle_random, handle_random_ranged],
        )
        .mount("/assets", FileServer::from(assets).rank(-1))
        .mount("/", routes![index, app_js_gzip, app_css_gzip]);

    match rocket.launch().await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error {:?}", e);
            Err(e)
        }
    }
}
