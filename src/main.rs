use rocket::catch;
use rocket::fs::{relative, NamedFile};
use rocket::response::{content, Redirect};
use rocket::serde::json::Json;
use rocket::shield::{Hsts, Shield};
use rocket::time::Duration;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct Resume {
	accent_color: Option<String>,
	font_size: Option<String>,
	page_title: String,
	pages: Vec<Page>,
	header: Header,
	pdf: Option<String>,
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
	contact: Vec<ContactItem>,
	anchor: String,
}

#[derive(Serialize, Deserialize)]
struct ContactItem {
	label: String,
	value: Option<String>,
	env_var: Option<String>,
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
	timeframe_subtitle: Option<String>,
	bullets: Vec<String>,
	footnote: Option<String>,
}

#[rocket::get("/<path..>")]
pub async fn static_pages(path: PathBuf) -> Option<NamedFile> {
	let mut path = Path::new(relative!("static")).join(path);
	if path.is_dir() {
		path.push("index.html");
	}
	NamedFile::open(path).await.ok()
}

#[rocket::get("/?<file>")]
pub fn page(file: Option<String>) -> Template {
	let default_filename =
		std::env::var("FILE_NAME").expect("Missing environment variable: FILE_NAME");

	let filename = file
		.filter(|s| s.chars().all(char::is_alphanumeric))
		.map(|s| format!("{}.json", s))
		.unwrap_or(default_filename.clone());

	let jsondata = match std::fs::read_to_string(format!("static/data/{}", filename)) {
		Ok(data) => data,
		Err(_) => std::fs::read_to_string(format!("static/data/{}", default_filename))
			.expect("Unable to read JSON file"),
	};

	let mut resume: Resume = serde_json::from_str(&jsondata).expect("Unable to parse JSON file");

	for contact_item in &mut resume.header.contact {
		if let Some(env_var) = &contact_item.env_var {
			let value = std::env::var(env_var)
				.unwrap_or_else(|_| panic!("Missing environment variable: {}", env_var));

			let value_base64 =
				base64::Engine::encode(&base64::engine::general_purpose::STANDARD_NO_PAD, value);

			let value_base64_rev = value_base64.chars().rev().collect();
			contact_item.value = Some(value_base64_rev);
		}
	}

	let mut context = HashMap::new();
	context.insert("data", &resume);
	Template::render("template", context)
}

#[rocket::launch]
fn rocket() -> _ {
	dotenvy::dotenv().ok();
	rocket::build()
		.mount(
			"/",
			rocket::routes![page, static_pages, accent, version, fontsize],
		)
		.register("/", rocket::catchers![not_found])
		.attach(Template::fairing())
		.attach(Shield::default().enable(Hsts::IncludeSubDomains(Duration::new(31536000, 0))))
}

#[catch(404)]
pub fn not_found() -> Redirect {
	Redirect::to("/")
}

#[derive(Serialize)]
pub struct VersionInfo {
	version: String,
}

#[rocket::get("/version")]
pub fn version() -> Json<VersionInfo> {
	Json(VersionInfo {
		version: env!("CARGO_PKG_VERSION").to_string(),
	})
}

#[rocket::get("/accent/<filename>")]
fn accent(filename: String) -> Option<content::RawCss<String>> {
	if filename.len() == 10 && filename.ends_with(".css") {
		let hexcode = &filename[0..6];

		if hexcode.chars().all(|c| c.is_ascii_hexdigit()) {
			return Some(content::RawCss(format!(
				":root {{ --accent-color: #{}; }}",
				hexcode
			)));
		}
	}

	None
}

#[rocket::get("/fontsize/<filename>")]
fn fontsize(filename: String) -> Option<content::RawCss<String>> {
	if filename.ends_with(".css") {
		return Some(content::RawCss(format!(
			":root {{ --font-size: {}; }}",
			&filename[0..filename.len() - 4]
		)));
	}

	None
}
