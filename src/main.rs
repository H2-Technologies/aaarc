#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::http::{Cookie, SameSite};
use rocket::{fs::NamedFile, http::CookieJar, response::Redirect};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct EmailResponse {
    email: String,
    primary: bool,
    verified: bool,
    visibility: Option<String>,
}
struct GitHub;
struct Google;

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

#[get("/navbar.js")]
async fn navbar_js() -> Option<NamedFile> {
    NamedFile::open("static/js/navbar.js").await.ok()
}

#[get("/events")]
async fn events() -> Option<NamedFile> {
    NamedFile::open("static/events.html").await.ok()
}

#[get("/events.css")]
async fn events_css() -> Option<NamedFile> {
    NamedFile::open("static/css/events.css").await.ok()
}

#[get("/auth")]
async fn auth_main() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("static/auth.html").await
}

#[get("/auth.js")]
async fn auth_js() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("static/js/auth.js").await
}

#[get("/auth.css")]
async fn auth_css() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("static/css/auth.css").await
}

#[get("/login/github")]
fn github(oauth2: OAuth2<GitHub>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["user:email"]).unwrap()
}

#[get("/login/google")]
fn google(oauth2: OAuth2<Google>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["email", "profile"]).unwrap()
}

#[get("/github")]
async fn github_callback(token: TokenResponse<GitHub>, cookies: &CookieJar<'_>) -> Redirect {
    println!("Token: {:?}", token.access_token().to_string());
    let res: Result<String, Status> = get_email(&token.access_token().to_string()).await;
    println!("Response: {:?}", res);
    cookies.add_private(
        Cookie::build(("token", token.access_token().to_string()))
            .same_site(SameSite::Lax)
            .build(),
    );
    Redirect::to("/events")
}

#[get("/google")]
async fn google_callback(token: TokenResponse<Google>, cookies: &CookieJar<'_>) -> Redirect {
    cookies.add_private(
        Cookie::build(("token", token.access_token().to_string()))
            .same_site(SameSite::Lax)
            .build(),
    );
    Redirect::to("/events")
}

#[get("/robots.txt")]
async fn robots() -> Option<NamedFile> {
    NamedFile::open("static/robots.txt").await.ok()
}

#[get("/sitemap.xml")]
async fn sitemap() -> Option<NamedFile> {
    NamedFile::open("static/sitemap.xml").await.ok()
}

#[get("/security.txt")]
async fn security() -> Option<NamedFile> {
    NamedFile::open("static/.well-known/security.txt").await.ok()
}

#[get("/pgp-key.txt")]
async fn pgp_key() -> Option<NamedFile> {
    NamedFile::open("static/.well-known/pgp-key.txt").await.ok()
}

#[rocket::main]
async fn main() {
    let allowedEmails: Vec<String> = vec!["admin@austinh.dev".to_string(), "ahadley1124@gmail.com".to_string(), "kd8otq@gmail.com".to_string()];

    let _ = rocket::build()
        .mount("/", routes![index, favicon, events, robots, sitemap, pgp_key])
        .mount("/css/", routes![styles, events_css, auth_css])
        .mount("/js/", routes![main_js, navbar_js, auth_js])
        .mount("/images", routes![covered_bridge])
        .mount("/.well-known", routes![security, pgp_key])
        .mount(
            "/auth",
            routes![
                auth_main,
                github,
                google,
                github_callback,
                google_callback
            ],
        )
        .attach(OAuth2::<GitHub>::fairing("github"))
        .attach(OAuth2::<Google>::fairing("google"))
        .launch()
        .await;
}

async fn get_email(token: &str) -> Result<String, Status> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/user/emails")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "Ashland-Area-Amateur-Radio-Club<admin@austinh.dev>")
        .send()
        .await;
    match response {
        Ok(res) => {
            if res.status().is_success() {
                //println!("Response: {:?}", res.text().await.unwrap());
                return Ok(res.text().await.unwrap());
                //let emails: Vec<EmailResponse> = res.json().await.map_err(|_| Status::InternalServerError)?;
                //if let Some(email) = emails.iter().find(|e| e.primary && e.verified) {
                //    return Ok(email.email.clone());
                //} else {
                //    return Err(Status::InternalServerError);
                //}
            } else {
                println!("Response: {:?}", res.text().await.unwrap());
                return Err(Status::InternalServerError);
            }
        }
        Err(_) => return Err(Status::InternalServerError),
    }
}
