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
	page_title: String,
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

			let value_base64 = base64::encode(value);
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
		.mount("/", rocket::routes![page, static_pages])
		.register("/", rocket::catchers![not_found])
		.attach(Template::fairing())
		.attach(Shield::default().enable(Hsts::IncludeSubDomains(Duration::new(31536000, 0))))
}

#[catch(404)]
pub fn not_found() -> Redirect {
	Redirect::to("/")
}
