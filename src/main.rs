#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::http::{Cookie, SameSite};
use rocket::response::content::RawJson;
use rocket::response::Responder;
use rocket::Request;
use rocket::{fs::NamedFile, http::CookieJar, response::Redirect};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::read_dir;
use std::io::Write; // Import the serde_json crate

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

#[get("/resources.js")]
async fn resources_js() -> Option<NamedFile> {
    NamedFile::open("static/js/resources.js").await.ok()
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
    NamedFile::open("static/.well-known/security.txt")
        .await
        .ok()
}

#[get("/pgp-key.txt")]
async fn pgp_key() -> Option<NamedFile> {
    NamedFile::open("static/.well-known/pgp-key.txt").await.ok()
}

#[get("/repeaters")]
async fn repeaters() -> Option<NamedFile> {
    NamedFile::open("static/repeaters.html").await.ok()
}

#[get("/repeaters.css")]
async fn repeaters_css() -> Option<NamedFile> {
    NamedFile::open("static/css/repeaters.css").await.ok()
}

#[get("/contact")]
async fn contact() -> Option<NamedFile> {
    NamedFile::open("static/contact.html").await.ok()
}

#[get("/resources")]
async fn resources() -> Option<NamedFile> {
    NamedFile::open("static/resources.html").await.ok()
}

#[get("/ads.txt")]
async fn ads() -> Option<NamedFile> {
    NamedFile::open("static/ads.txt").await.ok()
}

#[get("/privacy-policy")]
async fn privacy_policy() -> Option<NamedFile> {
    NamedFile::open("static/privacy-policy.html").await.ok()
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(Cookie::from("token"));
    Redirect::to("/")
}

#[get("/")]
async fn ping() -> String {
    "pong".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
struct FileInfo {
    name: String,
    size: u64,
    modified: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileList {
    files: Vec<FileInfo>,
}

#[get("/files/list")]
async fn list_files() -> RawJson<String> {
    let mut files: Vec<FileInfo> = Vec::new();
    let dir = read_dir("./static/content/").unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        let modified = metadata.modified().unwrap();
        let modified = modified
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();
        let modified = chrono::NaiveDateTime::from_timestamp(modified.as_secs() as i64, 0);
        let modified = modified.format("%Y-%m-%d %H:%M:%S").to_string();
        files.push(FileInfo {
            name: entry.file_name().to_string_lossy().to_string(),
            size: metadata.len(),
            modified: modified,
        });
    }
    //let file_list = FileList { files: files };
    let json = serde_json::to_string(&files).unwrap();
    println!("JSON: {:?}", json);
    RawJson(json)
}

#[catch(default)]
async fn errer(status: Status, request: &Request<'_>) -> &'static str {
    println!("Error: {:?}", status);
    log_file_rotate(request);
    "An error occurred"
}

#[rocket::main]
async fn main() {
    let allowed_emails: Vec<&str> = vec![
        "ahadley1124@gmail.com",
        "admin@austinh.dev",
        "kd8otq@gmail.com",
    ];
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                index,
                favicon,
                events,
                robots,
                sitemap,
                pgp_key,
                repeaters,
                contact,
                resources,
                ads,
                privacy_policy,
                logout
            ],
        )
        .mount(
            "/css/",
            routes![styles, events_css, auth_css, repeaters_css],
        )
        .mount("/js/", routes![main_js, navbar_js, auth_js, resources_js])
        .mount("/images/", routes![covered_bridge])
        .mount("/.well-known/", routes![security, pgp_key])
        .mount(
            "/auth/",
            routes![auth_main, github, google, github_callback, google_callback],
        )
        .mount("/api/", routes![ping, list_files])
        .attach(OAuth2::<GitHub>::fairing("github"))
        .attach(OAuth2::<Google>::fairing("google"))
        .manage(allowed_emails)
        .launch()
        .await;
}

async fn get_email(token: &str) -> Result<String, Status> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/user/emails")
        .header("Authorization", format!("Bearer {}", token))
        .header(
            "User-Agent",
            "Ashland-Area-Amateur-Radio-Club<admin@austinh.dev>",
        )
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

fn log_file_rotate(req: &rocket::Request) {
    //get the size of the file
    let file = std::fs::File::open("/var/log/rocket.log").unwrap();
    let metadata = file.metadata().unwrap();
    let size = metadata.len();
    //if the file is larger than 10MB, rotate it
    if size > 10_000_000 {
        let new_name = format!(
            "/var/log/rocket.log.{}",
            chrono::Local::now().format("%Y-%m-%d")
        );
        std::fs::rename("/var/log/rocket.log", new_name).unwrap();
    }
    // format a string with the date, method, uri, and status code
    let log = format!(
        "{} - {} {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        req.method(),
        req.uri(),
    );
    // open the file in append mode and write the log
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("/var/log/rocket.log")
        .unwrap();
    file.write_all(log.as_bytes()).unwrap();
}
