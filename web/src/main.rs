use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rocket::catch;
use rocket::fs::{relative, NamedFile};
use rocket::response::Redirect;
use rocket::shield::{Hsts, Shield};
use rocket::time::Duration;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Resume {
    pages: Vec<Page>,
    header: Header,
}

#[derive(Serialize, Deserialize)]
struct Page {
    sidebar: Vec<SidebarItem>,
    content: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
struct Header {
    name: String,
    title: String,
    contact: Contact,
    anchor: String,
}

#[derive(Serialize, Deserialize)]
struct Contact {
    website: String,
    email: String,
    phone: String,
    location: String,
}

#[derive(Serialize, Deserialize)]
struct SidebarItem {
    label: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Content {
    label: String,
    items: Vec<ContentItem>,
}

#[derive(Serialize, Deserialize)]
struct ContentItem {
    title: String,
    subtitle: String,
    timeframe: String,
    bullets: Vec<String>,
}

#[rocket::get("/<path..>")]
pub async fn static_pages(path: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(relative!("static")).join(path);
    if path.is_dir() {
        path.push("index.html");
    }
    NamedFile::open(path).await.ok()
}

#[rocket::get("/")]
pub fn page() -> Template {
    let filename = std::env::var("FILE_NAME").expect("Missing environment variable: FILE_NAME");
    let jsondata =
        std::fs::read_to_string(format!("json/{}", filename)).expect("Unable to read JSON file");
    let resume: Resume = serde_json::from_str(&jsondata).expect("Unable to parse JSON file");
    let mut context = HashMap::new();
    context.insert("data", &resume);
    Template::render("template", context)
}

#[rocket::launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    rocket::build()
        .mount("/", rocket::routes![page, static_pages])
        .register("/", rocket::catchers![not_found])
        .attach(Template::fairing())
        .attach(Shield::default().enable(Hsts::IncludeSubDomains(Duration::new(31536000, 0))))
}

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/")
}
