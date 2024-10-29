pub mod web {
    use rocket::get;
    use rocket::http::{Cookie, CookieJar};
    use rocket::response::content::RawHtml;
    use rocket::tokio::fs::File;

    use crate::models::gen_uuid;

    #[get("/")]
    pub async fn index(cookies: &CookieJar<'_>) -> RawHtml<Vec<u8>> {
        let _c = cookies.get("uuid");

        match _c {
            Some(_) => (),
            None => {
                cookies.add(Cookie::build(("uuid", gen_uuid().to_string())));
            }
        }

        let html = rocket::tokio::fs::read("static/index.html")
            .await
            .expect("Failed to read file");
        RawHtml(html)
    }

    #[cfg(not(debug_assertions))]
    #[get("/app.js")]
    pub async fn app_js() -> GzipResponder {
        let mut file = File::open("static/app.js.gz").await.unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.unwrap();
        GzipResponder { body: buf }
    }

    #[cfg(debug_assertions)]
    #[get("/app.js")]
    pub async fn app_js() -> File {
        File::open("static/app.js").await.unwrap()
    }

    #[cfg(not(debug_assertions))]
    #[get("/app.css")]
    pub async fn app_css() -> GzipResponder {
        let mut file = File::open("static/app.css.gz").await.unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.unwrap();
        GzipResponder { body: buf }
    }

    #[cfg(debug_assertions)]
    #[get("/app.css")]
    pub async fn app_css() -> File {
        File::open("static/app.css").await.unwrap()
    }
}

pub mod api {
    use rocket::{get, post, serde::json::Json};

    use crate::models::{gen_uuid, random, RandomRange, RAND_MAX};

    #[get("/uuid")]
    pub fn handle_uuid() -> String {
        let uuid = gen_uuid();
        format!("{:?}", uuid)
    }

    #[post("/random/<count>", format = "json", data = "<range>")]
    pub fn handle_random_ranged(count: usize, range: Json<RandomRange>) -> String {
        let v = random(count, range.min, range.max);
        format!("{:?}", v)
    }

    #[get("/random/<count>")]
    pub fn handle_random(count: usize) -> String {
        let v = random(count, 0, RAND_MAX);
        format!("{:?}", v)
    }
}
