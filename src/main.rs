use rocket::fs::NamedFile;
#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[get("/styles.css")]
async fn styles() -> Option<NamedFile> {
    NamedFile::open("static/css/styles.css").await.ok()
}

#[get("/main.js")]
async fn main_js() -> Option<NamedFile> {
    NamedFile::open("static/js/main.js").await.ok()
}

#[get("/favicon.ico")]
fn favicon() -> rocket::http::Status {
    rocket::http::Status::NoContent
}

#[get("/coveredBridge.jpg")]
async fn covered_bridge() -> Option<NamedFile> {
    NamedFile::open("static/images/coveredBridge.jpg")
        .await
        .ok()
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![index, favicon])
        .mount("/css/", routes![styles])
        .mount("/js/", routes![main_js])
        .mount("/images", routes![covered_bridge])
        .launch()
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_index() {
        let rocket = rocket::build();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_styles() {
        let rocket = rocket::build();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get(uri!(super::styles)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_main_js() {
        let rocket = rocket::build();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get(uri!(super::main_js)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_favicon() {
        let rocket = rocket::build();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get(uri!(super::favicon)).dispatch();
        assert_eq!(response.status(), Status::NoContent);
    }

    #[test]
    fn test_covered_bridge() {
        let rocket = rocket::build();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get(uri!(super::covered_bridge)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_404() {
        let rocket = rocket::build();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/not_a_real_page").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
