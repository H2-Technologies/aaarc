use rocket::fs::NamedFile;
#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![index]).launch().await;
}
